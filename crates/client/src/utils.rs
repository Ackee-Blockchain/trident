use crate::test_generator::Error;
use crate::versions_config::TridentVersionsConfig;

use crate::constants::*;
use cargo_metadata::Package;
use fehler::{throw, throws};
use std::path::Path;
use std::{fs::File, io::prelude::*};
use std::{fs::OpenOptions, io, path::PathBuf};
use tokio::fs;
use toml::{value::Table, Value};

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
pub async fn create_directory(path: &PathBuf) {
    match path.exists() {
        true => {}
        false => {
            fs::create_dir(path).await?;
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
}
#[throws]
pub async fn collect_program_packages() -> Vec<cargo_metadata::Package> {
    let packages: Vec<cargo_metadata::Package> = program_packages().collect();
    if packages.is_empty() {
        throw!(Error::NoProgramsFound)
    } else {
        packages
    }
}
pub fn program_packages() -> impl Iterator<Item = cargo_metadata::Package> {
    let cargo_toml_data = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .expect("Cargo.toml reading failed");

    cargo_toml_data.packages.into_iter().filter(|package| {
        // TODO less error-prone test if the package is a _program_?
        if let Some("programs") = package.manifest_path.iter().nth_back(2) {
            return true;
        }
        false
    })
}

#[throws]
pub fn update_gitignore(root: &PathBuf, ignored_path: &str) {
    let gitignore_path = construct_path!(root, GIT_IGNORE);
    if gitignore_path.exists() {
        let file = File::open(&gitignore_path)?;
        for line in io::BufReader::new(file).lines().map_while(Result::ok) {
            if line == ignored_path {
                // INFO do not add the ignored path again if it is already in the .gitignore file
                println!("{SKIP} [{GIT_IGNORE}], already contains [{ignored_path}]");

                return;
            }
        }
        // Check if the file ends with a newline
        let mut file = File::open(&gitignore_path)?;
        let mut buf = [0; 1];
        file.seek(io::SeekFrom::End(-1))?;
        file.read_exact(&mut buf)?;

        let file = OpenOptions::new().append(true).open(gitignore_path);

        if let Ok(mut file) = file {
            if buf[0] == b'\n' {
                writeln!(file, "{}", ignored_path)?;
            } else {
                writeln!(file, "\n{}", ignored_path)?;
            }
            println!("{FINISH} [{GIT_IGNORE}] update with [{ignored_path}]");
        }
    } else {
        println!("{SKIP} [{GIT_IGNORE}], not found");
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
// #[throws]
// pub async fn initialize_package_metadata(
//     packages: &[Package],
//     versions_config: &TridentVersionsConfig,
// ) {
//     for package in packages {
//         let manifest_path = package.manifest_path.as_std_path();
//         let cargo_toml_content = fs::read_to_string(&manifest_path).await?;
//         let mut cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

//         // Ensure the 'trident-fuzzing' feature exists with the required dependency.
//         let features_table = ensure_table(&mut cargo_toml, "features")?;

//         features_table.insert(
//             "trident-fuzzing".to_string(),
//             Value::Array(vec![Value::String("dep:trident-fuzz".to_string())]),
//         );

//         // Ensure the required dependencies are present in the 'dependencies' section.
//         let dependencies_table = ensure_table(&mut cargo_toml, "dependencies")?;

//         // Add 'trident-derive-accounts-snapshots' dependency in table format.
//         dependencies_table.insert(
//             "trident-derive-accounts-snapshots".to_string(),
//             Value::Table({
//                 let mut snapshots_table = toml::Table::new();
//                 snapshots_table.insert(
//                     "version".to_string(),
//                     Value::String(versions_config.trident_derive_accounts_snapshots.clone()),
//                 );
//                 snapshots_table
//             }),
//         );

//         // Add 'trident-fuzz' dependency with specified attributes if not present.
//         dependencies_table.insert(
//             "trident-fuzz".to_string(),
//             Value::Table({
//                 let mut trident_fuzz_table = toml::Table::new();
//                 trident_fuzz_table.insert(
//                     "version".to_string(),
//                     Value::String(versions_config.trident_fuzz.clone()),
//                 );
//                 trident_fuzz_table.insert("optional".to_string(), Value::Boolean(true));
//                 trident_fuzz_table
//             }),
//         );

//         // Write the updated Cargo.toml back to the file.
//         fs::write(&manifest_path, toml::to_string(&cargo_toml).unwrap()).await?;
//     }
// }

// #[throws]
// pub async fn update_package_metadata(
//     packages: &[Package],
//     versions_config: &TridentVersionsConfig,
// ) {
//     for package in packages {
//         let manifest_path = package.manifest_path.as_std_path();
//         let cargo_toml_content = fs::read_to_string(&manifest_path).await?;
//         let mut cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

//         // Ensure the 'trident-fuzzing' feature exists with the required dependency.
//         let features_table = ensure_table(&mut cargo_toml, "features")?;
//         if features_table.contains_key("trident-fuzzing") {
//             println!(
//                 "{SKIP} 'trident-fuzzing' feature already exists in package: {}",
//                 package.name
//             );
//         } else {
//             features_table.insert(
//                 "trident-fuzzing".to_string(),
//                 Value::Array(vec![Value::String("dep:trident-fuzz".to_string())]),
//             );
//         }

//         // Ensure the required dependencies are present in the 'dependencies' section.
//         let dependencies_table = ensure_table(&mut cargo_toml, "dependencies")?;

//         // Add 'trident-derive-accounts-snapshots' dependency in table format.
//         if dependencies_table.contains_key("trident-derive-accounts-snapshots") {
//             println!("{SKIP} 'trident-derive-accounts-snapshots' dependency already exists in package: {}", package.name);
//         } else {
//             dependencies_table.insert(
//                 "trident-derive-accounts-snapshots".to_string(),
//                 Value::Table({
//                     let mut snapshots_table = toml::Table::new();
//                     snapshots_table.insert(
//                         "version".to_string(),
//                         Value::String(versions_config.trident_derive_accounts_snapshots.clone()),
//                     );
//                     snapshots_table
//                 }),
//             );
//         }

//         // Add 'trident-fuzz' dependency with specified attributes if not present.
//         if dependencies_table.contains_key("trident-fuzz") {
//             println!(
//                 "{SKIP} 'trident-fuzz' dependency already exists in package: {}",
//                 package.name
//             );
//         } else {
//             dependencies_table.insert(
//                 "trident-fuzz".to_string(),
//                 Value::Table({
//                     let mut trident_fuzz_table = toml::Table::new();
//                     trident_fuzz_table.insert(
//                         "version".to_string(),
//                         Value::String(versions_config.trident_fuzz.clone()),
//                     );
//                     trident_fuzz_table.insert("optional".to_string(), Value::Boolean(true));
//                     trident_fuzz_table
//                 }),
//             );
//         }

//         // Write the updated Cargo.toml back to the file.
//         fs::write(&manifest_path, toml::to_string(&cargo_toml).unwrap()).await?;
//     }
// }

#[throws]
pub async fn add_workspace_member(root: &Path, member: &str) {
    // Construct the path to the Cargo.toml file
    let cargo = root.join("Cargo.toml");

    // Read and parse the Cargo.toml file
    let cargo_toml_content = fs::read_to_string(&cargo).await?;
    let mut cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

    // Ensure that the 'workspace' table exists
    let workspace_table = ensure_table(&mut cargo_toml, "workspace")?;

    // Ensure that the 'members' array exists within the 'workspace' table
    let members = workspace_table
        .entry("members")
        .or_insert(Value::Array(Vec::new()))
        .as_array_mut()
        .ok_or(Error::CannotParseCargoToml)?;

    // Check if the new member already exists in the 'members' array
    if !members.iter().any(|x| x.as_str() == Some(member)) {
        // Add the new member to the 'members' array
        members.push(Value::String(member.to_string()));
        println!("{FINISH} [{CARGO_TOML}] updated with [{member}]");

        // Write the updated Cargo.toml back to the file
        let updated_toml = toml::to_string(&cargo_toml).unwrap();
        fs::write(cargo, updated_toml).await?;
    } else {
        println!("{SKIP} [{CARGO_TOML}], already contains [{member}]");
    }
}

#[throws]
pub async fn add_bin_target(cargo_path: &PathBuf, name: &str, path: &str) {
    // Read the existing Cargo.toml file
    let cargo_toml_content = fs::read_to_string(cargo_path).await?;
    let mut cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

    // Create a new bin table
    let mut bin_table = Table::new();
    bin_table.insert("name".to_string(), Value::String(name.to_string()));
    bin_table.insert("path".to_string(), Value::String(path.to_string()));

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
    fs::write(cargo_path, updated_toml).await?;
}

#[throws]
pub async fn initialize_fuzz_tests_manifest(
    versions_config: &TridentVersionsConfig,
    _packages: &[Package],
    cargo_dir: &Path,
) {
    let cargo_path = cargo_dir.join("Cargo.toml");

    let cargo_toml_content = fs::read_to_string(&cargo_path).await?;
    let mut cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

    // Ensure the required dependencies are present in the 'dependencies' section.
    let dependencies_table = ensure_table(&mut cargo_toml, "dependencies")?;

    // Add 'trident-fuzz' dependency in table format.
    dependencies_table.insert(
        "trident-fuzz".to_string(),
        Value::Table({
            let mut trident_client = toml::Table::new();
            trident_client.insert(
                "version".to_string(),
                Value::String(versions_config.trident_fuzz.clone()),
            );
            trident_client
        }),
    );

    // for package in packages {
    //     let manifest_path = package.manifest_path.parent().unwrap().as_std_path();
    //     let relative_path = pathdiff::diff_paths(manifest_path, cargo_dir).unwrap();

    //     let relative_path_str = relative_path.to_str().unwrap_or_default();

    //     let package_name = package.name.clone();
    //     dependencies_table.insert(
    //         package_name,
    //         Value::Table({
    //             let mut package_entry = toml::Table::new();
    //             package_entry.insert(
    //                 "path".to_string(),
    //                 Value::String(relative_path_str.to_owned()),
    //             );
    //             // package_entry.insert(
    //             //     "features".to_string(),
    //             //     Value::Array(vec![Value::String("trident-fuzzing".to_string())]),
    //             // );
    //             package_entry
    //         }),
    //     );
    // }

    fs::write(cargo_path, toml::to_string(&cargo_toml).unwrap()).await?;
}

#[throws]
pub async fn update_fuzz_tests_manifest(
    versions_config: &TridentVersionsConfig,
    _packages: &[Package],
    cargo_dir: &Path,
) {
    let cargo_path = cargo_dir.join("Cargo.toml");

    let cargo_toml_content = fs::read_to_string(&cargo_path).await?;
    let mut cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

    // Ensure the required dependencies are present in the 'dependencies' section.
    let dependencies_table = ensure_table(&mut cargo_toml, "dependencies")?;

    // Add 'trident-fuzz' dependency in table format.
    dependencies_table.entry("trident-fuzz").or_insert_with(|| {
        let mut trident_client = toml::Table::new();
        trident_client.insert(
            "version".to_string(),
            Value::String(versions_config.trident_fuzz.clone()),
        );
        Value::Table(trident_client)
    });

    // for package in packages {
    //     let manifest_path = package.manifest_path.parent().unwrap().as_std_path();
    //     let relative_path = pathdiff::diff_paths(manifest_path, cargo_dir).unwrap();

    //     let relative_path_str = relative_path.to_str().unwrap_or_default();

    //     dependencies_table.entry(&package.name).or_insert_with(|| {
    //         let mut package_entry = toml::Table::new();
    //         package_entry.insert(
    //             "path".to_string(),
    //             Value::String(relative_path_str.to_owned()),
    //         );
    //         package_entry.insert(
    //             "features".to_string(),
    //             Value::Array(vec![Value::String("trident-fuzzing".to_string())]),
    //         );
    //         Value::Table(package_entry)
    //     });
    // }

    fs::write(cargo_path, toml::to_string(&cargo_toml).unwrap()).await?;
}
