use anyhow::Context;
use std::env::current_dir;
use fehler::{throw, throws};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use toml::{value::Table, Value};

const TESTS_WORKSPACE: &str = "trdelnik-tests";
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
    #[error("cannot parse Cargo.toml")]
    CannotParseCargoToml,
    #[error("{0:?}")]
    Io(#[from] std::io::Error),
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("{0:?}")]
    Toml(#[from] toml::de::Error),
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

    /// Initializes the `trdelnik-tests/tests` directory with all the necessary files. Adds the
    /// `test.rs` file and generates `Cargo.toml` with `dev-dependencies`. Updates root's `Cargo.toml`
    /// workspace members.
    ///
    /// Before you start writing trdelnik tests do not forget to add your program as a dependency
    /// to the `trdelnik-tests/Cargo.toml`. For example:
    ///
    /// ```toml
    /// # <project_root>/trdelnik-tests/Cargo.toml
    /// # ...
    /// [dev-dependencies]
    /// my-program = { path = "../programs/my-program" }
    /// # ...
    /// ```
    ///
    /// Then you can easily use it in tests:
    ///
    /// ```rust
    /// use my_program;
    ///
    /// // ...
    ///
    /// #[trdelnik_test]
    /// async fn test() {
    ///     // ...
    ///     my_program::do_something(/*...*/);
    ///     // ...
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - there is not a root directory (no `Anchor.toml` file)
    #[throws]
    pub async fn generate(&self) {
        self.check_workspace().await?;
        self.generate_test_files().await?;
        self.update_workspace().await?;
    }

    /// Creates the `trdelnik-tests` workspace with `tests` directory and empty `test.rs` file
    /// finally it generates the `Cargo.toml` file.
    #[throws]
    async fn generate_test_files(&self) {
        let workspace_path = Path::new(&self.root).join(TESTS_WORKSPACE);
        self.create_directory(&workspace_path, TESTS_WORKSPACE)
            .await?;
        let tests_path = workspace_path.join(TESTS_DIRECTORY);
        self.create_directory(&tests_path, TESTS_DIRECTORY).await?;
        let test_path = tests_path.join(TESTS_FILE_NAME);
        match test_path.exists() {
            true => println!("Skipping creating the {} file", TESTS_FILE_NAME),
            false => {
                println!("Creating the {} file ...", TESTS_FILE_NAME);
                fs::write(test_path, "").await?;
            }
        };
        self.initialize_cargo_toml().await?;
    }

    /// Creates a new directory on the specified `path` and with the specified `name`
    // todo: the function should be located in the different module, File module for example
    async fn create_directory<'a>(
        &self,
        path: &'a PathBuf,
        name: &str,
    ) -> Result<&'a PathBuf, Error> {
        match path.exists() {
            true => println!("Skipping creating the {} directory", name),
            false => {
                println!("Creating the {} directory ...", name);
                fs::create_dir(path).await?;
            }
        };
        Ok(path)
    }

    /// Creates and initializes the Cargo.toml. Adds `dev-dependencies` for the tests runner.
    #[throws]
    async fn initialize_cargo_toml(&self) {
        let cargo_toml = Path::new(&self.root).join(TESTS_WORKSPACE).join(CARGO_TOML);
        if cargo_toml.exists() {
            println!("Skipping creating the {} file", CARGO_TOML);
            return;
        }
        println!("Creating the {} file ...", CARGO_TOML);
        // todo: the `trdelnik-client` path should be changed to crate version after the release
        let toml = r#"[package]
name = "trdelnik-tests"
version = "0.1.0"
description = "Created with Trdelnik"
edition = "2021"

[dev-dependencies]
fehler = "1.0.0"
rstest = "0.12.0"
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

    /// Adds `trdelnik-tests` workspace to the `root`'s `Cargo.toml` workspace members if needed.
    #[throws]
    async fn update_workspace(&self) {
        let cargo = Path::new(&self.root).join(CARGO_TOML);
        let mut content: Value = fs::read_to_string(&cargo).await?.parse()?;
        let test_workspace_value = Value::String(String::from(TESTS_WORKSPACE));
        let members = content
            .as_table_mut()
            .ok_or(Error::CannotParseCargoToml)?
            .entry("workspace")
            .or_insert(Value::Table(Table::default()))
            .as_table_mut()
            .ok_or(Error::CannotParseCargoToml)?
            .entry("members")
            .or_insert(Value::Array(vec![test_workspace_value.clone()]))
            .as_array_mut()
            .ok_or(Error::CannotParseCargoToml)?;
        match members.iter().find(|&x| x.eq(&test_workspace_value)) {
            Some(_) => println!("Skipping updating project workspace"),
            None => {
                members.push(test_workspace_value);
                println!("Project workspace successfully updated");
            }
        }
        fs::write(cargo, content.to_string()).await?;
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
