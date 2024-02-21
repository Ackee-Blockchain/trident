use crate::{
    commander::{Commander, Error as CommanderError},
    fuzzer,
    idl::Idl,
    program_client_generator,
    snapshot_generator::generate_snapshots_code,
};
use cargo_metadata::{camino::Utf8PathBuf, Package};
use fehler::{throw, throws};
use std::{
    env,
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
};
use std::{fs::File, io::prelude::*};
use syn::ItemUse;
use thiserror::Error;
use tokio::fs;
use toml::{value::Table, Value};

use crate::constants::*;

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

pub struct TestGenerator {
    pub root: PathBuf,
    pub idl: Idl,
    pub codes_libs_pairs: Vec<(String, Utf8PathBuf)>,
    pub packages: Vec<Package>,
    pub use_tokens: Vec<ItemUse>,
}
impl Default for TestGenerator {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! construct_path {
    ($root:expr, $($component:expr),*) => {
        {
            let mut path = $root.to_owned();
            $(path = path.join($component);)*
            path
        }
    };
}

impl TestGenerator {
    /// Creates a new instance of `TestGenerator` with default values.
    ///
    /// # Returns
    ///
    /// A new `TestGenerator` instance.
    pub fn new() -> Self {
        Self {
            root: Path::new("../../").to_path_buf(),
            idl: Idl::default(),
            codes_libs_pairs: Vec::default(),
            packages: Vec::default(),
            use_tokens: Vec::default(),
        }
    }
    /// Creates a new instance of `TestGenerator` with a specified root directory.
    ///
    /// # Arguments
    ///
    /// * `root` - A string slice that holds the path to the root directory.
    ///
    /// # Returns
    ///
    /// A new `TestGenerator` instance with the specified root directory.
    pub fn new_with_root(root: String) -> Self {
        Self {
            root: Path::new(&root).to_path_buf(),
            idl: Idl::default(),
            codes_libs_pairs: Vec::default(),
            packages: Vec::default(),
            use_tokens: Vec::default(),
        }
    }

    /// Generates both proof of concept (POC) and fuzz tests along with necessary scaffolding.
    #[throws]
    pub async fn generate_both(&mut self) {
        // expands programs within programs folder
        self.expand_programs_data().await?;
        // expands program_client and obtains
        // use statements
        // if program_client is not yet initialized
        // use statements are set to default
        self.expand_program_client().await?;
        self.create_program_client_crate().await?;
        self.create_trdelnik_tests_crate().await?;
        self.add_new_poc_test().await?;
        self.add_new_fuzz_test().await?;
        self.create_trdelnik_manifest().await?;
        self.update_gitignore("hfuzz_target")?;
    }

    /// Generates fuzz tests along with the necessary setup.
    #[throws]
    pub async fn generate_fuzz(&mut self) {
        self.expand_programs_data().await?;
        self.expand_program_client().await?;
        self.create_trdelnik_tests_crate().await?;
        self.add_new_fuzz_test().await?;
        self.create_trdelnik_manifest().await?;
        self.update_gitignore("trdelnik-tests/fuzz_tests/fuzzing/hfuzz_target")?;
    }
    /// Generates proof of concept (POC) tests along with the necessary setup.
    #[throws]
    pub async fn generate_poc(&mut self) {
        self.expand_programs_data().await?;
        self.expand_program_client().await?;
        self.create_program_client_crate().await?;
        self.create_trdelnik_tests_crate().await?;
        self.add_new_poc_test().await?;
        self.create_trdelnik_manifest().await?;
    }
    #[throws]
    pub async fn build(&mut self) {
        self.expand_programs_data().await?;
        self.create_program_client_crate().await?;
    }
    /// ## Adds new Fuzz test template to the trdelnik-tests folder
    #[throws]
    pub async fn add_fuzz_test(&mut self) {
        self.packages = Commander::collect_program_packages().await?;
        (self.idl, self.codes_libs_pairs) =
            Commander::expand_program_packages(&self.packages).await?;
        self.add_new_fuzz_test().await?;
    }
    /// Gathers and expands program data necessary for generating tests.
    #[throws]
    async fn expand_programs_data(&mut self) {
        self.packages = Commander::collect_program_packages().await?;
        (self.idl, self.codes_libs_pairs) =
            Commander::expand_program_packages(&self.packages).await?;
        self.use_tokens = Commander::expand_program_client().await?;
    }
    /// Gathers and expands program data necessary for generating tests.
    #[throws]
    async fn expand_program_client(&mut self) {
        self.use_tokens = Commander::expand_program_client().await?;
    }

