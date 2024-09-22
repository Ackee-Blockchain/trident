use crate::config::discover_root;
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
        let root = match discover_root() {
            Ok(root) => root,
            Err(_) => throw!(Error::BadWorkspace),
        };
        self.clean_anchor_target().await?;
        self.clean_hfuzz_target(&root).await?;
        self.clean_afl_target(&root).await?;
    }

    #[throws]
    async fn clean_anchor_target(&self) {
        Command::new("anchor").arg("clean").spawn()?.wait().await?;
    }
    #[throws]
    async fn clean_hfuzz_target(&self, root: &PathBuf) {
        let hfuzz_target_path = Path::new(root).join(CARGO_TARGET_DIR_DEFAULT_HFUZZ);
        if hfuzz_target_path.exists() {
            fs::remove_dir_all(hfuzz_target_path).await?;
        } else {
            println!(
                "{SKIP} [{}] directory not found",
                CARGO_TARGET_DIR_DEFAULT_HFUZZ
            )
        }
    }

    #[throws]
    async fn clean_afl_target(&self, root: &PathBuf) {
        let afl_target_path = Path::new(root).join(CARGO_TARGET_DIR_DEFAULT_AFL);
        if afl_target_path.exists() {
            fs::remove_dir_all(afl_target_path).await?;
        } else {
            println!(
                "{SKIP} [{}] directory not found",
                CARGO_TARGET_DIR_DEFAULT_AFL
            )
        }
    }
}
