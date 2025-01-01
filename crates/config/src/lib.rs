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
pub struct Config {
    pub honggfuzz: Option<HonggFuzz>,
    pub afl: Option<Afl>,
    pub fuzz: Option<Fuzz>,
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
        let _config: Config = toml::from_str(&s).expect("failed to parse the Trident config file");
        _config
    }

    pub fn get_honggfuzz_args(&self, cli_input: String) -> String {
        if let Some(honggfuzz) = &self.honggfuzz {
            let mut args = honggfuzz.get_collect_fuzz_args();
            args.push(cli_input);
            args.join(" ")
        } else {
            String::default()
        }
    }
    pub fn get_honggfuzz_target_dir(&self) -> String {
        if let Some(honggfuzz) = &self.honggfuzz {
            honggfuzz.get_cargo_target_dir().value
        } else {
            CARGO_TARGET_DIR_DEFAULT_HFUZZ.to_string()
        }
    }
    pub fn get_honggfuzz_workspace(&self) -> String {
        if let Some(honggfuzz) = &self.honggfuzz {
            honggfuzz.get_hfuzz_workspace().value
        } else {
            HFUZZ_WORKSPACE_DEFAULT_HFUZZ.to_string()
        }
    }

    pub fn get_afl_build_args(&self) -> Vec<String> {
        if let Some(afl) = &self.afl {
            afl.get_collect_build_args()
        } else {
            // if nothing is provided, use the default target dir
            vec![format!("--target-dir {}", CARGO_TARGET_DIR_DEFAULT_AFL)]
        }
    }
    pub fn get_afl_fuzz_args(&self) -> Vec<String> {
        if let Some(afl) = &self.afl {
            afl.get_collect_fuzz_args()
        } else {
            // if nothing is provided, use the default workspace in and out
            vec![
                format!("-i {}", AFL_WORKSPACE_DEFAULT_IN),
                format!("-o {}", AFL_WORKSPACE_DEFAULT_OUT),
            ]
        }
    }
    pub fn get_afl_cargo_build_dir(&self) -> String {
        if let Some(afl) = &self.afl {
            afl.get_cargo_build_dir()
                .expect("AFL Cargo Target Dir argument not available")
                .value
                .clone()
                .expect("AFL Cargo Target Dir value not available")
        } else {
            // if nothing is provided, use the default target dir
            CARGO_TARGET_DIR_DEFAULT_AFL.to_string()
        }
    }
    pub fn get_afl_target_path(&self) -> String {
        if let Some(afl) = &self.afl {
            let afl_arg = afl
                .get_cargo_build_dir()
                .expect("AFL Cargo Target Dir argument not available");

            let mut target_path = afl_arg
                .value
                .clone()
                .expect("AFL Cargo Target Dir value not available");

            target_path.push_str("/debug/");
            target_path
        } else {
            // if nothing is provided, use the default target dir
            let mut target_path = CARGO_TARGET_DIR_DEFAULT_AFL.to_string();
            target_path.push_str("/debug/");
            target_path
        }
    }
    pub fn get_afl_workspace_in(&self) -> String {
        if let Some(afl) = &self.afl {
            let afl_arg = afl
                .get_workspace_in()
                .expect("AFL Workspace in value argument available");

            afl_arg
                .value
                .clone()
                .expect("AFL Workspace in value not available")
        } else {
            // if nothing is provided, use the default workspace in
            AFL_WORKSPACE_DEFAULT_IN.to_string()
        }
    }
    pub fn get_initial_seed(&self) -> Vec<AflSeed> {
        if let Some(afl) = &self.afl {
            afl.get_seeds()
        } else {
            // if nothing is provided, use the default seed
            vec![AflSeed::default()]
        }
    }

    pub fn get_fuzzing_with_stats(&self) -> bool {
        if let Some(fuzz) = &self.fuzz {
            fuzz.get_fuzzing_with_stats()
        } else {
            false
        }
    }
    pub fn get_allow_duplicate_txs(&self) -> bool {
        if let Some(fuzz) = &self.fuzz {
            fuzz.get_allow_duplicate_txs()
        } else {
            false
        }
    }

    pub fn programs(&self) -> Vec<FuzzProgram> {
        if let Some(fuzz) = &self.fuzz {
            if let Some(programs) = &fuzz.programs {
                programs.iter().map(FuzzProgram::from).collect()
            } else {
                Vec::default()
            }
        } else {
            Vec::default()
        }
    }
    pub fn accounts(&self) -> Vec<FuzzAccount> {
        if let Some(fuzz) = &self.fuzz {
            if let Some(accounts) = &fuzz.accounts {
                accounts.iter().map(FuzzAccount::from).collect()
            } else {
                Vec::default()
            }
        } else {
            Vec::default()
        }
    }
}
