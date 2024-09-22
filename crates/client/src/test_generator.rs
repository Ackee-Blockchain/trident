use crate::___private::test_fuzz_generator;
use crate::commander::{Commander, Error as CommanderError};

use crate::source_code_generators::fuzz_instructions_generator;

use cargo_metadata::Package;
use fehler::{throw, throws};
use std::{fs::File, io::prelude::*};
use std::{
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tokio::fs;
use toml::{value::Table, Value};

use crate::constants::*;

use anchor_lang_idl_spec::Idl;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot parse Cargo.toml")]
    CannotParseCargoToml,
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    Toml(#[from] toml::de::Error),
    #[error("{0:?}")]
    Commander(#[from] CommanderError),
    #[error("Cannot find the Anchor.toml file to locate the root folder")]
    BadWorkspace,
    #[error("The Anchor project does not contain any programs")]
    NoProgramsFound,
    #[error("read program code failed: '{0}'")]
    ReadProgramCodeFailed(String),
    #[error("parsing Cargo.toml dependencies failed")]
    ParsingCargoTomlDependenciesFailed,
}

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

macro_rules! load_template {
    ($file:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $file))
    };
}

pub struct TestGenerator {
    pub root: PathBuf,
    pub program_packages: Vec<Package>,
    pub anchor_idls: Vec<Idl>,
}
impl Default for TestGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl TestGenerator {
    pub fn new() -> Self {
        Self {
            root: Path::new("../../").to_path_buf(),
            program_packages: Vec::default(),
            anchor_idls: Vec::default(),
        }
    }
    pub fn new_with_root(root: String) -> Self {
        Self {
            root: Path::new(&root).to_path_buf(),
            program_packages: Vec::default(),
            anchor_idls: Vec::default(),
        }
    }
    #[throws]
    pub async fn generate_fuzz(&mut self) {
        let root_path = self.root.to_str().unwrap().to_string();
        let commander = Commander::with_root(root_path);

        commander.build_anchor_project().await?;

        self.get_program_packages().await?;
        self.load_idls()?;
        self.init_fuzz_tests().await?;
        self.create_trident_manifest().await?;
        self.update_gitignore(CARGO_TARGET_DIR_DEFAULT_HFUZZ)?;
        self.update_gitignore(CARGO_TARGET_DIR_DEFAULT_AFL)?;
    }
    #[throws]
    pub async fn add_fuzz_test(&mut self) {
        let root_path = self.root.to_str().unwrap().to_string();
        let commander = Commander::with_root(root_path);

        commander.build_anchor_project().await?;

        self.get_program_packages().await?;
        self.load_idls()?;
        self.add_new_fuzz_test().await?;
        self.create_trident_manifest().await?;
        self.update_gitignore(CARGO_TARGET_DIR_DEFAULT_HFUZZ)?;
    }

    #[throws]
    fn load_idls(&mut self) {
        let target_path: PathBuf = [self.root.as_ref(), Path::new("target/idl/")]
            .iter()
            .collect();
        self.anchor_idls =
            crate::anchor_idl::load_idls(target_path, &self.program_packages).unwrap();
    }

    #[throws]
    async fn get_program_packages(&mut self) {
        self.program_packages = Commander::collect_program_packages().await?;
    }

    #[throws]
    async fn init_fuzz_tests(&mut self) {
        // create reuqired paths
        let fuzz_dir_path =
            construct_path!(self.root, TESTS_WORKSPACE_DIRECTORY, FUZZ_TEST_DIRECTORY);
        let fuzz_tests_manifest_path = construct_path!(fuzz_dir_path, CARGO_TOML);

        if fuzz_dir_path.exists() {
            // obtain directory contents
            let mut directories: std::collections::HashSet<_> = fuzz_dir_path
                .read_dir()
                .expect("Reading directory failed")
                .map(|r| {
                    r.expect("Reading directory; DirEntry error")
                        .file_name()
                        .to_string_lossy()
                        .to_string()
                })
                .collect();
            directories.retain(|x| x != "fuzzing");
            directories.retain(|x| x != CARGO_TOML);
            // if folder structure exists and fuzz_tests directory is not empty we skip
            if fuzz_tests_manifest_path.exists() && !directories.is_empty() {
                println!("{SKIP} looks like [Fuzz] Tests are already initialized");
            } else {
                self.add_new_fuzz_test().await?
            }
        } else {
            self.add_new_fuzz_test().await?
        }
    }

