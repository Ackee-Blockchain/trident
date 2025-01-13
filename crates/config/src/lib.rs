pub mod afl;
pub mod argument;
pub mod constants;
pub mod fuzz;
pub mod honggfuzz;

use afl::*;
use constants::*;
use fuzz::*;
use honggfuzz::*;

pub mod utils;

use serde::Deserialize;
use std::{fs, io};
use thiserror::Error;
use utils::{discover_root, resolve_path};

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
    pub honggfuzz: Option<HonggFuzz>,
    pub afl: Option<Afl>,
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
    // honggfuzz
    pub fn get_honggfuzz_args(&self, cli_input: String) -> String {
        if let Some(honggfuzz) = &self.honggfuzz {
            let mut args = honggfuzz.get_collect_fuzz_args();
            args.push(cli_input);
            args.join(" ")
        } else {
            String::default()
        }
    }
    pub fn get_honggfuzz_workspace(&self) -> String {
        let path = self
            .honggfuzz
            .as_ref()
            .map(|honggfuzz| honggfuzz.get_hfuzz_workspace().value)
            .unwrap_or_else(|| HFUZZ_WORKSPACE_DEFAULT_HFUZZ.to_string());
        let full_path = resolve_path(&path);
        full_path.to_str().unwrap().to_string()
    }
    pub fn get_honggfuzz_target_dir(&self) -> String {
        let path = self
            .honggfuzz
            .as_ref()
            .map(|honggfuzz| honggfuzz.get_cargo_target_dir().value)
            .unwrap_or_else(|| CARGO_TARGET_DIR_DEFAULT_HFUZZ.to_string());
        let full_path = resolve_path(&path);
        full_path.to_str().unwrap().to_string()
    }
    // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
    // afl
    pub fn get_afl_target_dir(&self) -> String {
        let path = self
            .afl
            .as_ref()
            .map(|afl| afl.get_cargo_target_dir().value.unwrap())
            .unwrap_or_else(|| CARGO_TARGET_DIR_DEFAULT_AFL.to_string());
        let full_path = resolve_path(&path);
        full_path.to_str().unwrap().to_string()
    }
    pub fn get_afl_target_path(&self, target: &str) -> String {
        let mut afl_target_dir = self.get_afl_target_dir();
        afl_target_dir.push_str("/debug/");
        afl_target_dir.push_str(target);
        afl_target_dir
    }
    pub fn get_afl_workspace_in(&self) -> String {
        let path = self
            .afl
            .as_ref()
            .map(|afl| afl.get_workspace_in().value.unwrap())
            .unwrap_or_else(|| AFL_WORKSPACE_DEFAULT_IN.to_string());
        let full_path = resolve_path(&path);
        full_path.to_str().unwrap().to_string()
    }
    pub fn get_afl_workspace_out(&self) -> String {
        let path = self
            .afl
            .as_ref()
            .map(|afl| afl.get_workspace_out().value.unwrap())
            .unwrap_or_else(|| AFL_WORKSPACE_DEFAULT_OUT.to_string());
        let full_path = resolve_path(&path);
        full_path.to_str().unwrap().to_string()
    }
    pub fn get_afl_build_args(&self) -> Vec<String> {
        self.afl
            .as_ref()
            .map(|afl| afl.get_collect_build_args())
            .unwrap_or_default()
    }
    pub fn get_afl_fuzz_args(&self) -> Vec<String> {
        self.afl
            .as_ref()
            .map(|afl| afl.get_collect_fuzz_args())
            .unwrap_or_default()
    }

    pub fn get_initial_seed(&self) -> Vec<AflSeed> {
        self.afl
            .as_ref()
            .map(|afl| afl.get_seeds())
            .unwrap_or_else(|| vec![AflSeed::default()])
    }
    // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
    // fuzz
    pub fn get_fuzzing_with_stats(&self) -> bool {
        self.fuzz
            .as_ref()
            .map(|fuzz| fuzz.get_fuzzing_with_stats())
            .unwrap_or_default()
    }
    pub fn get_allow_duplicate_txs(&self) -> bool {
        self.fuzz
            .as_ref()
            .map(|fuzz| fuzz.get_allow_duplicate_txs())
            .unwrap_or_default()
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
