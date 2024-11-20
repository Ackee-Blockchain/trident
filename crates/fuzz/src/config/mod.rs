pub mod afl;
pub mod constants;
pub mod fuzz;
pub mod honggfuzz;

use afl::*;
use constants::*;
use fuzz::*;
use honggfuzz::*;

use anyhow::Context;
use fehler::throw;
use serde::Deserialize;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

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
pub struct Config {
    pub honggfuzz: HonggFuzz,
    pub afl: Afl,
    pub fuzz: Fuzz,
}

#[derive(Default, Debug, Deserialize, Clone)]
struct _Config {
    #[serde(default)]
    pub honggfuzz: Option<_HonggFuzz>,
    #[serde(default)]
    pub afl: Option<_Afl>,
    #[serde(default)]
    pub fuzz: Option<_Fuzz>,
}

impl From<_Config> for Config {
    fn from(_c: _Config) -> Self {
        Self {
            honggfuzz: _c.honggfuzz.unwrap_or_default().into(),
            afl: _c.afl.unwrap_or_default().into(),
            fuzz: _c.fuzz.unwrap_or_default().into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let root = discover_root().expect("failed to find the root folder");
        let s = fs::read_to_string(root.join(TRIDENT_TOML).as_path())
            .expect("failed to read the Trident config file");
        let _config: _Config = toml::from_str(&s).expect("failed to parse the Trident config file");
        _config.into()
    }

    pub fn get_honggfuzz_args(&self, cli_input: String) -> String {
        let mut args = self.honggfuzz.get_collect_fuzz_args();
        args.push(cli_input);
        args.join(" ")
    }
    pub fn get_env_arg(&self, key: &EnvVariable) -> String {
        let expect = format!("{:#?} not found", key);
        self.honggfuzz.get_env_variable(key).expect(&expect)
    }

    pub fn get_afl_build_args(&self) -> Vec<String> {
        self.afl.get_collect_build_args()
    }
    pub fn get_afl_fuzz_args(&self) -> Vec<String> {
        self.afl.get_collect_fuzz_args()
    }
    pub fn get_afl_cargo_build_dir(&self) -> String {
        self.afl
            .get_cargo_build_dir()
            .expect("AFL Cargo Target Dir argument not available")
            .val
            .clone()
            .expect("AFL Cargo Target Dir value not available")
    }
    pub fn get_afl_target_path(&self) -> String {
        let afl_arg = self
            .afl
            .get_cargo_build_dir()
            .expect("AFL Cargo Target Dir argument not available");

        let mut target_path = afl_arg
            .val
            .clone()
            .expect("AFL Cargo Target Dir value not available");

        target_path.push_str("/debug/");
        target_path
    }
    pub fn get_afl_workspace_in(&self) -> String {
        let afl_arg = self
            .afl
            .get_workspace_in()
            .expect("AFL Workspace in value argument available");

        afl_arg
            .val
            .clone()
            .expect("AFL Workspace in value not available")
    }
    pub fn get_initial_seed(&self) -> &[AflSeed] {
        &self.afl.seeds
    }

    pub fn get_fuzzing_with_stats(&self) -> bool {
        self.fuzz.get_fuzzing_with_stats()
    }
    pub fn get_allow_duplicate_txs(&self) -> bool {
        self.fuzz.get_allow_duplicate_txs()
    }
}

/// Tries to find the root directory with the `Anchor.toml` file.
/// Throws an error when there is no directory with the `Anchor.toml` file
pub fn discover_root() -> Result<PathBuf, Error> {
    let current_dir = env::current_dir()?;
    let mut dir = Some(current_dir.as_path());
    while let Some(cwd) = dir {
        for file in std::fs::read_dir(cwd)
            .with_context(|| format!("Error reading the directory with path: {}", cwd.display()))?
        {
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

pub fn find_full_path(input: &str) -> Result<PathBuf, std::io::Error> {
    let path = Path::new(input);

    // If the path is absolute, return it directly
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    let root = discover_root().expect("Failed to obtain Anchor root");

    Ok(root.join(path))
}