    #[throws]
    pub async fn add_new_fuzz_test(&self) {
        let fuzz_dir_path =
            construct_path!(self.root, TESTS_WORKSPACE_DIRECTORY, FUZZ_TEST_DIRECTORY);
        let fuzz_tests_manifest_path = construct_path!(fuzz_dir_path, CARGO_TOML);

        self.create_directory_all(&fuzz_dir_path).await?;

        let fuzz_id = if fuzz_dir_path.read_dir()?.next().is_none() {
            0
        } else {
            let mut directories: std::collections::HashSet<_> = fuzz_dir_path
                .read_dir()
                .expect("Reading directory failed")
                .map(|r| {
                    r.expect("Reading directory; DirEntry error")
                        .file_name()
                        .to_string_lossy()
                        .to_string()
                })
                .collect();

            // INFO discard known entries created by framework, everything else
            // created by user will be taken as fuzz test.
            directories.retain(|x| x != "fuzzing");
            directories.retain(|x| x != "Cargo.toml");

            let mut fuzz_id = directories.len();
            loop {
                let fuzz_test = format!("fuzz_{fuzz_id}");
                if directories.contains(&fuzz_test) && fuzz_id < usize::MAX {
                    fuzz_id += 1;
                } else {
                    break fuzz_id;
                }
            }
        };

        let new_fuzz_test = format!("fuzz_{fuzz_id}");
        let new_fuzz_test_dir = fuzz_dir_path.join(&new_fuzz_test);
        let new_bin_target = format!("{new_fuzz_test}/test_fuzz.rs");

        self.create_directory(&new_fuzz_test_dir).await?;

        self.initialize_fuzz(&new_fuzz_test_dir).await?;

        self.initialize_fuzz_instructions(&new_fuzz_test_dir)
            .await?;

        let cargo_toml_content =
            load_template!("/src/templates/trident-tests/Cargo_fuzz.toml.tmpl");

        self.create_file(&fuzz_tests_manifest_path, cargo_toml_content)
            .await?;

        self.add_bin_target(&fuzz_tests_manifest_path, &new_fuzz_test, &new_bin_target)
            .await?;
        self.add_program_dependencies(
            &fuzz_dir_path,
            "dependencies",
            Some(vec!["trident-fuzzing"]),
        )
        .await?;

        self.add_workspace_member(&format!(
            "{TESTS_WORKSPACE_DIRECTORY}/{FUZZ_TEST_DIRECTORY}",
        ))
        .await?;
    }

    #[throws]
    pub async fn initialize_fuzz(&self, new_fuzz_test_dir: &Path) {
        let fuzz_test_path = new_fuzz_test_dir.join(FUZZ_TEST);

        let program_fuzzer = test_fuzz_generator::generate_source_code(&self.anchor_idls);

        let program_fuzzer = Commander::format_program_code_nightly(&program_fuzzer).await?;

        self.create_file(&fuzz_test_path, &program_fuzzer).await?;
    }

    #[throws]
    pub async fn initialize_fuzz_instructions(&self, new_fuzz_test_dir: &Path) {
        let fuzz_instructions_path = new_fuzz_test_dir.join(FUZZ_INSTRUCTIONS_FILE_NAME);
        let program_fuzzer = fuzz_instructions_generator::generate_source_code(&self.anchor_idls);

        let program_fuzzer = Commander::format_program_code(&program_fuzzer).await?;

        self.create_file(&fuzz_instructions_path, &program_fuzzer)
            .await?;
    }

