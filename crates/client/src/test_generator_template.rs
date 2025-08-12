use crate::___private::TestGenerator;
use crate::commander::Commander;
use crate::constants::*;
use crate::construct_path;
use crate::utils::*;
use fehler::throws;

use std::path::Path;

use crate::error::Error;

impl TestGenerator {
    #[throws]
    pub(crate) async fn create_test_fuzz(&self, fuzz_test_dir: &Path) {
        let test_fuzz = self.get_test_fuzz();
        let test_fuzz = Commander::format_program_code_nightly(&test_fuzz).await?;
        let test_fuzz_path = construct_path!(fuzz_test_dir, FUZZ_TEST);

        create_file(&self.root, &test_fuzz_path, &test_fuzz).await?;
    }

    #[throws]
    pub(crate) async fn create_types(&self, fuzz_test_dir: &Path) {
        let types = self.get_types();
        let types = Commander::format_program_code_nightly(&types).await?;
        let types_path = construct_path!(fuzz_test_dir, TYPES_FILE_NAME);
        create_file(&self.root, &types_path, &types).await?;
    }

    #[throws]
    pub(crate) async fn create_fuzz_accounts(&self, fuzz_test_dir: &Path) {
        let fuzz_accounts = self.get_fuzz_accounts();
        let fuzz_accounts = Commander::format_program_code_nightly(&fuzz_accounts).await?;
        let fuzz_accounts_path = construct_path!(fuzz_test_dir, FUZZ_ACCOUNTS_FILE_NAME);

        create_file(&self.root, &fuzz_accounts_path, &fuzz_accounts).await?;
    }

    #[throws]
    pub(crate) async fn create_cargo_toml(&self, trident_tests: &Path, test_name: &str) {
        // Check if Cargo.toml already exists
        let cargo_toml_path = construct_path!(trident_tests, CARGO_TOML);

        if cargo_toml_path.exists() {
            self.add_fuzz_target(trident_tests, test_name).await?;
        } else {
            // If it doesn't exist, let the template crate generate it
            let cargo_toml_content = self.get_cargo_fuzz_toml();
            create_file(&self.root, &cargo_toml_path, &cargo_toml_content).await?;
            self.add_fuzz_target(trident_tests, test_name).await?;
        }
    }

    #[throws]
    pub(crate) async fn create_trident_toml(&self) {
        let trident_toml_content = self.get_trident_toml();
        let trident_toml_path = construct_path!(self.root, TESTS_WORKSPACE_DIRECTORY, TRIDENT_TOML);

        create_file(&self.root, &trident_toml_path, &trident_toml_content).await?;
    }

    #[throws]
    pub(crate) async fn refresh_types_file(&self, fuzz_test_name: &str) {
        let fuzz_test_dir = construct_path!(self.root, TESTS_WORKSPACE_DIRECTORY, fuzz_test_name);
        let types_path = construct_path!(fuzz_test_dir, TYPES_FILE_NAME);

        let types = self.get_types();
        let types = Commander::format_program_code_nightly(&types).await?;

        // Write the file (this will overwrite if it exists)
        std::fs::write(&types_path, &types)?;

        let relative_path = types_path
            .strip_prefix(&self.root)?
            .to_str()
            .unwrap_or_default();
        println!("{FINISH} [{relative_path}] refreshed");
    }

    #[throws]
    pub(crate) async fn add_new_fuzz_test(&self, test_name: &Option<String>) {
        let trident_tests = construct_path!(self.root, TESTS_WORKSPACE_DIRECTORY);

        let new_fuzz_test = match test_name {
            Some(name) => name.to_owned(),
            None => format!("fuzz_{}", get_fuzz_id(&trident_tests)?),
        };

        let new_fuzz_test_dir = construct_path!(trident_tests, &new_fuzz_test);

        if new_fuzz_test_dir.exists() {
            println!("{SKIP} [{}] already exists", new_fuzz_test_dir.display());
            return;
        }

        create_directory_all(&new_fuzz_test_dir).await?;
        self.create_test_fuzz(&new_fuzz_test_dir).await?;
        self.create_types(&new_fuzz_test_dir).await?;
        self.create_fuzz_accounts(&new_fuzz_test_dir).await?;
        self.create_cargo_toml(&trident_tests, &new_fuzz_test)
            .await?;
    }
}
