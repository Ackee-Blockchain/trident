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
    root: PathBuf,
}

impl TestGenerator {
    pub fn new() -> Self {
        Self {
            root: current_dir().unwrap(),
        }
    }

    /// Initializes the the `tests` directory with all the necessary files. Adds the `test.rs` file
    /// and generates `Cargo.toml` with `dev-dependencies`
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - the directory is not the root directory (does not contain the `Anchor.toml` file)
    #[throws]
    pub async fn generate(&self) {
        self.check_workspace()?;
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

    /// Checks if the command is called from the `root` directory
    /// The `root` directory is your program workspace - the place where `Anchor.toml` file is located
    fn check_workspace(&self) -> Result<(), Error> {
        let anchor_toml = Path::new(&self.root).join(ANCHOR_TOML);
        match anchor_toml.exists() {
            false => throw!(Error::BadWorkspace),
            _ => Ok(())
        }
    }
}
