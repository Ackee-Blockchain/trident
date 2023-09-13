use crate::config::Config;
use fehler::{throw, throws};
use std::{
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tokio::{fs, process::Command};

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot parse Cargo.toml")]
    CannotParseCargoToml,
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    Toml(#[from] toml::de::Error),
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
        // TODO this will look for parent dir with Anchor.toml,
        // so command does not have to be called from project root
        let root = match Config::discover_root() {
            Ok(root) => root,
            Err(_) => throw!(Error::BadWorkspace),
        };
        self.clean_anchor_target(&root).await?;
        self.clean_hfuzz_target(&root).await?;
    }

    #[throws]
    async fn clean_anchor_target(&self, _root: &PathBuf) {
        let _success = Command::new("anchor")
            .arg("clean")
            .spawn()?
            .wait()
            .await?
            .success();
    }
    #[throws]
    async fn clean_hfuzz_target(&self, root: &PathBuf) {
        let hfuzz_target_path = Path::new(root)
            .join(crate::test_generator::TESTS_WORKSPACE)
            .join(crate::test_generator::HFUZZ_TARGET);
        if hfuzz_target_path.try_exists().unwrap() {
            fs::remove_dir_all(hfuzz_target_path).await?;
        } else {
            println!(
                "skipping {}: not found",
                crate::test_generator::HFUZZ_TARGET
            )
        }
    }
}
