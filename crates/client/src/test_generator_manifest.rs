use crate::___private::TestGenerator;
use crate::constants::*;
use crate::construct_path;
use fehler::throws;

use std::path::Path;

use std::fs;
use toml::{Table, Value};

use crate::test_generator::Error;

impl TestGenerator {
    #[throws]
    pub(crate) async fn trident_dependency(&self, trident_tests_dir: &Path) {
        let cargo_path = construct_path!(trident_tests_dir, CARGO_TOML);

        let cargo_toml_content = fs::read_to_string(&cargo_path)?;

        let mut cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

        // Ensure the required dependencies are present in the 'dependencies' section.
        let dependencies_table = ensure_table(&mut cargo_toml, "dependencies")?;

        let current_package_version = env!("CARGO_PKG_VERSION");
        // Add 'trident-fuzz' dependency in table format.
        dependencies_table.entry("trident-fuzz").or_insert_with(|| {
            let mut trident_client = toml::Table::new();
            trident_client.insert(
                "version".to_string(),
                Value::String(current_package_version.to_string()),
            );
            Value::Table(trident_client)
        });

        fs::write(cargo_path, toml::to_string(&cargo_toml).unwrap())?;
    }

    #[throws]
    pub(crate) async fn program_dependency(&self, trident_tests_dir: &Path) {
        let cargo_path = construct_path!(trident_tests_dir, CARGO_TOML);

        let cargo_toml_content = fs::read_to_string(&cargo_path)?;
        let mut cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

        // Ensure the required dependencies are present in the 'dependencies' section.
        let dependencies_table = ensure_table(&mut cargo_toml, "dependencies")?;

        for package in &self.program_packages {
            let manifest_path = package.manifest_path.parent().unwrap().as_std_path();
            let relative_path = pathdiff::diff_paths(manifest_path, trident_tests_dir).unwrap();

            let relative_path_str = relative_path.to_str().unwrap_or_default();

            dependencies_table.entry(&package.name).or_insert_with(|| {
                let mut package_entry = toml::Table::new();
                package_entry.insert(
                    "path".to_string(),
                    Value::String(relative_path_str.to_owned()),
                );
                Value::Table(package_entry)
            });
        }

        fs::write(cargo_path, toml::to_string(&cargo_toml).unwrap())?;
    }

    #[throws]
    pub(crate) async fn fuzz_target(&self, trident_tests_dir: &Path, new_fuzz_test: &str) {
        let cargo_path = construct_path!(trident_tests_dir, CARGO_TOML);

        // Read the existing Cargo.toml file
        let cargo_toml_content = fs::read_to_string(&cargo_path)?;
        let mut cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

        // Create a new bin table
        let mut bin_table = Table::new();
        bin_table.insert("name".to_string(), Value::String(new_fuzz_test.to_string()));
        bin_table.insert(
            "path".to_string(),
            Value::String(format!("{new_fuzz_test}/{FUZZ_TEST}").to_string()),
        );

        // Add the new [[bin]] section to the [[bin]] array
        if let Some(bin_array) = cargo_toml.get_mut("bin") {
            if let Value::Array(bin_array) = bin_array {
                bin_array.push(Value::Table(bin_table));
            } else {
                // If "bin" exists but is not an array, replace it with an array
                let bin_array = vec![Value::Table(bin_table)];
                cargo_toml
                    .as_table_mut()
                    .unwrap()
                    .insert("bin".to_string(), Value::Array(bin_array));
            }
        } else {
            // If there is no existing [[bin]] array, create one
            let bin_array = vec![Value::Table(bin_table)];
            cargo_toml
                .as_table_mut()
                .unwrap()
                .insert("bin".to_string(), Value::Array(bin_array));
        }

        // Write the updated Cargo.toml file
        let updated_toml = toml::to_string(&cargo_toml).unwrap();
        fs::write(cargo_path, updated_toml)?;
    }
}

/// Ensures that a table exists in the given TOML content, and returns a mutable reference to it.
pub fn ensure_table<'a>(content: &'a mut Value, table_name: &str) -> Result<&'a mut Table, Error> {
    content
        .as_table_mut()
        .ok_or(Error::ParsingCargoTomlDependenciesFailed)?
        .entry(table_name)
        .or_insert(Value::Table(toml::Table::new()))
        .as_table_mut()
        .ok_or(Error::ParsingCargoTomlDependenciesFailed)
}
