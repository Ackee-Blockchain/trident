use crate::test_generator::Error;

use crate::constants::*;
use fehler::{throw, throws};
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;

#[macro_export]
macro_rules! construct_path {
    ($root:expr, $($component:expr),*) => {
        {
            let mut path = $root.to_owned();
            $(path = path.join($component);)*
            path
        }
    };
}
#[macro_export]
macro_rules! load_template {
    ($file:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $file))
    };
}

#[throws]
pub async fn create_directory_all(path: &PathBuf) {
    match path.exists() {
        true => {}
        false => {
            fs::create_dir_all(path).await?;
        }
    };
}

#[throws]
pub async fn create_file(root: &PathBuf, path: &PathBuf, content: &str) {
    let file = path.strip_prefix(root)?.to_str().unwrap_or_default();

    match path.exists() {
        true => {
            println!("{SKIP} [{file}] already exists")
        }
        false => {
            fs::write(path, content).await?;
            println!("{FINISH} [{file}] created");
        }
    };
}

#[throws]
pub fn get_fuzz_id(fuzz_dir_path: &Path) -> i32 {
    if fuzz_dir_path.exists() {
        if fuzz_dir_path.read_dir()?.next().is_none() {
            0
        } else {
            let entries = fuzz_dir_path.read_dir()?;
            let mut max_num = -1;
            for entry in entries {
                let entry = entry?;
                let file_name = entry.file_name().into_string().unwrap_or_default();
                if file_name.starts_with("fuzz_") {
                    let stripped = file_name.strip_prefix("fuzz_").unwrap_or_default();
                    let num = stripped.parse::<i32>()?;
                    max_num = max_num.max(num);
                }
            }
            max_num + 1
        }
    } else {
        0
    }
}
#[throws]
pub async fn collect_program_packages(
    program_name: Option<String>,
) -> Vec<cargo_metadata::Package> {
    let packages: Vec<cargo_metadata::Package> = program_packages(program_name).collect();
    if packages.is_empty() {
        throw!(Error::NoProgramsFound)
    } else {
        packages
    }
}
pub fn program_packages(
    program_name: Option<String>,
) -> Box<dyn Iterator<Item = cargo_metadata::Package>> {
    let cargo_toml_data = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .expect("Cargo.toml reading failed");

    match program_name {
        Some(name) => Box::new(
            cargo_toml_data
                .packages
                .into_iter()
                .filter(move |package| package.name == name),
        ),
        None => Box::new(cargo_toml_data.packages.into_iter().filter(|package| {
            // TODO less error-prone test if the package is a _program_?
            if let Some("programs") = package.manifest_path.iter().nth_back(2) {
                return true;
            }
            false
        })),
    }
}

// #[throws]
// pub async fn add_workspace_member(root: &Path, member: &str) {
//     // Construct the path to the Cargo.toml file
//     let cargo = root.join("Cargo.toml");

//     // Read and parse the Cargo.toml file
//     let cargo_toml_content = fs::read_to_string(&cargo).await?;
//     let mut cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

//     // Ensure that the 'workspace' table exists
//     let workspace_table = ensure_table(&mut cargo_toml, "workspace")?;

//     // Ensure that the 'members' array exists within the 'workspace' table
//     let members = workspace_table
//         .entry("members")
//         .or_insert(Value::Array(Vec::new()))
//         .as_array_mut()
//         .ok_or(Error::CannotParseCargoToml)?;

//     // Check if the new member already exists in the 'members' array
//     if !members.iter().any(|x| x.as_str() == Some(member)) {
//         // Add the new member to the 'members' array
//         members.push(Value::String(member.to_string()));
//         println!("{FINISH} [{CARGO_TOML}] updated with [{member}]");

//         // Write the updated Cargo.toml back to the file
//         let updated_toml = toml::to_string(&cargo_toml).unwrap();
//         fs::write(cargo, updated_toml).await?;
//     } else {
//         println!("{SKIP} [{CARGO_TOML}], already contains [{member}]");
//     }
// }
