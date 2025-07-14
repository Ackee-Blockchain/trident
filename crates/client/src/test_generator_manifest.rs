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
    pub(crate) async fn add_fuzz_target(&self, trident_tests_dir: &Path, new_fuzz_test: &str) {
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