    /// Adds a new proof of concept (POC) test to the test workspace.
    #[throws]
    async fn add_new_poc_test(&self) {
        let program_name = if !&self.idl.programs.is_empty() {
            &self.idl.programs.first().unwrap().name.snake_case
        } else {
            throw!(Error::NoProgramsFound)
        };

        let poc_dir_path =
            construct_path!(self.root, TESTS_WORKSPACE_DIRECTORY, POC_TEST_DIRECTORY);
        let new_poc_test_dir = construct_path!(poc_dir_path, TESTS_DIRECTORY);
        let cargo_path = construct_path!(poc_dir_path, CARGO_TOML);
        let poc_test_path = construct_path!(new_poc_test_dir, POC_TEST);

        self.create_directory(&poc_dir_path).await?;
        self.create_directory(&new_poc_test_dir).await?;
        let cargo_toml_content =
            load_template("/src/templates/trdelnik-tests/Cargo_poc.toml.tmpl")?;
        self.create_file(&cargo_path, &cargo_toml_content).await?;

        let poc_test_content = load_template("/src/templates/trdelnik-tests/test.rs")?;
        let test_content = poc_test_content.replace("###PROGRAM_NAME###", program_name);
        let use_instructions = format!("use program_client::{}_instruction::*;\n", program_name);
        let template = format!("{use_instructions}{test_content}");

        self.create_file(&poc_test_path, &template).await?;

        // add poc test to the workspace virtual manifest
        self.add_workspace_member(&format!("{TESTS_WORKSPACE_DIRECTORY}/{POC_TEST_DIRECTORY}",))
            .await?;

        // add program dev-dependencies into the poc tests Cargo
        // dev-deps are ok as they are used with the cargo test
        self.add_program_dependencies(&poc_dir_path, "dev-dependencies", None)
            .await?;
    }

    /// Creates the `Trdelnik.toml` file
    #[throws]
    async fn create_trdelnik_manifest(&self) {
        let trdelnik_toml_path = construct_path!(self.root, TRDELNIK_TOML);
        let trdelnik_toml_content = load_template("/src/templates/Trdelnik.toml.tmpl")?;
        self.create_file(&trdelnik_toml_path, &trdelnik_toml_content)
            .await?;
    }

