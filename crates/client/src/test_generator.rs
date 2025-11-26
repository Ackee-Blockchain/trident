use crate::commander::Commander;
use crate::construct_path;
use crate::error::Error;
use fehler::throws;
use std::path::Path;
use std::path::PathBuf;
use trident_idl_spec::Idl;
use trident_template::GeneratedFiles;
use trident_template::TridentTemplates;

pub struct TestGenerator {
    pub(crate) root: PathBuf,
    pub(crate) skip_build: bool,
    pub(crate) anchor_idls: Vec<Idl>,
    pub(crate) template_engine: TridentTemplates,
    pub(crate) generated_files: Option<GeneratedFiles>,
}

impl TestGenerator {
    #[throws]
    pub fn new_with_root(root: &str, skip_build: bool) -> Self {
        Self {
            root: Path::new(&root).to_path_buf(),
            skip_build,
            anchor_idls: Vec::default(),
            template_engine: TridentTemplates::new()?,
            generated_files: None,
        }
    }

    #[throws]
    pub async fn initialize(&mut self, program_name: Option<String>, test_name: Option<String>) {
        if !self.skip_build {
            Commander::build_anchor_project(&self.root, program_name.clone()).await?;
        }

        self.load_programs_idl(program_name.clone())?;
        self.create_template().await?;
        self.add_new_fuzz_test(&test_name).await?;
        self.create_trident_toml().await?;
        self.create_vscode_settings().await?;
    }

    #[throws]
    pub async fn add_fuzz_test(&mut self, program_name: Option<String>, test_name: Option<String>) {
        if !self.skip_build {
            Commander::build_anchor_project(&self.root, program_name.clone()).await?;
        }

        self.load_programs_idl(program_name.clone())?;

        self.create_template().await?;

        self.add_new_fuzz_test(&test_name).await?;
    }

    #[throws]
    pub async fn refresh_fuzz_test(
        &mut self,
        fuzz_test_name: String,
        program_name: Option<String>,
    ) {
        if !self.skip_build {
            Commander::build_anchor_project(&self.root, program_name.clone()).await?;
        }

        self.load_programs_idl(program_name)?;
        self.create_template().await?;
        self.refresh_types_file(&fuzz_test_name).await?;
    }

    #[throws]
    async fn create_template(&mut self) {
        let current_package_version = env!("CARGO_PKG_VERSION");

        // Generate templates using Tera
        let output = self
            .template_engine
            .generate(&self.anchor_idls, current_package_version)?;

        // Store the generated output
        self.generated_files = Some(output);
    }

    #[throws]
    fn load_programs_idl(&mut self, program_name: Option<String>) {
        let target_path = construct_path!(self.root, "target/idl/");

        // TODO consider optionally excluding packages
        self.anchor_idls = crate::idl_loader::load_idls(target_path, program_name)?;
    }

    pub(crate) fn get_test_fuzz(&self) -> String {
        if let Some(ref output) = self.generated_files {
            output.test_fuzz.clone()
        } else {
            String::new()
        }
    }

    pub(crate) fn get_types(&self) -> String {
        if let Some(ref output) = self.generated_files {
            output.types.clone()
        } else {
            String::new()
        }
    }

    pub(crate) fn get_fuzz_accounts(&self) -> String {
        if let Some(ref output) = self.generated_files {
            output.fuzz_accounts.clone()
        } else {
            String::new()
        }
    }

    pub(crate) fn get_trident_toml(&self) -> String {
        if let Some(ref output) = self.generated_files {
            output.trident_toml.clone()
        } else {
            String::new()
        }
    }

    pub(crate) fn get_cargo_fuzz_toml(&self) -> String {
        if let Some(ref output) = self.generated_files {
            output.cargo_fuzz_toml.clone()
        } else {
            String::new()
        }
    }
}
