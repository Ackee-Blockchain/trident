use cargo_metadata::Package;

use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

use crate::___private::AnchorVersion;
use crate::commander::Commander;
use crate::source_code_generators;

pub fn load_idls(
    dir_path: PathBuf,
    program_packages: &[Package],
) -> Result<AnchorVersion, Box<dyn Error>> {
    let mut anchor_version: AnchorVersion =
        Commander::get_anchor_version().unwrap_or_default().into();

    let mut package_names = program_packages
        .iter()
        .map(|package| package.name.replace("-", "_"));

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
            if package_names.any(|package| package == package_name) {
                // Open the file in read-only mode
                let mut file = File::open(&path)?;

                // Read the file contents into a string
                let mut json_content = String::new();
                file.read_to_string(&mut json_content)?;

                // Parse the string of data into an Idl struct
                match anchor_version {
                    AnchorVersion::Unknown => todo!(),
                    AnchorVersion::V29(ref mut vec) => {
                        match serde_json::from_str::<source_code_generators::anchor_29::types::Idl>(
                            &json_content,
                        ) {
                            Ok(idl) => vec.push(idl),
                            Err(e) => panic!("Unable to parse IDL: {}", e),
                        }
                    }
                    AnchorVersion::V30(ref mut vec) => {
                        match serde_json::from_str::<anchor_lang_idl_spec::Idl>(&json_content) {
                            Ok(idl) => vec.push(idl),
                            Err(e) => panic!("Unable to parse IDL: {}", e),
                        }
                    }
                }
            }
        }
    }

    Ok(anchor_version)
}