    /// Creates the `trdelnik-tests` folder
    #[throws]
    async fn create_trdelnik_tests_crate(&self) {
        let workspace_path = construct_path!(self.root, TESTS_WORKSPACE_DIRECTORY);
        self.create_directory(&workspace_path).await?;
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
                println!("\x1b[93mSkipping\x1b[0m: {CARGO_TOML}, already contains {member}.")
            }
            None => {
                members.push(new_member);
                println!("\x1b[92mSuccesfully\x1b[0m updated: {CARGO_TOML} with {member} member.");
            }
        };
        fs::write(cargo, content.to_string()).await?;
    }

    /// ## Creates program client folder and generates source code
    #[throws]
    async fn create_program_client_crate(&self) {
        let cargo_path = construct_path!(self.root, PROGRAM_CLIENT_DIRECTORY, CARGO_TOML);
        let src_path = construct_path!(self.root, PROGRAM_CLIENT_DIRECTORY, SRC_DIRECTORY);
        let crate_path = construct_path!(self.root, PROGRAM_CLIENT_DIRECTORY);
        let lib_path = construct_path!(self.root, PROGRAM_CLIENT_DIRECTORY, SRC_DIRECTORY, LIB);

        self.create_directory_all(&src_path).await?;

        // load template
        let cargo_toml_content = load_template("/src/templates/program_client/Cargo.toml.tmpl")?;

        // if path exists the file will not be overwritten
        self.create_file(&cargo_path, &cargo_toml_content).await?;

        self.add_program_dependencies(&crate_path, "dependencies", Some(vec!["no-entrypoint"]))
            .await?;

        let program_client =
            program_client_generator::generate_source_code(&self.idl, &self.use_tokens);
        let program_client = Commander::format_program_code(&program_client).await?;

        if lib_path.exists() {
            self.update_file(&lib_path, &program_client).await?;
        } else {
            self.create_file(&lib_path, &program_client).await?;
        }
    }
    /// Creates the `trdelnik-tests` workspace with `src/bin` directory and empty `fuzz_target.rs` file
    #[throws]
    pub async fn add_new_fuzz_test(&self) {
        let program_name = if !&self.idl.programs.is_empty() {
            &self.idl.programs.first().unwrap().name.snake_case
        } else {
            throw!(Error::NoProgramsFound)
        };
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

        let fuzz_test_path = new_fuzz_test_dir.join(FUZZ_TEST);

        let fuzz_test_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/test_fuzz.rs"
        ))
        .to_string();
        let use_entry = format!("use {}::entry;\n", program_name);
        let use_instructions = format!("use {}::ID as PROGRAM_ID;\n", program_name);
        let use_fuzz_instructions = format!(
            "use fuzz_instructions::{}_fuzz_instructions::FuzzInstruction;\n",
            program_name
        );
        let template =
            format!("{use_entry}{use_instructions}{use_fuzz_instructions}{fuzz_test_content}");
        let fuzz_test_content = template.replace("###PROGRAM_NAME###", program_name);

        self.create_file(&fuzz_test_path, &fuzz_test_content)
            .await?;

        // create fuzz instructions file
        let fuzz_instructions_path = new_fuzz_test_dir.join(FUZZ_INSTRUCTIONS_FILE_NAME);
        let program_fuzzer = fuzzer::fuzzer_generator::generate_source_code(&self.idl);
        let program_fuzzer = Commander::format_program_code(&program_fuzzer).await?;

        self.create_file(&fuzz_instructions_path, &program_fuzzer)
            .await?;

        // // create accounts_snapshots file
        let accounts_snapshots_path = new_fuzz_test_dir.join(ACCOUNTS_SNAPSHOTS_FILE_NAME);
        let fuzzer_snapshots = generate_snapshots_code(&self.codes_libs_pairs)
            .map_err(Error::ReadProgramCodeFailed)?;
        let fuzzer_snapshots = Commander::format_program_code(&fuzzer_snapshots).await?;

        self.create_file(&accounts_snapshots_path, &fuzzer_snapshots)
            .await?;

        let cargo_toml_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/Cargo_fuzz.toml.tmpl"
        ));

        self.create_file(&fuzz_tests_manifest_path, cargo_toml_content)
            .await?;

        self.add_bin_target(&fuzz_tests_manifest_path, &new_fuzz_test, &new_bin_target)
            .await?;
        self.add_program_dependencies(&fuzz_dir_path, "dependencies", None)
            .await?;

        self.add_workspace_member(&format!(
            "{TESTS_WORKSPACE_DIRECTORY}/{FUZZ_TEST_DIRECTORY}",
        ))
        .await?;
    }

    /// ## Creates a new directory and all missing parent directories on the specified path
    #[throws]
    async fn create_directory_all<'a>(&self, path: &'a PathBuf) {
        match path.exists() {
            true => {}
            false => {
                fs::create_dir_all(path).await?;
            }
        };
    }
    /// ## Creates directory with specified path
    #[throws]
    async fn create_directory<'a>(&self, path: &'a Path) {
        match path.exists() {
            true => {}
            false => {
                fs::create_dir(path).await?;
            }
        };
    }
    /// ##  Creates a new file with a given content on the specified path
    /// - Skip if file already exists
    #[throws]
    async fn create_file<'a>(&self, path: &'a Path, content: &str) {
        let file = path.strip_prefix(&self.root).unwrap().to_str().unwrap();

        match path.exists() {
            true => {
                println!("\x1b[93mSkipping\x1b[0m: {file}, already exists.")
            }
            false => {
                fs::write(path, content).await?;
                println!("\x1b[92mSuccesfully\x1b[0m created: {file}.");
            }
        };
    }
    /// ## Updates a file with a given content on the specified path
    /// - Skip if file does not exists
    #[throws]
    async fn update_file<'a>(&self, path: &'a Path, content: &str) {
        let file = path.strip_prefix(&self.root).unwrap().to_str().unwrap();
        match path.exists() {
            true => {
                fs::write(path, content).await?;
                println!("\x1b[92mSuccesfully\x1b[0m updated: {file}.");
            }
            false => {
                fs::write(path, content).await?;
                println!("\x1b[92mSuccesfully\x1b[0m created: {file}.");
            }
        };
    }

    /// ## Updates .gitignore file in the `root` directory and appends `ignored_path` to the end of the file
    #[throws]
    fn update_gitignore(&self, ignored_path: &str) {
        let gitignore_path = construct_path!(self.root, GIT_IGNORE);
        if gitignore_path.exists() {
            let file = File::open(&gitignore_path)?;
            for line in io::BufReader::new(file).lines().flatten() {
                if line == ignored_path {
                    // INFO do not add the ignored path again if it is already in the .gitignore file
                    println!(
                        "\x1b[93mSkipping\x1b[0m: {GIT_IGNORE}, already contains {ignored_path}."
                    );

                    return;
                }
            }
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(gitignore_path);

            if let Ok(mut file) = file {
                writeln!(file, "{}", ignored_path)?;
                println!("\x1b[92mSuccesfully\x1b[0m updated: {GIT_IGNORE} with {ignored_path}.");
            }
        } else {
            println!("\x1b[93mSkipping\x1b[0m: {GIT_IGNORE}, not found.")
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
    /// ## Adds program dependency to specified Cargo.toml
    /// - for example, we need to use program entry within the fuzzer
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

        if !&self.packages.is_empty() {
            for package in self.packages.iter() {
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
        } else {
            throw!(Error::NoProgramsFound)
        }
    }
}

pub fn load_template(file_path: &str) -> Result<String, std::io::Error> {
    let mut _path = String::from(MANIFEST_PATH);
    _path.push_str(file_path);
    let full_path = Path::new(&_path);

    std::fs::read_to_string(full_path)
}
