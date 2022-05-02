use fehler::{throw, throws};
use std::env::current_dir;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;

const TESTS_DIRECTORY: &str = "tests";
const TESTS_FILE_NAME: &str = "test.rs";
const CARGO_TOML: &str = "Cargo.toml";
const ANCHOR_TOML: &str = "Anchor.toml";

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid workspace")]
    BadWorkspace,
    #[error("{0:?}")]
    Io(#[from] std::io::Error),
}

pub struct TestGenerator {
    path: PathBuf,
}

impl TestGenerator {
    pub fn new() -> Self {
        Self {
            path: current_dir().unwrap(),
        }
    }

    #[throws]
    pub async fn generate(&self) {
        self.check_workspace().await?;
        self.generate_test_files().await?;
    }

    #[throws]
    async fn generate_test_files(&self) {
        let path = Path::new(&self.path).join(TESTS_DIRECTORY);
        if fs::metadata(&path).await.is_ok() {
            return;
        }
        fs::create_dir(&path).await?;
        fs::write(path.join(TESTS_FILE_NAME), "").await?;
        self.initialize_cargo_toml().await?;
    }

    #[throws]
    async fn initialize_cargo_toml(&self) {
        let cargo_toml = Path::new(&self.path).join(TESTS_DIRECTORY).join(CARGO_TOML);
        // todo: the `trdelnik-client` path should be changed to crate version after the release
        let toml = r#"[package]
name = "tests"
version = "0.1.0"
description = "Created with Trdelnik"
edition = "2021"

[dev-dependencies]
fehler = "1.0.0"
trdelnik-client = { path = "../../../crates/client" }
program_client = { path = "../program_client" }
"#;
        fs::write(cargo_toml, toml).await?;
    }

    #[throws]
    async fn check_workspace(&self) {
        let anchor_toml = Path::new(&self.path).join(ANCHOR_TOML);
        match anchor_toml.exists() {
            false => throw!(Error::BadWorkspace),
            _ => {}
        }
    }
}
