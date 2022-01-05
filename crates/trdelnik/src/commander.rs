use fehler::throws;
use thiserror::Error;
use tokio::{process::{Command, Child}};
use std::{borrow::Cow, io};
use solana_sdk::signer::keypair::Keypair;
use tokio::fs;
use crate::Client;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    IoError(#[from] io::Error),
    #[error("localnet is not running")]
    LocalnetIsNotRunning,
    #[error("localnet is still running")]
    LocalnetIsStillRunning,
    #[error("build programs failed")]
    BuildProgramsFailed,
}

pub struct LocalnetHandle {
    solana_test_validator_process: Child,
}

impl LocalnetHandle {
    #[throws]
    /// _Note_: Manual kill: `kill -9 $(lsof -t -i:8899)`
    pub async fn stop(mut self) {
        self.solana_test_validator_process.kill().await?;
        if Client::new(Keypair::new()).is_localnet_running(false).await {
            Err(Error::LocalnetIsStillRunning)?
        }
        fs::remove_dir_all("test-ledger").await?;
        println!("localnet stopped and its ledger deleted");
    }
}

pub struct Commander {
    root: Cow<'static, str>
}

impl Commander {
    pub fn new() -> Self {
        Self {
            root: "../../".into()
        }
    }

    pub fn with_root(root: impl Into<Cow<'static, str>>) -> Self {
        Self {
            root: root.into()
        }
    }

    #[throws]
    pub async fn build_programs(&self) {
        let success = Command::new("cargo")
            .arg("build-bpf")
            .spawn()?
            .wait()
            .await?
            .success();
        if !success {
            Err(Error::BuildProgramsFailed)?;
        }
    }

    #[throws]
    pub async fn start_localnet(&self) -> LocalnetHandle {
        let process = Command::new("solana-test-validator")
            .arg("-C")
            .arg([&self.root, "config.yml"].concat())
            .arg("-r")
            .arg("-q")
            .spawn()?;
        if !Client::new(Keypair::new()).is_localnet_running(true).await {
            Err(Error::LocalnetIsNotRunning)?
        }
        println!("localnet started");
        LocalnetHandle {
            solana_test_validator_process: process,
        }
    }
}
