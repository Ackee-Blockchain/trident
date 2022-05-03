use anyhow::Context;
use fehler::{throw, throws};
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
    #[error("must have current dir")]
    MustHaveCurrentDir,
    #[error("{0:?}")]
    Io(#[from] std::io::Error),
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
}

pub struct TestGenerator {
    root: PathBuf,
}

impl TestGenerator {
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap(),
        }
    }

    /// Initializes the the `tests` directory with all the necessary files. Adds the `test.rs` file
    /// and generates `Cargo.toml` with `dev-dependencies`
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - there is not a root directory (no `Anchor.toml` file)
    #[throws]
    pub async fn generate(&mut self) {
        self.root = self.discover_root()?;
        self.generate_test_files().await?;
    }

    /// Creates the `test` folder in the `root` directory, adds the empty `test.rs` file and
    /// generates the `Cargo.toml` file.
    #[throws]
    async fn generate_test_files(&self) {
        let path = Path::new(&self.root).join(TESTS_DIRECTORY);
        if !path.exists() {
            fs::create_dir(&path).await?;
        }
        let test_path = path.join(TESTS_FILE_NAME);
        if !test_path.exists() {
            fs::write(test_path, "").await?;
        }
        self.initialize_cargo_toml().await?;
    }

    /// Creates and initializes the Cargo.toml. Adds `dev-dependencies` for the tests runner.
    #[throws]
    async fn initialize_cargo_toml(&self) {
        let cargo_toml = Path::new(&self.root).join(TESTS_DIRECTORY).join(CARGO_TOML);
        if cargo_toml.exists() {
            return;
        }
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

    /// Tries to find the root directory with the `Anchor.toml` file.
    /// Throws an error when there is no directory with the `Anchor.toml` file
    // todo: this function should be a part of some Config / File implementation
    fn discover_root(&self) -> Result<PathBuf, Error> {
        let mut dir = Some(self.root.as_path());
        while let Some(cwd) = dir {
            for file in std::fs::read_dir(cwd).with_context(|| {
                format!("Error reading the directory with path: {}", cwd.display())
            })? {
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
}
