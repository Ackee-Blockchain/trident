use crate::{commander::Error as CommanderError, config::Config};
use fehler::{throw, throws};
use std::{
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tokio::fs;
use toml::{value::Table, Value};

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot parse Cargo.toml")]
    CannotParseCargoToml,
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    Toml(#[from] toml::de::Error),
    #[error("{0:?}")]
    Commander(#[from] CommanderError),
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
    pub async fn clean_full(&self) {
        // TODO this will look for parent dir with Anchor.toml,
        // so command does not have to be called from project root
        let root = match Config::discover_root() {
            Ok(root) => root,
            Err(_) => throw!(Error::BadWorkspace),
        };
        self.remove_program_client_folder(&root).await?;
        self.remove_trdelnik_tests_folder(&root).await?;
        self.remove_from_workspace(&root).await?;
        self.remove_manifest_files(&root).await?;
    }
    #[throws]
    async fn remove_program_client_folder(&self, root: &PathBuf) {
        let program_client_path = Path::new(root).join(crate::commander::PROGRAM_CLIENT_DIRECTORY);
        if program_client_path.try_exists().unwrap() {
            fs::remove_dir_all(program_client_path).await?;
            println!(
                "Removed {} directory.",
                crate::commander::PROGRAM_CLIENT_DIRECTORY
            );
        } else {
            println!(
                "Directory {} does not exists.",
                crate::commander::PROGRAM_CLIENT_DIRECTORY
            );
        }
    }
    #[throws]
    async fn remove_trdelnik_tests_folder(&self, root: &PathBuf) {
        let trdelnik_tests_path = Path::new(root).join(crate::test_generator::TESTS_WORKSPACE);
        if trdelnik_tests_path.try_exists().unwrap() {
            fs::remove_dir_all(trdelnik_tests_path).await?;
            println!(
                "Removed {} directory.",
                crate::test_generator::TESTS_WORKSPACE
            );
        } else {
            println!(
                "Directory {} does not exists.",
                crate::test_generator::TESTS_WORKSPACE
            );
        }
    }
    #[throws]
    async fn remove_from_workspace(&self, root: &PathBuf) {
        let cargo = Path::new(root).join(crate::config::CARGO_TOML);
        let mut content: Value = fs::read_to_string(&cargo).await?.parse()?;
        let test_workspace_value =
            Value::String(String::from(crate::test_generator::TESTS_WORKSPACE));

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

        let values_count_before = members.len();

        members.retain(|x| x != &test_workspace_value);

        let values_count_after = members.len();

        if values_count_before == values_count_after {
            println!(
                "{} not found in {} file.",
                crate::test_generator::TESTS_WORKSPACE,
                crate::config::CARGO_TOML
            );
        } else {
            println!(
                "{} deleted from {} file.",
                crate::test_generator::TESTS_WORKSPACE,
                crate::config::CARGO_TOML
            );
        }
        fs::write(cargo, content.to_string()).await?;
    }
    #[throws]
    async fn remove_manifest_files(&self, root: &PathBuf) {
        let trdelnik_toml_path = Path::new(root).join(crate::config::TRDELNIK_TOML);
        if trdelnik_toml_path.try_exists().unwrap() {
            fs::remove_file(trdelnik_toml_path).await?;
            println!("Removed {} file.", crate::config::TRDELNIK_TOML);
        } else {
            println!("File {} does not exists.", crate::config::TRDELNIK_TOML);
        }
    }
}
