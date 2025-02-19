use heck::ToSnakeCase;
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

use trident_idl_spec::Idl;

pub fn load_idls(
    dir_path: PathBuf,
    program_name: Option<String>,
) -> Result<Vec<Idl>, Box<dyn Error>> {
    let mut idls = Vec::new();

    // Read the directory and iterate over each entry
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(ref program_name) = program_name {
            if path.is_file()
                && !path
                    .file_name()
                    .and_then(|name| name.to_str())
                    // convert program_name to match case of IDL names
                    .map(|name| name.trim_end_matches(".json") == program_name.to_snake_case())
                    .unwrap_or(false)
            {
                continue;
            }
        }

        // Only process .json files
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            // Remove the .json extension to get the package name
            // let package_name = idl_name_str.trim_end_matches(".json");

            // Check if the package name is in the list of known packages
            // if package_names.iter().any(|name| name == package_name) {
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

    Ok(idls)
}
