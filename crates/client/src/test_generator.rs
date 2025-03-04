use crate::commander::{Commander, Error as CommanderError};
use crate::constants::*;
use crate::{construct_path, utils::*};
use cargo_metadata::Package;
use fehler::throws;
use std::num::ParseIntError;
use std::path::StripPrefixError;
use std::{
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;
use trident_idl_spec::Idl;
use trident_template::Template;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot parse Cargo.toml")]
    CannotParseCargoToml,
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    StripPrefix(#[from] StripPrefixError),
    #[error("{0:?}")]
    TridentVersionsConfig(#[from] serde_json::Error),
    #[error("{0:?}")]
    ParseInt(#[from] ParseIntError),
    #[error("{0:?}")]
    Toml(#[from] toml::de::Error),
    #[error("{0:?}")]
    Commander(#[from] CommanderError),
    #[error("The Anchor project does not contain any programs")]
    NoProgramsFound,
    #[error("parsing Cargo.toml dependencies failed")]
    ParsingCargoTomlDependenciesFailed,
}

pub struct TestGenerator {
    pub root: PathBuf,
    pub program_packages: Vec<Package>,
    pub anchor_idls: Vec<Idl>,
    pub template: Template,
}
impl TestGenerator {
    #[throws]
    pub fn new_with_root(root: &str) -> Self {
        Self {
            root: Path::new(&root).to_path_buf(),
            program_packages: Vec::default(),
            anchor_idls: Vec::default(),
            template: Template::default(),
        }
    }
    #[throws]
    pub async fn initialize(&mut self, program_name: Option<String>, test_name: Option<String>) {
        Commander::build_anchor_project(program_name.clone()).await?;

        self.get_program_packages(program_name.clone()).await?;
        self.load_programs_idl(program_name.clone())?;
        self.create_template().await?;
        self.add_new_fuzz_test(test_name).await?;
        self.create_trident_toml().await?;

        self.update_gitignore(CARGO_TARGET_DIR_DEFAULT_HFUZZ)?;
        self.update_gitignore(CARGO_TARGET_DIR_DEFAULT_AFL)?;
    }

    #[throws]
    pub async fn add_fuzz_test(&mut self, program_name: Option<String>, test_name: Option<String>) {
        Commander::build_anchor_project(program_name.clone()).await?;

        self.get_program_packages(program_name.clone()).await?;
        self.load_programs_idl(program_name.clone())?;
        self.create_template().await?;
        self.add_new_fuzz_test(test_name).await?;

        self.update_gitignore(CARGO_TARGET_DIR_DEFAULT_HFUZZ)?;
        self.update_gitignore(CARGO_TARGET_DIR_DEFAULT_AFL)?;

        // update_package_metadata(&self.program_packages, &self.versions_config).await?;
    }

    #[throws]
    async fn get_program_packages(&mut self, program_name: Option<String>) {
        // TODO consider optionally excluding packages
        self.program_packages = collect_program_packages(program_name).await?;
    }

    #[throws]
    async fn create_template(&mut self) {
        // Obtain lib names so we can generate entries in the test_fuzz.rs file
        let lib_names = self
            .program_packages
            .iter()
            .map(|p| {
                // This is little dirty
                // We check if there is any target, if so we check only the first one and check if it is lib
                // if so we take its name.
                // Otherwise we take the package name.
                if !p.targets.is_empty() && p.targets[0].kind.iter().any(|k| k == "lib") {
                    p.targets[0].name.clone()
                } else {
                    p.name.clone()
                }
            })
            .collect::<Vec<String>>();

        // Older anchor idls didnt't contain program names so we parse them for backwards compatibility
        self.template.create_template(&self.anchor_idls, &lib_names);
    }

    #[throws]
    fn load_programs_idl(&mut self, program_name: Option<String>) {
        let target_path = construct_path!(self.root, "target/idl/");

        // TODO consider optionally excluding packages
        self.anchor_idls = crate::idl_loader::load_idls(target_path, program_name).unwrap();
    }
}
