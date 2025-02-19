use crate::___private::TestGenerator;
use crate::commander::Commander;
use crate::constants::*;
use crate::{construct_path, load_template, utils::*};
use fehler::throws;

use std::path::Path;

use crate::test_generator::Error;

impl TestGenerator {
    #[throws]
    pub(crate) async fn create_instructions(&self, fuzz_test_dir: &Path) {
        let instructions = construct_path!(fuzz_test_dir, INSTRUCTIONS_DIRECTORY);
        let instructions_mod_path = construct_path!(instructions, "mod.rs");
        create_directory_all(&instructions).await?;

        let instructions_source_codes = self.template.get_instructions();

        for (mut name, source_code) in instructions_source_codes {
            let source_code = Commander::format_program_code_nightly(&source_code).await?;
            name.push_str(".rs");
            let instruction_path = instructions.join(&name);
            create_file(&self.root, &instruction_path, &source_code).await?;
        }

        let instructions_mod = self.template.get_instructions_mod();
        let instructions_mod = Commander::format_program_code_nightly(&instructions_mod).await?;
        create_file(&self.root, &instructions_mod_path, &instructions_mod).await?;
    }

    #[throws]
    pub(crate) async fn create_transactions(&self, fuzz_test_dir: &Path) {
        let transactions = construct_path!(fuzz_test_dir, TRANSACTIONS_DIRECTORY);
        let transactions_mod_path = construct_path!(transactions, "mod.rs");
        create_directory_all(&transactions).await?;

        let transactions_source_codes = self.template.get_transactions();

        for (mut name, source_code) in transactions_source_codes {
            let source_code = Commander::format_program_code_nightly(&source_code).await?;
            name.push_str(".rs");
            let transaction_path = transactions.join(&name);
            create_file(&self.root, &transaction_path, &source_code).await?;
        }

        let transactions_mod = self.template.get_transactions_mod();
        let transactions_mod = Commander::format_program_code_nightly(&transactions_mod).await?;
        create_file(&self.root, &transactions_mod_path, &transactions_mod).await?;
    }

    #[throws]
    pub(crate) async fn create_test_fuzz(&self, fuzz_test_dir: &Path) {
        let test_fuzz = self.template.get_test_fuzz();
        let test_fuzz = Commander::format_program_code_nightly(&test_fuzz).await?;
        let test_fuzz_path = construct_path!(fuzz_test_dir, FUZZ_TEST);

        create_file(&self.root, &test_fuzz_path, &test_fuzz).await?;
    }

    #[throws]
    pub(crate) async fn create_custom_types(&self, fuzz_test_dir: &Path) {
        let custom_types = self.template.get_custom_types();
        let custom_types = Commander::format_program_code_nightly(&custom_types).await?;
        let custom_types_path = construct_path!(fuzz_test_dir, TYPES_FILE_NAME);
        create_file(&self.root, &custom_types_path, &custom_types).await?;
    }

    #[throws]
    pub(crate) async fn create_fuzz_transactions(&self, fuzz_test_dir: &Path) {
        let fuzz_transactions = self.template.get_fuzz_transactions();
        let fuzz_transactions = Commander::format_program_code_nightly(&fuzz_transactions).await?;
        let fuzz_transactions_path = construct_path!(fuzz_test_dir, FUZZ_TRANSACTIONS_FILE_NAME);

        create_file(&self.root, &fuzz_transactions_path, &fuzz_transactions).await?;
    }

    #[throws]
    pub(crate) async fn create_cargo_toml(&self, trident_tests: &Path) {
        let cargo_toml_content = load_template!("/src/template/Cargo_fuzz.toml.tmpl");

        let cargo_toml_path = construct_path!(trident_tests, CARGO_TOML);

        create_file(&self.root, &cargo_toml_path, cargo_toml_content).await?;
    }

    #[throws]
    pub(crate) async fn create_trident_toml(&self) {
        let trident_toml_content = load_template!("/src/template/Trident.toml.tmpl");
        let trident_toml_path = construct_path!(self.root, TRIDENT_TOML);

        create_file(&self.root, &trident_toml_path, trident_toml_content).await?;
    }

    #[throws]
    pub(crate) async fn add_new_fuzz_test(&self, test_name: Option<String>) {
        let trident_tests = construct_path!(self.root, TESTS_WORKSPACE_DIRECTORY);

        let new_fuzz_test = match test_name {
            Some(name) => name,
            None => format!("fuzz_{}", get_fuzz_id(&trident_tests)?),
        };

        let new_fuzz_test_dir = construct_path!(trident_tests, &new_fuzz_test);

        if new_fuzz_test_dir.exists() {
            println!("{SKIP} [{}] already exists", new_fuzz_test_dir.display());
            return;
        }

        self.create_instructions(&new_fuzz_test_dir).await?;
        self.create_transactions(&new_fuzz_test_dir).await?;
        self.create_test_fuzz(&new_fuzz_test_dir).await?;
        self.create_custom_types(&new_fuzz_test_dir).await?;
        self.create_fuzz_transactions(&new_fuzz_test_dir).await?;
        self.create_cargo_toml(&trident_tests).await?;

        self.trident_dependency(&trident_tests).await?;
        self.program_dependency(&trident_tests).await?;
        self.fuzz_target(&trident_tests, &new_fuzz_test).await?

        // add_workspace_member(&self.root, &format!("{TESTS_WORKSPACE_DIRECTORY}",)).await?;
    }
}
