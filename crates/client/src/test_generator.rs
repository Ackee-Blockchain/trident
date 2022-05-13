use anyhow::Context;
use fehler::{throw, throws};
use std::{
    env, io,
    path::{Path, PathBuf},
};
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
    #[error("cannot parse Cargo.toml")]
    CannotParseCargoToml,
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("{0:?}")]
    Toml(#[from] toml::de::Error),
}

pub struct TestGenerator;
impl Default for TestGenerator {
    fn default() -> Self {
        Self::new()
    }
}
impl TestGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Initializes the `trdelnik-tests/tests` directory with all the necessary files. Adds the
    /// `test.rs` file and generates `Cargo.toml` with `dev-dependencies`. Updates root's `Cargo.toml`
    /// workspace members.
    ///
    /// The crate is generated from `trdelnik-tests` template located in `client/src/templates`.
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
    /// ```ignore
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
        let root = self.discover_root()?;
        self.generate_test_files(&root).await?;
        self.update_workspace(&root).await?;
    }

    /// Creates the `trdelnik-tests` workspace with `tests` directory and empty `test.rs` file
    /// finally it generates the `Cargo.toml` file. Crate is generated from `trdelnik-tests`
    /// template located in `client/src/templates`
    #[throws]
    async fn generate_test_files(&self, root: &PathBuf) {
        let workspace_path = Path::new(root).join(TESTS_WORKSPACE);
        self.create_directory(&workspace_path, TESTS_WORKSPACE)
            .await?;
        let tests_path = workspace_path.join(TESTS_DIRECTORY);
        self.create_directory(&tests_path, TESTS_DIRECTORY).await?;
        let test_path = tests_path.join(TESTS_FILE_NAME);
        let test_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/test.rs"
        ));
        self.create_file(&test_path, TESTS_FILE_NAME, test_content)
            .await?;
        let toml_path = workspace_path.join(CARGO_TOML);
        let toml_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/Cargo.toml"
        ));
        self.create_file(&toml_path, CARGO_TOML, toml_content)
            .await?;
    }

    /// Creates a new file with a given content on the specified `path` and `name`
    // todo: the function should be located in the different module, File module for example
    async fn create_file<'a>(
        &self,
        path: &'a PathBuf,
        name: &str,
        content: &str,
    ) -> Result<&'a PathBuf, Error> {
        match path.exists() {
            true => println!("Skipping creating the {} file", name),
            false => {
                println!("Creating the {} file ...", name);
                fs::write(path, content).await?;
            }
        };
        Ok(path)
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

    /// Tries to find the root directory with the `Anchor.toml` file.
    /// Throws an error when there is no directory with the `Anchor.toml` file
    // todo: this function should be a part of some Config / File implementation
    fn discover_root(&self) -> Result<PathBuf, Error> {
        let current_dir = env::current_dir()?;
        let mut dir = Some(current_dir.as_path());
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
    async fn update_workspace(&self, root: &PathBuf) {
        let cargo = Path::new(&root).join(CARGO_TOML);
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
}
