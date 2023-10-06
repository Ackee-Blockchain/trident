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
pub struct Fuzz {
    /// Timeout in seconds (default: 10)
    /// -t
    pub timeout: (char, u16),
    /// Number of fuzzing iterations (default: 0 [no limit])
    /// -N
    pub iterations: (char, u64),
    /// Don't close children's stdin, stdout, stderr; can be noisy
    /// -Q
    pub keep_output: (char, bool),
    /// Disable ANSI console; use simple log output
    /// -v
    pub verbose: (char, bool),
}

#[derive(Default, Debug, Deserialize, Clone)]
struct _Test {
    #[serde(default)]
    pub validator_startup_timeout: Option<u64>,
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
}

impl From<_Test> for Test {
    fn from(_t: _Test) -> Self {
        Self {
            validator_startup_timeout: _t.validator_startup_timeout.unwrap_or(10_000),
        }
    }
}

impl From<_Fuzz> for Fuzz {
    fn from(_f: _Fuzz) -> Self {
        Self {
            timeout: ('t', _f.timeout.unwrap_or(10)),
            iterations: ('N', _f.iterations.unwrap_or(0)),
            keep_output: ('Q', _f.keep_output.unwrap_or(false)),
            verbose: ('v', _f.verbose.unwrap_or(false)),
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
    pub fn get_fuzz_env_variable(&self) -> String {
        let iterations = &self.fuzz.iterations;
        let timeout = &self.fuzz.timeout;
        let verbose = &self.fuzz.verbose;
        let keep_out = &self.fuzz.keep_output;

        let mut variable_string = format!(
            "-{} {} -{} {}",
            iterations.0, iterations.1, timeout.0, timeout.1,
        );

        variable_string = match verbose.1 {
            true => format!("{} -{}", variable_string, verbose.0),
            false => variable_string,
        };

        variable_string = match keep_out.1 {
            true => format!("{} -{}", variable_string, keep_out.0),
            false => variable_string,
        };

        variable_string
    }
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}
