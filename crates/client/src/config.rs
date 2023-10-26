extern crate lazy_static;

use anyhow::Context;
use fehler::throw;
use serde::Deserialize;
use std::{env, fs, io, path::PathBuf};
use thiserror::Error;

pub const CARGO_TOML: &str = "Cargo.toml";
pub const TRDELNIK_TOML: &str = "Trdelnik.toml";
pub const ANCHOR_TOML: &str = "Anchor.toml";

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid workspace")]
    BadWorkspace,
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Test {
    pub validator_startup_timeout: u64,
}
#[derive(Default, Debug, Deserialize, Clone)]
struct _Test {
    #[serde(default)]
    pub validator_startup_timeout: Option<u64>,
}
impl Default for Test {
    fn default() -> Self {
        Self {
            validator_startup_timeout: 10_000,
        }
    }
}
impl From<_Test> for Test {
    fn from(_t: _Test) -> Self {
        Self {
            validator_startup_timeout: _t.validator_startup_timeout.unwrap_or(10_000),
        }
    }
}
#[derive(Debug, Deserialize, Clone)]
pub struct FuzzArg {
    pub short_opt: Option<String>,
    pub long_opt: Option<String>,
    pub val: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Fuzz {
    pub fuzz_args: Vec<FuzzArg>,
}
#[derive(Default, Debug, Deserialize, Clone)]
struct _Fuzz {
    #[serde(default)]
    /// Timeout in seconds (default: 10)
    /// -t
    pub timeout: Option<u16>,
    #[serde(default)]
    /// Number of fuzzing iterations (default: 0 [no limit])
    /// -N
    pub iterations: Option<u64>,
    #[serde(default)]
    /// Don't close children's stdin, stdout, stderr; can be noisy
    /// -Q
    pub keep_output: Option<bool>,
    #[serde(default)]
    /// Disable ANSI console; use simple log output
    /// -v
    pub verbose: Option<bool>,
    #[serde(default)]
    /// Exit upon seeing the first crash (default: false)
    /// --exit_upon_crash
    pub exit_upon_crash: Option<bool>,
}
impl Default for Fuzz {
    fn default() -> Self {
        Self {
            fuzz_args: vec![
                FuzzArg::new("-t", "--timeout", &10.to_string()),
                FuzzArg::new("-N", "--iterations", &0.to_string()),
            ],
        }
    }
}
impl From<_Fuzz> for Fuzz {
    fn from(_f: _Fuzz) -> Self {
        let mut _self = Self { fuzz_args: vec![] };

        // timeout
        let timeout = _f.timeout.unwrap_or(10);
        _self
            .fuzz_args
            .push(FuzzArg::new("-t", "--timeout", &timeout.to_string()));

        // iterations
        let iterations = _f.iterations.unwrap_or(0);
        _self
            .fuzz_args
            .push(FuzzArg::new("-N", "--iterations", &iterations.to_string()));

        // keep_output
        let keep_output = _f.keep_output.unwrap_or(false);
        if keep_output {
            _self
                .fuzz_args
                .push(FuzzArg::new("-Q", "--keep_output", ""));
        }

        // verbose
        let verbose = _f.verbose.unwrap_or(false);
        if verbose {
            _self.fuzz_args.push(FuzzArg::new("-v", "--verbose", ""));
        }

        // exit_upon_crash
        let exit_upon_crash = _f.exit_upon_crash.unwrap_or(false);
        if exit_upon_crash {
            _self
                .fuzz_args
                .push(FuzzArg::new("", "--exit_upon_crash", ""));
        }
        _self
    }
}

impl FuzzArg {
    fn new(short_opt: &str, long_opt: &str, val: &str) -> Self {
        let short_opt = if short_opt.is_empty() {
            None
        } else {
            Some(short_opt.to_owned())
        };
        let long_opt = if long_opt.is_empty() {
            None
        } else {
            Some(long_opt.to_owned())
        };
        let val = if val.is_empty() {
            None
        } else {
            Some(val.to_owned())
        };
        Self {
            short_opt,
            long_opt,
            val,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub test: Test,
    pub fuzz: Fuzz,
}

#[derive(Default, Debug, Deserialize, Clone)]
struct _Config {
    #[serde(default)]
    pub test: Option<_Test>,
    #[serde(default)]
    pub fuzz: Option<_Fuzz>,
}

impl From<_Config> for Config {
    fn from(_c: _Config) -> Self {
        Self {
            test: _c.test.unwrap_or_default().into(),
            fuzz: _c.fuzz.unwrap_or_default().into(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let root = Config::discover_root().expect("failed to find the root folder");
        let s = fs::read_to_string(root.join(TRDELNIK_TOML).as_path())
            .expect("failed to read the Trdelnik config file");
        let _config: _Config =
            toml::from_str(&s).expect("failed to parse the Trdelnik config file");
        _config.into()
    }

    /// Tries to find the root directory with the `Anchor.toml` file.
    /// Throws an error when there is no directory with the `Anchor.toml` file
    pub fn discover_root() -> Result<PathBuf, Error> {
        let current_dir = env::current_dir()?;
        let mut dir = Some(current_dir.as_path());
        while let Some(cwd) = dir {
            for file in std::fs::read_dir(cwd).with_context(|| {
                format!("Error reading the directory with path: {}", cwd.display())
            })? {
                let path = file
                    .with_context(|| {
                        format!("Error reading the directory with path: {}", cwd.display())
                    })?
                    .path();
                if let Some(filename) = path.file_name() {
                    if filename.to_str() == Some(ANCHOR_TOML) {
                        return Ok(PathBuf::from(cwd));
                    }
                }
            }
            dir = cwd.parent();
        }
        throw!(Error::BadWorkspace)
    }
    pub fn get_env_variable(self, cli_input: String) -> String {
        // Tested on a few examples, HFUZZ_RUN_ARGS give precedence to the later arguments.
        // so if HFUZZ_RUN_ARGS="-t 10 -t 15" -> timeout 15s is applied.
        // That means we do not need to parse the arguments from the CLI;
        // thus, we can simply append them at the end, and the CLI will have precedence.

        let mut toml_vars: String = String::new();
        for x in self.fuzz.fuzz_args {
            if x.short_opt.is_none() {
                if x.val.is_none() {
                    toml_vars = format!("{} {}", toml_vars, x.long_opt.unwrap());
                } else {
                    toml_vars =
                        format!("{} {} {}", toml_vars, x.short_opt.unwrap(), x.val.unwrap());
                }
            } else if x.val.is_none() {
                toml_vars = format!("{} {}", toml_vars, x.short_opt.unwrap());
            } else {
                toml_vars = format!("{} {} {}", toml_vars, x.short_opt.unwrap(), x.val.unwrap());
            }
        }
        toml_vars = toml_vars + " " + &cli_input;
        toml_vars
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_and_precedence1() {
        let config = Config {
            test: Test::default(),
            fuzz: Fuzz::default(),
        };

        let env_var_string = config.get_env_variable(String::default());
        assert_eq!(env_var_string, " -t 10 -N 0 ");
    }
    #[test]
    fn test_merge_and_precedence2() {
        let config = Config {
            test: Test::default(),
            fuzz: Fuzz::default(),
        };

        let env_var_string = config.get_env_variable("-t 0 -N10 --exit_upon_crash".to_string());

        assert_eq!(env_var_string, " -t 10 -N 0 -t 0 -N10 --exit_upon_crash");
    }
    #[test]
    fn test_merge_and_precedence3() {
        let config = Config {
            test: Test::default(),
            fuzz: Fuzz::default(),
        };
        let env_var_string =
            config.get_env_variable("-t 100 -N 5000 -Q -v --exit_upon_crash".to_string());
        assert_eq!(
            env_var_string,
            " -t 10 -N 0 -t 100 -N 5000 -Q -v --exit_upon_crash"
        );
    }
    #[test]
    fn test_merge_and_precedence4() {
        let config = Config {
            test: Test::default(),
            fuzz: Fuzz::default(),
        };

        let env_var_string = config.get_env_variable("-t 10 -N 500 -Q -v --exit_upon_crash -n 15 --mutations_per_run 8 --verifier -W random_dir --crashdir random_dir5 --run_time 666".to_string());
        assert_eq!(
            env_var_string,
            " -t 10 -N 0 -t 10 -N 500 -Q -v --exit_upon_crash -n 15 --mutations_per_run 8 --verifier -W random_dir --crashdir random_dir5 --run_time 666"
        );
    }
}