    #[throws]
    async fn create_trident_manifest(&self) {
        let trident_toml_path = construct_path!(self.root, TRIDENT_TOML);
        let trident_toml_content = load_template!("/src/templates/Trident.toml.tmpl");
        self.create_file(&trident_toml_path, trident_toml_content)
            .await?;
    }
    #[throws]
    async fn add_workspace_member(&self, member: &str) {
        let cargo = construct_path!(self.root, CARGO_TOML);
        let mut content: Value = fs::read_to_string(&cargo).await?.parse()?;
        let new_member = Value::String(String::from(member));

        let members = content
            .as_table_mut()
            .ok_or(Error::CannotParseCargoToml)?
            .entry("workspace")
            .or_insert(Value::Table(Table::default()))
            .as_table_mut()
            .ok_or(Error::CannotParseCargoToml)?
            .entry("members")
            .or_insert(Value::Array(vec![new_member.clone()]))
            .as_array_mut()
            .ok_or(Error::CannotParseCargoToml)?;

        match members.iter().find(|&x| x.eq(&new_member)) {
            Some(_) => {
                println!("{SKIP} [{CARGO_TOML}], already contains [{member}]")
            }
            None => {
                members.push(new_member);
                println!("{FINISH} [{CARGO_TOML}] updated with [{member}]");
                fs::write(cargo, content.to_string()).await?;
            }
        };
    }

    #[throws]
    async fn create_directory_all(&self, path: &PathBuf) {
        match path.exists() {
            true => {}
            false => {
                fs::create_dir_all(path).await?;
            }
        };
    }
    #[throws]
    async fn create_directory(&self, path: &PathBuf) {
        match path.exists() {
            true => {}
            false => {
                fs::create_dir(path).await?;
            }
        };
    }
    #[throws]
    async fn create_file(&self, path: &PathBuf, content: &str) {
        let file = path.strip_prefix(&self.root).unwrap().to_str().unwrap();

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
    fn update_gitignore(&self, ignored_path: &str) {
        let gitignore_path = construct_path!(self.root, GIT_IGNORE);
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
            println!("{SKIP} [{GIT_IGNORE}], not found")
        }
    }
    #[throws]
    async fn add_bin_target(&self, cargo_path: &PathBuf, name: &str, path: &str) {
        // Read the existing Cargo.toml file
        let cargo_toml_content = fs::read_to_string(cargo_path).await?;
        let mut cargo_toml: Value = cargo_toml_content.parse()?;

        // Create a new bin table
        let mut bin_table = Table::new();
        bin_table.insert("name".to_string(), Value::String(name.to_string()));
        bin_table.insert("path".to_string(), Value::String(path.to_string()));

        // Add the new [[bin]] section to the [[bin]] array
        if let Some(bin_array) = cargo_toml.as_table_mut().and_then(|t| t.get_mut("bin")) {
            if let Value::Array(bin_array) = bin_array {
                bin_array.push(Value::Table(bin_table));
            }
        } else {
            // If there is no existing [[bin]] array, create one
            let bin_array = Value::Array(vec![Value::Table(bin_table)]);
            cargo_toml
                .as_table_mut()
                .unwrap()
                .insert("bin".to_string(), bin_array);
        }

        // Write the updated Cargo.toml file
        fs::write(cargo_path, cargo_toml.to_string()).await?;
    }
    #[throws]
    async fn add_program_dependencies(
        &self,
        cargo_dir: &PathBuf,
        deps: &str,
        features: Option<Vec<&str>>,
    ) {
        let cargo_path = construct_path!(cargo_dir, "Cargo.toml");

        let mut cargo_toml_content: toml::Value = fs::read_to_string(&cargo_path).await?.parse()?;

        let client_toml_deps = cargo_toml_content
            .get_mut(deps)
            .and_then(toml::Value::as_table_mut)
            .ok_or(Error::ParsingCargoTomlDependenciesFailed)?;

        if self.program_packages.is_empty() {
            throw!(Error::NoProgramsFound)
        }
        for package in self.program_packages.iter() {
            let manifest_path = package.manifest_path.parent().unwrap().as_std_path();
            // INFO this will obtain relative path
            let relative_path = pathdiff::diff_paths(manifest_path, cargo_dir).unwrap();
            let dep: Value = if features.is_some() {
                format!(
                    r#"{} = {{ path = "{}", features = {:?} }}"#,
                    package.name,
                    relative_path.to_str().unwrap(),
                    features.as_ref().unwrap()
                )
                .parse()
                .unwrap()
            } else {
                format!(
                    r#"{} = {{ path = "{}" }}"#,
                    package.name,
                    relative_path.to_str().unwrap()
                )
                .parse()
                .unwrap()
            };
            if let toml::Value::Table(table) = dep {
                let (name, value) = table.into_iter().next().unwrap();
                client_toml_deps.entry(name).or_insert(value.clone());
            }
        }
        fs::write(cargo_path, cargo_toml_content.to_string()).await?;
    }
}
