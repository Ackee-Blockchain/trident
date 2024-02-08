use crate::{
    commander::{Commander, Error as CommanderError},
    config::{Config, CARGO_TOML, TRDELNIK_TOML},
};
use fehler::{throw, throws};
use std::{
    env,
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
};
use std::{fs::File, io::prelude::*};
use thiserror::Error;
use tokio::fs;
use toml::{value::Table, Value};

pub(crate) const TESTS_WORKSPACE: &str = "trdelnik-tests";
const TESTS_FILE_NAME: &str = "test.rs";
pub(crate) const FUZZ_INSTRUCTIONS_FILE_NAME: &str = "fuzz_instructions.rs";
pub(crate) const ACCOUNTS_SNAPSHOTS_FILE_NAME: &str = "accounts_snapshots.rs";
pub(crate) const HFUZZ_TARGET: &str = "hfuzz_target";

pub const FUZZ_TEST_DIRECTORY: &str = "fuzz_tests";
pub const FUZZ_TEST: &str = "test_fuzz.rs";
pub const POC_TEST_DIRECTORY: &str = "poc_tests";
pub const TESTS: &str = "tests";
pub const FUZZING: &str = "fuzzing";

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
}

pub struct TestGenerator;
impl Default for TestGenerator {
    fn default() -> Self {
        Self::new()
    }
}
impl TestGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Builds all the programs and creates `.program_client` directory. Initializes the
    /// `trdelnik-tests/tests` directory with all the necessary files. Adds the
    /// `test.rs` file and generates `Cargo.toml` with `dependencies`. Updates root's `Cargo.toml`
    /// workspace members.
    ///
    /// The crate is generated from `trdelnik-tests` template located in `client/src/templates`.
    ///
    /// Before you start writing trdelnik tests do not forget to add your program as a dependency
    /// to the `trdelnik-tests/Cargo.toml`. For example:
    ///
    /// ```toml
    /// # <project_root>/trdelnik-tests/Cargo.toml
    /// # ...
    /// [dependencies]
    /// my-program = { path = "../programs/my-program" }
    /// # ...
    /// ```
    ///
    /// Then you can easily use it in tests:
    ///
    /// ```ignore
    /// use my_program;
    ///
    /// // ...
    ///
    /// #[trdelnik_test]
    /// async fn test() {
    ///     // ...
    ///     my_program::do_something(/*...*/);
    ///     // ...
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - there is not a root directory (no `Anchor.toml` file)
    #[throws]
    pub async fn generate(&self, _skip_fuzzer: bool) {
        let root = match Config::discover_root() {
            Ok(root) => root,
            Err(_) => throw!(Error::BadWorkspace),
        };
        let root_path = root.to_str().unwrap().to_string();
        let commander = Commander::with_root(root_path);
        commander.create_program_client_crate().await?;
        self.generate_test_files(&root).await?;
        self.update_workspace(&root, "trdelnik-tests/poc_tests")
            .await?;
        let new_fuzz_test_dir = self.generate_fuzz_test_files(&root).await?;
        self.build_program_client(&commander, new_fuzz_test_dir)
            .await?;
        self.update_gitignore(
            &root,
            &format!("{TESTS_WORKSPACE}/{FUZZ_TEST_DIRECTORY}/{FUZZING}/{HFUZZ_TARGET}"),
        )?;
    }

    #[throws]
    pub async fn add_new_fuzz_test(&self) {
        let root = match Config::discover_root() {
            Ok(root) => root,
            Err(_) => throw!(Error::BadWorkspace),
        };
        let new_fuzz_test_dir = self.generate_fuzz_test_files(&root).await?;

        let root_path = root.to_str().unwrap().to_string();
        let commander = Commander::with_root(root_path);
        self.build_program_client(&commander, new_fuzz_test_dir)
            .await?;
        self.update_gitignore(
            &root,
            &format!("{TESTS_WORKSPACE}/{FUZZ_TEST_DIRECTORY}/{FUZZING}/{HFUZZ_TARGET}"),
        )?;
    }

    /// Builds and generates programs for `program_client` module
    #[throws]
    async fn build_program_client(&self, commander: &Commander, new_fuzz_test_dir: PathBuf) {
        commander.build_programs().await?;
        commander.generate_program_client_deps().await?;
        commander
            .generate_program_client_lib_rs(Some(new_fuzz_test_dir))
            .await?;
    }

    /// Creates the `trdelnik-tests` workspace with `tests` directory and empty `test.rs` file
    /// finally it generates the `Cargo.toml` file. Crate is generated from `trdelnik-tests`
    /// template located in `client/src/templates`
    #[throws]
    async fn generate_test_files(&self, root: &Path) {
        let workspace_path = root
            .join(TESTS_WORKSPACE)
            .join(POC_TEST_DIRECTORY)
            .join(TESTS);

        self.create_directory_all(&workspace_path, TESTS).await?;

        let test_path = workspace_path.join(TESTS_FILE_NAME);

        let test_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/test.rs"
        ));

        let program_libs = self.get_program_lib_names(root).await?;

        let program_name = if let Some(name) = program_libs.first() {
            name
        } else {
            throw!(Error::NoProgramsFound)
        };

        let test_content = test_content.replace("###PROGRAM_NAME###", program_name);
        let use_instructions = format!("use program_client::{}_instruction::*;\n", program_name);
        let template = format!("{use_instructions}{test_content}");

        self.create_file(&test_path, TESTS_FILE_NAME, &template)
            .await?;

        let cargo_toml_path = root
            .join(TESTS_WORKSPACE)
            .join(POC_TEST_DIRECTORY)
            .join(CARGO_TOML);

        let cargo_toml_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/Cargo_poc.toml.tmpl"
        ));

        self.create_file(&cargo_toml_path, CARGO_TOML, cargo_toml_content)
            .await?;

        let cargo_toml_dir = root.join(TESTS_WORKSPACE).join(POC_TEST_DIRECTORY);
        self.add_program_deps(root, &cargo_toml_dir).await?;

        let trdelnik_toml_path = root.join(TRDELNIK_TOML);
        let trdelnik_toml_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/Trdelnik.toml.tmpl"
        ));
        self.create_file(&trdelnik_toml_path, TRDELNIK_TOML, trdelnik_toml_content)
            .await?;
    }

    /// Creates the `trdelnik-tests` workspace with `src/bin` directory and empty `fuzz_target.rs` file
    #[throws]
    async fn generate_fuzz_test_files(&self, root: &Path) -> PathBuf {
        let fuzz_dir_path = root.join(TESTS_WORKSPACE).join(FUZZ_TEST_DIRECTORY);
        let fuzz_tests_manifest_path = fuzz_dir_path.join(CARGO_TOML);

        self.create_directory_all(&fuzz_dir_path, FUZZ_TEST_DIRECTORY)
            .await?;

        let libs = self.get_program_lib_names(root).await?;

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

        self.create_directory(&new_fuzz_test_dir, &new_fuzz_test)
            .await?;

        let fuzz_test_path = new_fuzz_test_dir.join(FUZZ_TEST);

        let fuzz_test_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/test_fuzz.rs"
        ))
        .to_string();

        // create fuzz target file
        let fuzz_test_content = if let Some(lib) = libs.first() {
            let use_entry = format!("use {}::entry;\n", lib);
            let use_instructions = format!("use program_client::{}_instruction::*;\n", lib);
            let use_fuzz_instructions = format!(
                "use fuzz_instructions::{}_fuzz_instructions::FuzzInstruction;\n",
                lib
            );
            let template =
                format!("{use_entry}{use_instructions}{use_fuzz_instructions}{fuzz_test_content}");
            template.replace("###PROGRAM_NAME###", lib)
        } else {
            throw!(Error::NoProgramsFound)
        };

        self.create_file(&fuzz_test_path, FUZZ_TEST, &fuzz_test_content)
            .await?;

        // create fuzz instructions file
        let fuzz_instructions_path = new_fuzz_test_dir.join(FUZZ_INSTRUCTIONS_FILE_NAME);
        let fuzz_instructions_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/fuzz_instructions.rs"
        ));
        self.create_file(
            &fuzz_instructions_path,
            FUZZ_INSTRUCTIONS_FILE_NAME,
            fuzz_instructions_content,
        )
        .await?;

        // // create accounts_snapshots file
        let accounts_snapshots_path = new_fuzz_test_dir.join(ACCOUNTS_SNAPSHOTS_FILE_NAME);
        let accounts_snapshots_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/accounts_snapshots.rs"
        ));
        self.create_file(
            &accounts_snapshots_path,
            ACCOUNTS_SNAPSHOTS_FILE_NAME,
            accounts_snapshots_content,
        )
        .await?;

        let cargo_toml_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/Cargo_fuzz.toml.tmpl"
        ));

        self.create_file(&fuzz_tests_manifest_path, CARGO_TOML, cargo_toml_content)
            .await?;

        self.add_bin_target(&fuzz_tests_manifest_path, &new_fuzz_test, &new_bin_target)
            .await?;
        self.add_program_deps(root, &fuzz_dir_path).await?;

        self.update_workspace(&root.to_path_buf(), "trdelnik-tests/fuzz_tests")
            .await?;
        new_fuzz_test_dir
    }

    /// Creates a new file with a given content on the specified `path` and `name`
    // todo: the function should be located in the different module, File module for example
    async fn create_file<'a>(
        &self,
        path: &'a PathBuf,
        name: &str,
        content: &str,
    ) -> Result<&'a PathBuf, Error> {
        match path.exists() {
            true => println!("Skipping creating the {name} file"),
            false => {
                println!("Creating the {name} file ...");
                fs::write(path, content).await?;
            }
        };
        Ok(path)
    }

    /// Creates a new directory on the specified `path` and with the specified `name`
    // todo: the function should be located in the different module, File module for example
    async fn create_directory<'a>(
        &self,
        path: &'a PathBuf,
        name: &str,
    ) -> Result<&'a PathBuf, Error> {
        match path.exists() {
            true => println!("Skipping creating the {name} directory"),
            false => {
                println!("Creating the {name} directory ...");
                fs::create_dir(path).await?;
            }
        };
        Ok(path)
    }

    /// Creates a new directory and all missing parent directories on the specified `path` and with the specified `name`
    // todo: the function should be located in the different module, File module for example
    async fn create_directory_all<'a>(
        &self,
        path: &'a PathBuf,
        name: &str,
    ) -> Result<&'a PathBuf, Error> {
        match path.exists() {
            true => println!("Skipping creating the {name} directory"),
            false => {
                println!("Creating the {name} directory ...");
                fs::create_dir_all(path).await?;
            }
        };
        Ok(path)
    }

    /// Adds `trdelnik-tests` workspace to the `root`'s `Cargo.toml` workspace members if needed.
    #[throws]
    async fn update_workspace(&self, root: &PathBuf, new_member: &str) {
        let cargo = Path::new(&root).join(CARGO_TOML);
        let mut content: Value = fs::read_to_string(&cargo).await?.parse()?;
        let test_workspace_value = Value::String(String::from(new_member));
        let members = content
            .as_table_mut()
            .ok_or(Error::CannotParseCargoToml)?
            .entry("workspace")
            .or_insert(Value::Table(Table::default()))
            .as_table_mut()
            .ok_or(Error::CannotParseCargoToml)?
            .entry("members")
            .or_insert(Value::Array(vec![test_workspace_value.clone()]))
            .as_array_mut()
            .ok_or(Error::CannotParseCargoToml)?;
        match members.iter().find(|&x| x.eq(&test_workspace_value)) {
            Some(_) => println!("Skipping updating project workspace"),
            None => {
                members.push(test_workspace_value);
                println!("Project workspace successfully updated");
            }
        };
        fs::write(cargo, content.to_string()).await?;
    }

    /// Updates .gitignore file in the `root` directory and appends `ignored_path` to the end of the file
    #[throws]
    fn update_gitignore(&self, root: &Path, ignored_path: &str) {
        let file_path = root.join(".gitignore");
        if file_path.exists() {
            let file = File::open(&file_path)?;
            for line in io::BufReader::new(file).lines().flatten() {
                if line == ignored_path {
                    // do not add the ignored path again if it is already in the .gitignore file
                    return;
                }
            }
            let file = OpenOptions::new().write(true).append(true).open(file_path);

            if let Ok(mut file) = file {
                writeln!(file, "{}", ignored_path)?;
                println!(".gitignore file sucessfully updated");
            }
        } else {
            println!("Skipping updating .gitignore file");
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

    /// Adds programs to Cargo.toml as a dependencies to be able to be used in tests and fuzz targets
    #[throws]
    async fn add_program_deps(&self, root: &Path, cargo_toml_dir: &Path) {
        let cargo_toml_path = cargo_toml_dir.join("Cargo.toml");
        let programs = self
            .get_programs(root, &cargo_toml_dir.to_path_buf())
            .await?;
        if !programs.is_empty() {
            println!("Adding programs to Cargo.toml ...");
            let mut content: Value = fs::read_to_string(&cargo_toml_path).await?.parse()?;
            let dev_deps = content
                .get_mut("dependencies")
                .and_then(Value::as_table_mut)
                .ok_or(Error::CannotParseCargoToml)?;
            for dep in programs {
                if let Value::Table(table) = dep {
                    let (name, value) = table.into_iter().next().unwrap();
                    dev_deps.entry(name).or_insert(value);
                }
            }
            fs::write(&cargo_toml_path, content.to_string()).await?;
        } else {
            println!("Skipping adding programs to Cargo.toml ...");
        }
    }

    /// Scans `programs` directory and returns a list of `toml::Value` programs and their paths.
    async fn get_programs(&self, root: &Path, cargo_dir: &PathBuf) -> Result<Vec<Value>, Error> {
        let programs = root.join("programs");
        if !programs.exists() {
            println!("Programs folder does not exist.");
            return Ok(Vec::new());
        }
        println!("Searching for programs ...");
        let mut program_names: Vec<Value> = vec![];
        let programs = std::fs::read_dir(programs)?;
        for program in programs {
            let file = program?;
            if file.path().is_dir() {
                let path = file.path().join(CARGO_TOML);
                if path.exists() {
                    let dependency = self.get_program_dep(&path, cargo_dir).await?;
                    program_names.push(dependency);
                }
            }
        }
        Ok(program_names)
    }

    /// Scans `programs` directory and returns a list of names of libraries
    async fn get_program_lib_names(&self, root: &Path) -> Result<Vec<String>, Error> {
        let programs = root.join("programs");
        if !programs.exists() {
            println!("Programs folder does not exist.");
            return Ok(Vec::new());
        }
        println!("Searching for programs ...");
        let mut program_names: Vec<String> = vec![];
        let programs = std::fs::read_dir(programs)?;
        for program in programs {
            let file = program?;
            if file.path().is_dir() {
                let path = file.path().join(CARGO_TOML);
                if path.exists() {
                    let content: Value = fs::read_to_string(&path).await?.parse()?;
                    let name = content
                        .get("lib")
                        .and_then(Value::as_table)
                        .and_then(|table| table.get("name"))
                        .and_then(Value::as_str)
                        .ok_or(Error::CannotParseCargoToml)?;
                    program_names.push(name.to_string());
                }
            }
        }
        Ok(program_names)
    }

    /// Gets the program name from `<program>/Cargo.toml` and returns a `toml::Value` program dependency.
    #[throws]
    async fn get_program_dep<'a>(&self, dir: &Path, cargo_dir: &PathBuf) -> Value {
        let manifest_path = dir.parent().unwrap();
        let relative_path = pathdiff::diff_paths(manifest_path, cargo_dir).unwrap();

        let content: Value = fs::read_to_string(&dir).await?.parse()?;
        let name = content
            .get("package")
            .and_then(Value::as_table)
            .and_then(|table| table.get("name"))
            .and_then(Value::as_str)
            .ok_or(Error::CannotParseCargoToml)?;
        format!(
            r#"{} = {{ path = "{}" }}"#,
            name,
            relative_path.to_str().unwrap()
        )
        .parse()
        .unwrap()
    }
}
