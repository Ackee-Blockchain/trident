use anyhow::Context;
use fehler::{throw, throws};
use std::{
    env, io,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tokio::process::Command;

use crate::constants::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("Cannot find the Anchor.toml file to locate the root folder")]
    BadWorkspace,
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
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
        self.clean_anchor_target(&root).await?;
        self.clean_fuzz_target(&root).await?;
    }

    #[throws]
    async fn clean_anchor_target(&self, root: &Path) {
        Command::new("anchor")
            .arg("clean")
            .current_dir(root)
            .spawn()?
            .wait()
            .await?;
    }

    #[throws]
    #[allow(dead_code)]
    async fn clean_fuzz_target(&self, root: &Path) {
        let trident_tests_dir = root.join("trident-tests");
        Command::new("cargo")
            .arg("clean")
            .current_dir(trident_tests_dir)
            .spawn()?
            .wait()
            .await?;
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
