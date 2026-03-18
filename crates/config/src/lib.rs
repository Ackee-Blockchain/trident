pub mod constants;
pub mod coverage;
pub mod fork;
pub mod fuzz;
mod metrics;
pub mod rpc_client;
use constants::*;
use coverage::*;
use fuzz::*;
pub mod utils;

use serde::Deserialize;
use solana_sdk::account::AccountSharedData;
use solana_sdk::pubkey::Pubkey;
use std::fs;
use std::io;
use thiserror::Error;
use utils::discover_root;

use crate::rpc_client::fork::load_forks_from_cache;
use crate::rpc_client::fork::process_forks;
mod regression;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid workspace")]
    BadWorkspace,
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
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
        Self::try_new().unwrap_or_else(|e| panic!("failed to load Trident config: {e}"))
    }

    pub fn try_new() -> Result<Self, Error> {
        let root = discover_root()?;
        let s = fs::read_to_string(root.join(TRIDENT_TOML).as_path())?;
        let config: TridentConfig = toml::from_str(&s)?;
        config.validate_preflight()?;

        Ok(config)
    }

    pub fn validate_preflight(&self) -> Result<(), Error> {
        if let Some(fuzz) = self.fuzz.as_ref() {
            fuzz.validate_preflight()?;
        }
        Ok(())
    }

    // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
    // fuzz
    pub fn get_metrics(&self) -> bool {
        self.fuzz
            .as_ref()
            .map(|fuzz| fuzz.get_metrics())
            .unwrap_or_default()
    }

    pub fn get_metrics_json(&self) -> bool {
        self.fuzz
            .as_ref()
            .map(|fuzz| fuzz.get_metrics_json())
            .unwrap_or_default()
    }

    pub fn get_metrics_dashboard(&self) -> bool {
        self.fuzz
            .as_ref()
            .map(|fuzz| fuzz.get_metrics_dashboard())
            .unwrap_or_default()
    }

    pub fn get_regression(&self) -> bool {
        self.fuzz
            .as_ref()
            .map(|fuzz| fuzz.get_regression())
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
        self.try_programs()
            .unwrap_or_else(|e| panic!("failed to parse programs in Trident.toml: {e}"))
    }

    pub fn try_programs(&self) -> Result<Vec<FuzzProgram>, Error> {
        self.fuzz
            .as_ref()
            .map(|fuzz| {
                if let Some(programs) = &fuzz.programs {
                    programs.iter().map(FuzzProgram::try_from_raw).collect()
                } else {
                    Ok(Vec::default())
                }
            })
            .unwrap_or_else(|| Ok(Vec::default()))
    }

    pub fn accounts(&self) -> Vec<FuzzAccount> {
        self.try_accounts()
            .unwrap_or_else(|e| panic!("failed to parse accounts in Trident.toml: {e}"))
    }

    pub fn try_accounts(&self) -> Result<Vec<FuzzAccount>, Error> {
        self.fuzz
            .as_ref()
            .map(|fuzz| {
                if let Some(accounts) = &fuzz.accounts {
                    accounts.iter().map(FuzzAccount::try_from_raw).collect()
                } else {
                    Ok(Vec::default())
                }
            })
            .unwrap_or_else(|| Ok(Vec::default()))
    }

    pub fn forks(&self) -> Vec<fuzz::FuzzFork> {
        self.try_forks()
            .unwrap_or_else(|e| panic!("failed to parse forks in Trident.toml: {e}"))
    }

    pub fn try_forks(&self) -> Result<Vec<fuzz::FuzzFork>, Error> {
        self.fuzz
            .as_ref()
            .map(|fuzz| fuzz.get_forks())
            .unwrap_or_else(|| Ok(Vec::default()))
    }

    /// Get forked accounts from cache (silent, no RPC calls).
    ///
    /// This should be called after `fork()` has been executed to ensure
    /// all accounts are cached. Used by worker threads during parallel fuzzing.
    pub fn get_forked_accounts(&self) -> Vec<(Pubkey, AccountSharedData)> {
        let forks = match self.try_forks() {
            Ok(forks) => forks,
            Err(e) => {
                eprintln!("Warning: Failed to parse forks from config: {}", e);
                return Vec::new();
            }
        };
        if forks.is_empty() {
            return Vec::new();
        }

        match load_forks_from_cache(&forks) {
            Ok(fork_results) => fork_results,
            Err(e) => {
                eprintln!("Warning: Failed to load forks from cache: {}", e);
                Vec::new()
            }
        }
    }

    /// Process all fork entries - fetch from RPC if needed and cache them.
    ///
    /// This should be called ONCE in the main thread before parallel fuzzing starts.
    pub fn fork(&self) -> Result<(), Box<dyn std::error::Error>> {
        let forks = self.try_forks()?;
        if forks.is_empty() {
            return Ok(());
        }

        process_forks(&forks)
    }
}
