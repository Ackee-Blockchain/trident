use crate::config::Config;
use fehler::{throw, throws};
use std::{
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tokio::{fs, process::Command};

use crate::constants::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("Cannot find the Anchor.toml file to locate the root folder")]
    BadWorkspace,
}

pub struct Cleaner;

impl Default for Cleaner {
    fn default() -> Self {
        Self::new()
    }
}

impl Cleaner {
    pub fn new() -> Self {
        Self
    }
    #[throws]
    pub async fn clean_target(&self) {
        let root = match Config::discover_root() {
            Ok(root) => root,
            Err(_) => throw!(Error::BadWorkspace),
        };
        self.clean_anchor_target().await?;
        self.clean_hfuzz_target(&root).await?;
    }

    #[throws]
    async fn clean_anchor_target(&self) {
        Command::new("anchor").arg("clean").spawn()?.wait().await?;
    }
    #[throws]
    async fn clean_hfuzz_target(&self, root: &PathBuf) {
        let hfuzz_target_path = Path::new(root)
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(FUZZ_TEST_DIRECTORY)
            .join(FUZZING)
            .join(HFUZZ_TARGET);
        if hfuzz_target_path.exists() {
            fs::remove_dir_all(hfuzz_target_path).await?;
        } else {
            println!(
                "{SKIP} [{}/{}/{}/{}] directory not found",
                TESTS_WORKSPACE_DIRECTORY, FUZZ_TEST_DIRECTORY, FUZZING, HFUZZ_TARGET
            )
        }
    }
}
