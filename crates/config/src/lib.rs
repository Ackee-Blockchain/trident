pub mod constants;
pub mod coverage;
pub mod fuzz;
mod metrics;
use constants::*;
use coverage::*;
use fuzz::*;

pub mod utils;

use serde::Deserialize;
use std::fs;
use std::io;
use thiserror::Error;
use utils::discover_root;

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
pub struct TridentConfig {
    pub fuzz: Option<Fuzz>,
}

impl Default for TridentConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TridentConfig {
    pub fn new() -> Self {
        let root = discover_root().expect("failed to find the root folder");
        let s = fs::read_to_string(root.join(TRIDENT_TOML).as_path())
            .expect("failed to read the Trident config file");
        let _config: TridentConfig =
            toml::from_str(&s).expect("failed to parse the Trident config file");
        _config
    }

    // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
    // fuzz
    pub fn get_fuzzing_with_stats(&self) -> bool {
        self.fuzz
            .as_ref()
            .map(|fuzz| fuzz.get_fuzzing_with_stats())
            .unwrap_or_default()
    }

    pub fn get_dashboard(&self) -> bool {
        self.fuzz
            .as_ref()
            .map(|fuzz| fuzz.get_dashboard())
            .unwrap_or_default()
    }

    pub fn get_coverage(&self) -> Coverage {
        self.fuzz
            .as_ref()
            .map(|fuzz| fuzz.get_coverage())
            .unwrap_or_default()
    }

    pub fn loopcount(&self) -> u64 {
        self.get_coverage().get_loopcount()
    }

    pub fn coverage_server_port(&self) -> u16 {
        self.get_coverage().get_server_port()
    }

    pub fn programs(&self) -> Vec<FuzzProgram> {
        self.fuzz
            .as_ref()
            .map(|fuzz| {
                if let Some(programs) = &fuzz.programs {
                    programs.iter().map(FuzzProgram::from).collect()
                } else {
                    Vec::default()
                }
            })
            .unwrap_or_default()
    }

    pub fn accounts(&self) -> Vec<FuzzAccount> {
        self.fuzz
            .as_ref()
            .map(|fuzz| {
                if let Some(accounts) = &fuzz.accounts {
                    accounts.iter().map(FuzzAccount::from).collect()
                } else {
                    Vec::default()
                }
            })
            .unwrap_or_default()
    }
}
