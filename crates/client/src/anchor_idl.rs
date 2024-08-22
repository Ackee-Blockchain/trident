use anchor_lang_idl_spec::Idl;
use cargo_metadata::Package;
use convert_case::{Case, Casing};

use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

pub fn load_idls(
    dir_path: PathBuf,
    program_packages: &[Package],
) -> Result<Vec<Idl>, Box<dyn Error>> {
    let mut idls = Vec::new();

    let package_names: Vec<String> = program_packages
        .iter()
        .map(|package| {
            let name = &package.name;
            name.to_case(Case::Snake)
        })
        .collect();

    // Read the directory and iterate over each entry
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        let idl_name = entry.file_name();
        let idl_name_str = idl_name.to_str().unwrap();

        // Only process .json files
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            // Remove the .json extension to get the package name
            let package_name = idl_name_str.trim_end_matches(".json");

            // Check if the package name is in the list of known packages
            if package_names.iter().any(|name| name == package_name) {
                // Open the file in read-only mode
                let mut file = File::open(&path)?;

                // Read the file contents into a string
                let mut json_content = String::new();
                file.read_to_string(&mut json_content)?;

                // Parse the string of data into an Idl struct
                match serde_json::from_str::<Idl>(&json_content) {
                    Ok(parsed_idl) => {
                        idls.push(parsed_idl);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse {}: {}", path.display(), e);
                        // Continue to the next file on failure
                    }
                }
            }
        }
    }

    Ok(idls)
}