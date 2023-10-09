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

#[derive(Debug, Deserialize, Clone)]
pub struct HfuzzRunArgs {
    // short option, long option, value
    pub hfuzz_run_args: Vec<(String, String, String)>,
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
    pub fn get_fuzz_env_variable(&self, cli_vars: &String) -> String {
        let mut fuzz_args: Vec<(String, String, String)> =
            self.hfuzz_run_args.hfuzz_run_args.clone();

        if !cli_vars.is_empty() {
            // FIXME: we split by whitespace without respecting escaping or quotes - same approach as honggfuzz-rs so there is no point to fix it here before the upstream is fixed

            for x in &mut fuzz_args {
                let split_whitespaces = cli_vars.split_whitespace();
                let mut args_iter = split_whitespaces;

                let short_opt = format!("-{}", x.0.trim_start_matches('-'));
                let long_opt = format!("--{}", x.1.trim_start_matches('-'));
                while let Some(arg) = args_iter.next() {
                    match arg.strip_prefix(&short_opt) {
                        Some(val) if short_opt.len() > 1 => {
                            if val.is_empty() && (x.2 == "true" || x.2 == "false") {
                                // -v single bool options
                                x.2 = "true".to_owned();
                            } else if !val.is_empty() {
                                // -ecrash for crash extension with no space
                                x.2 = val.to_string();
                            } else if let Some(next_arg) = args_iter.next() {
                                // -e crash for crash extension with space
                                // -t timeout
                                x.2 = next_arg.to_string();
                            }
                        }
                        _ => {
                            if arg.starts_with(&long_opt) && long_opt.len() > 2 {
                                if x.2 == "false" || x.2 == "true" {
                                    // --exit_upon_crash
                                    x.2 = "true".to_owned()
                                } else if let Some(next_arg) = args_iter.next() {
                                    // --iterations value
                                    x.2 = next_arg.to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut toml_vars: String = String::new();
        for x in &fuzz_args {
            if x.2 == "true" {
                // add only variable
                if x.0.is_empty() {
                    toml_vars = format!("{} {}", toml_vars, x.1);
                } else {
                    toml_vars = format!("{} {}", toml_vars, x.0);
                }
            } else if x.2 == "false" {
                // do nothing
            } else if x.0.is_empty() {
                // add long form with value
                toml_vars = format!("{} {} {}", toml_vars, x.1, x.2);
            } else {
                // add short form with value
                toml_vars = format!("{} {} {}", toml_vars, x.0, x.2);
            }
        }
        toml_vars
    }
}
lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}
