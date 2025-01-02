use crate::___private::test_fuzz_generator;
use crate::commander::{Commander, Error as CommanderError};
use crate::constants::*;
use crate::source_code_generators::fuzz_instructions_generator;
use crate::versions_config::TridentVersionsConfig;
use crate::{construct_path, load_template, utils::*};
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
    pub test_fuzz: String,
    pub fuzz_instructions: String,
    pub versions_config: TridentVersionsConfig,
}
impl TestGenerator {
    #[throws]
    pub fn new_with_root(root: &str) -> Self {
        let versions_config: TridentVersionsConfig =
            serde_json::from_str(load_template!("/config.json"))?;

        Self {
            root: Path::new(&root).to_path_buf(),
            program_packages: Vec::default(),
            anchor_idls: Vec::default(),
            fuzz_instructions: String::default(),
            test_fuzz: String::default(),
            versions_config,
        }
    }
    #[throws]
    pub async fn initialize(&mut self) {
        Commander::build_anchor_project().await?;

        self.get_program_packages().await?;
        self.load_programs_idl()?;
        self.generate_source_codes().await?;
        self.initialize_new_fuzz_test().await?;

        update_gitignore(&self.root, CARGO_TARGET_DIR_DEFAULT_HFUZZ)?;
        update_gitignore(&self.root, CARGO_TARGET_DIR_DEFAULT_AFL)?;
    }

    #[throws]
    pub async fn add_fuzz_test(&mut self) {
        Commander::build_anchor_project().await?;

        self.get_program_packages().await?;
        self.load_programs_idl()?;
        self.generate_source_codes().await?;
        self.add_new_fuzz_test().await?;

        // update_package_metadata(&self.program_packages, &self.versions_config).await?;
    }

    #[throws]
    async fn get_program_packages(&mut self) {
        // TODO consider optionally excluding packages
        self.program_packages = collect_program_packages().await?;
    }

    #[throws]
    async fn generate_source_codes(&mut self) {
        let test_fuzz = test_fuzz_generator::generate_source_code(&self.anchor_idls);
        let fuzz_instructions =
            fuzz_instructions_generator::generate_source_code(&self.anchor_idls);

        self.test_fuzz = Commander::format_program_code_nightly(&test_fuzz).await?;
        self.fuzz_instructions = Commander::format_program_code_nightly(&fuzz_instructions).await?;
    }

    #[throws]
    fn load_programs_idl(&mut self) {
        let target_path = construct_path!(self.root, "target/idl/");

        // TODO consider optionally excluding packages
        self.anchor_idls = crate::idl_loader::load_idls(target_path).unwrap();
    }

    #[throws]
    pub async fn add_new_fuzz_test(&self) {
        let fuzz_dir_path = construct_path!(self.root, TESTS_WORKSPACE_DIRECTORY);
        let fuzz_tests_manifest_path = construct_path!(fuzz_dir_path, CARGO_TOML);

        create_directory_all(&fuzz_dir_path).await?;

        let fuzz_id = get_fuzz_id(&fuzz_dir_path)?;
        let new_fuzz_test = format!("fuzz_{fuzz_id}");
        let new_fuzz_test_dir = fuzz_dir_path.join(&new_fuzz_test);
        let new_bin_target = format!("{new_fuzz_test}/test_fuzz.rs");

        create_directory(&new_fuzz_test_dir).await?;

        let fuzz_test_path = new_fuzz_test_dir.join(FUZZ_TEST);
        let fuzz_instructions_path = new_fuzz_test_dir.join(FUZZ_INSTRUCTIONS_FILE_NAME);

        let cargo_toml_content = load_template!("/src/template/Cargo_fuzz.toml.tmpl");

        create_file(&self.root, &fuzz_test_path, &self.test_fuzz).await?;
        create_file(&self.root, &fuzz_instructions_path, &self.fuzz_instructions).await?;
        create_file(&self.root, &fuzz_tests_manifest_path, cargo_toml_content).await?;

        add_bin_target(&fuzz_tests_manifest_path, &new_fuzz_test, &new_bin_target).await?;

        update_fuzz_tests_manifest(
            &self.versions_config,
            &self.program_packages,
            &fuzz_dir_path,
        )
        .await?;

        // add_workspace_member(&self.root, &format!("{TESTS_WORKSPACE_DIRECTORY}",)).await?;
    }
    #[throws]
    pub async fn initialize_new_fuzz_test(&self) {
        let fuzz_dir_path = construct_path!(self.root, TESTS_WORKSPACE_DIRECTORY);
        let fuzz_tests_manifest_path = construct_path!(fuzz_dir_path, CARGO_TOML);
        let trident_toml_path = construct_path!(self.root, TRIDENT_TOML);

        create_directory_all(&fuzz_dir_path).await?;

        let fuzz_id = get_fuzz_id(&fuzz_dir_path)?;
        let new_fuzz_test = format!("fuzz_{fuzz_id}");
        let new_fuzz_test_dir = fuzz_dir_path.join(&new_fuzz_test);
        let new_bin_target = format!("{new_fuzz_test}/test_fuzz.rs");

        create_directory(&new_fuzz_test_dir).await?;

        let fuzz_test_path = new_fuzz_test_dir.join(FUZZ_TEST);
        let fuzz_instructions_path = new_fuzz_test_dir.join(FUZZ_INSTRUCTIONS_FILE_NAME);

        let cargo_toml_content = load_template!("/src/template/Cargo_fuzz.toml.tmpl");

        let trident_toml_content = load_template!("/../config/template/Trident.toml.tmpl");

        create_file(&self.root, &fuzz_test_path, &self.test_fuzz).await?;
        create_file(&self.root, &fuzz_instructions_path, &self.fuzz_instructions).await?;
        create_file(&self.root, &fuzz_tests_manifest_path, cargo_toml_content).await?;
        create_file(&self.root, &trident_toml_path, trident_toml_content).await?;

        add_bin_target(&fuzz_tests_manifest_path, &new_fuzz_test, &new_bin_target).await?;
        initialize_fuzz_tests_manifest(
            &self.versions_config,
            &self.program_packages,
            &fuzz_dir_path,
        )
        .await?;

        // add_workspace_member(
        //     &self.root,
        //     &format!("{TESTS_WORKSPACE_DIRECTORY}/{FUZZ_TEST_DIRECTORY}",),
        // )
        // .await?;
    }
}
