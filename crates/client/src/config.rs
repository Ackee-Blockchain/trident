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

impl Default for Test {
    fn default() -> Self {
        Self {
            validator_startup_timeout: 10_000,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct HfuzzRunArgs {
    // short option, long option, value
    pub hfuzz_run_args: Vec<(String, String, String)>,
    pub remaining_cli_args: Option<String>,
}

impl Default for HfuzzRunArgs {
    fn default() -> Self {
        Self {
            hfuzz_run_args: vec![
                ("-t".to_string(), "--timeout".to_string(), 10.to_string()),
                ("-N".to_string(), "--iterations".to_string(), 0.to_string()),
                (
                    "-Q".to_string(),
                    "--keep_output".to_string(),
                    "false".to_string(),
                ),
                (
                    "-v".to_string(),
                    "--verbose".to_string(),
                    "false".to_string(),
                ),
                (
                    String::new(),
                    "--exit_upon_crash".to_string(),
                    "false".to_string(),
                ),
            ],
            remaining_cli_args: None,
        }
    }
}

#[derive(Default, Debug, Deserialize, Clone)]
struct _Test {
    #[serde(default)]
    pub validator_startup_timeout: Option<u64>,
}

#[derive(Default, Debug, Deserialize, Clone)]
struct _HfuzzRunArgs {
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

impl From<_Test> for Test {
    fn from(_t: _Test) -> Self {
        Self {
            validator_startup_timeout: _t.validator_startup_timeout.unwrap_or(10_000),
        }
    }
}

impl From<_HfuzzRunArgs> for HfuzzRunArgs {
    fn from(_f: _HfuzzRunArgs) -> Self {
        let timeout = _f.timeout.unwrap_or(10);
        let iterations = _f.iterations.unwrap_or(0);
        let keep_output = _f.keep_output.unwrap_or(false);
        let verbose = _f.verbose.unwrap_or(false);
        let exit_upon_crash = _f.exit_upon_crash.unwrap_or(false);
        Self {
            hfuzz_run_args: vec![
                (
                    "-t".to_string(),
                    "--timeout".to_string(),
                    timeout.to_string(),
                ),
                (
                    "-N".to_string(),
                    "--iterations".to_string(),
                    iterations.to_string(),
                ),
                (
                    "-Q".to_string(),
                    "--keep_output".to_string(),
                    keep_output.to_string(),
                ),
                (
                    "-v".to_string(),
                    "--verbose".to_string(),
                    verbose.to_string(),
                ),
                (
                    String::new(),
                    "--exit_upon_crash".to_string(),
                    exit_upon_crash.to_string(),
                ),
            ],
            remaining_cli_args: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub test: Test,
    pub hfuzz_run_args: HfuzzRunArgs,
}

#[derive(Default, Debug, Deserialize, Clone)]
struct _Config {
    #[serde(default)]
    pub test: Option<_Test>,
    #[serde(default)]
    pub hfuzz_run_args: Option<_HfuzzRunArgs>,
}

impl From<_Config> for Config {
    fn from(_c: _Config) -> Self {
        Self {
            test: _c.test.unwrap_or_default().into(),
            hfuzz_run_args: _c.hfuzz_run_args.unwrap_or_default().into(),
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
    pub fn merge_with_cli(&mut self, cli_vars: &str) {
        // FIXME: we split by whitespace without respecting escaping or quotes - same approach as honggfuzz-rs so there is no point to fix it here before the upstream is fixed
        let mut splitted_in_vector: Vec<String> =
            cli_vars.split_whitespace().map(str::to_string).collect();

        for x in &mut self.hfuzz_run_args.hfuzz_run_args {
            // no need to re-format these as the long/short formats are predefined by us
            let short_opt = &x.0;
            let long_opt = &x.1;

            let mut index = 0;
            while let Some(arg) = splitted_in_vector.get(index) {
                match arg.strip_prefix(short_opt) {
                    Some(_val) if short_opt.len() > 1 => {
                        // TODO: this expects only two possible inputs, but within the get_cmd_option_value
                        // we check also for -ecrash
                        // -t timeout
                        // -v <no next input because flag signals true>
                        if x.2 == "true" || x.2 == "false" {
                            // -v
                            x.2 = "true".to_owned();
                            splitted_in_vector.remove(index);
                        } else if let Some(_next_arg) = splitted_in_vector.get(index + 1) {
                            // -t timeout
                            splitted_in_vector.remove(index);
                            x.2 = splitted_in_vector.remove(index);
                        } else {
                            index += 1;
                        }
                    }
                    _ => {
                        // This part also expects only two possible inputs
                        // --exit_upon_crash <signals true>
                        // --iterations VALUE
                        if arg.starts_with(long_opt) && long_opt.len() > 2 {
                            if x.2 == "false" || x.2 == "true" {
                                // --exit_upon_crash <signals true>
                                x.2 = "true".to_owned();
                                splitted_in_vector.remove(index);
                                continue;
                            } else if let Some(_next_arg) = splitted_in_vector.get(index + 1) {
                                // --iterations VALUE
                                splitted_in_vector.remove(index);
                                x.2 = splitted_in_vector.remove(index);
                                continue;
                            }
                        }
                        index += 1;
                    }
                }
            }
        }
        if !splitted_in_vector.is_empty() {
            let remaining_cli_args = splitted_in_vector.join(" ");
            self.hfuzz_run_args.remaining_cli_args = Some(remaining_cli_args);
        }
    }
    pub fn get_env_variables(&self) -> String {
        let mut toml_vars: String = String::new();
        for x in &self.hfuzz_run_args.hfuzz_run_args {
            if x.2 == "true" {
                // add only flag
                if x.0.is_empty() {
                    toml_vars = format!("{} {}", toml_vars, x.1);
                } else {
                    toml_vars = format!("{} {}", toml_vars, x.0);
                }
            } else if x.2 == "false" {
                // do nothing
            } else if x.0.is_empty() {
                // add long form with value, this means short forms are filled first
                // by default if they are present
                toml_vars = format!("{} {} {}", toml_vars, x.1, x.2);
            } else {
                // add short form with value
                toml_vars = format!("{} {} {}", toml_vars, x.0, x.2);
            }
        }

        if let Some(remaining_cli_args) = &self.hfuzz_run_args.remaining_cli_args {
            toml_vars = toml_vars + " " + remaining_cli_args;
        }
        toml_vars
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_and_precedence1() {
        let mut config = Config {
            test: Test::default(),
            hfuzz_run_args: HfuzzRunArgs::default(),
        };

        config.merge_with_cli("");
        let env_var_string = config.get_env_variables();
        assert_eq!(env_var_string, " -t 10 -N 0");
    }
    #[test]
    fn test_merge_and_precedence2() {
        let mut config = Config {
            test: Test::default(),
            hfuzz_run_args: HfuzzRunArgs {
                hfuzz_run_args: vec![
                    ("-t".to_string(), "--timeout".to_string(), 423.to_string()),
                    (
                        "-N".to_string(),
                        "--iterations".to_string(),
                        8943.to_string(),
                    ),
                    (
                        "-Q".to_string(),
                        "--keep_output".to_string(),
                        "true".to_string(),
                    ),
                    (
                        "-v".to_string(),
                        "--verbose".to_string(),
                        "false".to_string(),
                    ),
                    (
                        String::new(),
                        "--exit_upon_crash".to_string(),
                        "true".to_string(),
                    ),
                ],
                remaining_cli_args: None,
            },
        };

        config.merge_with_cli("");
        let env_var_string = config.get_env_variables();
        assert_eq!(env_var_string, " -t 423 -N 8943 -Q --exit_upon_crash");
    }
    #[test]
    fn test_merge_and_precedence3() {
        let mut config = Config {
            test: Test::default(),
            hfuzz_run_args: HfuzzRunArgs {
                hfuzz_run_args: vec![
                    ("-t".to_string(), "--timeout".to_string(), 423.to_string()),
                    (
                        "-N".to_string(),
                        "--iterations".to_string(),
                        8943.to_string(),
                    ),
                    (
                        "-Q".to_string(),
                        "--keep_output".to_string(),
                        "false".to_string(),
                    ),
                    (
                        "-v".to_string(),
                        "--verbose".to_string(),
                        "false".to_string(),
                    ),
                    (
                        String::new(),
                        "--exit_upon_crash".to_string(),
                        "false".to_string(),
                    ),
                ],
                remaining_cli_args: None,
            },
        };

        config.merge_with_cli("--exit_upon_crash");
        let env_var_string = config.get_env_variables();
        assert_eq!(env_var_string, " -t 423 -N 8943 --exit_upon_crash");
    }
    #[test]
    fn test_merge_and_precedence4() {
        let mut config = Config {
            test: Test::default(),
            hfuzz_run_args: HfuzzRunArgs {
                hfuzz_run_args: vec![
                    ("-t".to_string(), "--timeout".to_string(), 1.to_string()),
                    ("-N".to_string(), "--iterations".to_string(), 1.to_string()),
                    (
                        "-Q".to_string(),
                        "--keep_output".to_string(),
                        "false".to_string(),
                    ),
                    (
                        "-v".to_string(),
                        "--verbose".to_string(),
                        "false".to_string(),
                    ),
                    (
                        String::new(),
                        "--exit_upon_crash".to_string(),
                        "false".to_string(),
                    ),
                ],
                remaining_cli_args: None,
            },
        };

        config.merge_with_cli("-t 10 -N 500 -Q -v --exit_upon_crash");
        let env_var_string = config.get_env_variables();
        assert_eq!(env_var_string, " -t 10 -N 500 -Q -v --exit_upon_crash");
    }
    #[test]
    fn test_merge_and_precedence5() {
        let mut config = Config {
            test: Test::default(),
            hfuzz_run_args: HfuzzRunArgs {
                hfuzz_run_args: vec![
                    ("-t".to_string(), "--timeout".to_string(), 1.to_string()),
                    ("-N".to_string(), "--iterations".to_string(), 1.to_string()),
                    (
                        "-Q".to_string(),
                        "--keep_output".to_string(),
                        "false".to_string(),
                    ),
                    (
                        "-v".to_string(),
                        "--verbose".to_string(),
                        "false".to_string(),
                    ),
                    (
                        String::new(),
                        "--exit_upon_crash".to_string(),
                        "false".to_string(),
                    ),
                ],
                remaining_cli_args: None,
            },
        };

        config.merge_with_cli(
            "-t 10 -N 500 -Q -v --exit_upon_crash -n 15 --mutations_per_run 8 --verifier -W random_dir --crashdir random_dir5 --run_time 666",
        );
        let env_var_string = config.get_env_variables();
        assert_eq!(
            env_var_string,
            " -t 10 -N 500 -Q -v --exit_upon_crash -n 15 --mutations_per_run 8 --verifier -W random_dir --crashdir random_dir5 --run_time 666"
        );
    }
    #[test]
    fn test_merge_and_precedence6() {
        let mut config = Config {
            test: Test::default(),
            hfuzz_run_args: HfuzzRunArgs {
                hfuzz_run_args: vec![
                    ("-t".to_string(), "--timeout".to_string(), 1.to_string()),
                    ("-N".to_string(), "--iterations".to_string(), 1.to_string()),
                    (
                        "-Q".to_string(),
                        "--keep_output".to_string(),
                        "true".to_string(),
                    ),
                    (
                        "-v".to_string(),
                        "--verbose".to_string(),
                        "true".to_string(),
                    ),
                    (
                        String::new(),
                        "--exit_upon_crash".to_string(),
                        "true".to_string(),
                    ),
                ],
                remaining_cli_args: None,
            },
        };

        config.merge_with_cli(
            "-n 15 --mutations_per_run 8 --verifier -W random_dir --crashdir random_dir5 --run_time 666",
        );
        let env_var_string = config.get_env_variables();
        assert_eq!(
            env_var_string,
            " -t 1 -N 1 -Q -v --exit_upon_crash -n 15 --mutations_per_run 8 --verifier -W random_dir --crashdir random_dir5 --run_time 666"
        );
    }
}
