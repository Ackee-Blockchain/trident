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
use toml::{
    value::{Map, Table},
    Value,
};

pub(crate) const TESTS_WORKSPACE: &str = "trdelnik-tests";
const TESTS_DIRECTORY: &str = "tests";
const FUZZ_DIRECTORY: &str = "src/bin";
const TESTS_FILE_NAME: &str = "test.rs";
const FUZZ_TEST_FILE_NAME: &str = "fuzz_target.rs";

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
    pub async fn generate(&self, skip_fuzzer: bool) {
        let root = match Config::discover_root() {
            Ok(root) => root,
            Err(_) => throw!(Error::BadWorkspace),
        };
        let root_path = root.to_str().unwrap().to_string();
        let commander = Commander::with_root(root_path);
        commander.create_program_client_crate().await?;
        self.generate_test_files(&root).await?;
        self.update_workspace(&root).await?;
        self.build_program_client(&commander).await?;
        if !skip_fuzzer {
            self.generate_fuzz_test_files(&root).await?;
            self.update_gitignore(&root, "hfuzz_target")?;
        }
    }

    /// Builds and generates programs for `program_client` module
    #[throws]
    async fn build_program_client(&self, commander: &Commander) {
        commander.build_programs().await?;
        commander.generate_program_client_deps().await?;
        commander.generate_program_client_lib_rs().await?;
    }

    /// Creates the `trdelnik-tests` workspace with `tests` directory and empty `test.rs` file
    /// finally it generates the `Cargo.toml` file. Crate is generated from `trdelnik-tests`
    /// template located in `client/src/templates`
    #[throws]
    async fn generate_test_files(&self, root: &Path) {
        let workspace_path = root.join(TESTS_WORKSPACE);
        self.create_directory(&workspace_path, TESTS_WORKSPACE)
            .await?;

        let tests_path = workspace_path.join(TESTS_DIRECTORY);
        self.create_directory(&tests_path, TESTS_DIRECTORY).await?;
        let test_path = tests_path.join(TESTS_FILE_NAME);
        let test_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/test.rs"
        ));
        self.create_file(&test_path, TESTS_FILE_NAME, test_content)
            .await?;

        let cargo_toml_path = workspace_path.join(CARGO_TOML);
        let cargo_toml_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/Cargo.toml.tmpl"
        ));
        self.create_file(&cargo_toml_path, CARGO_TOML, cargo_toml_content)
            .await?;
        self.add_program_deps(root, &cargo_toml_path).await?;

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
    async fn generate_fuzz_test_files(&self, root: &Path) {
        let fuzzer_path = root.join(TESTS_WORKSPACE).join(FUZZ_DIRECTORY);
        self.create_directory_all(&fuzzer_path, FUZZ_DIRECTORY)
            .await?;

        let libs = self.get_libs(root).await?;

        let fuzzer_test_path = fuzzer_path.join(FUZZ_TEST_FILE_NAME);
        let fuzz_test_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/trdelnik-tests/fuzz_target.rs"
        ))
        .to_string();

        let fuzz_test_content = if let Some(lib) = libs.first() {
            let use_entry = format!("use {}::entry;\n", lib);
            let prog_name = format!("const PROGRAM_NAME: &str  = \"{lib}\";\n");
            let use_instructions = format!("use program_client::{}_instruction::*;\n", lib);
            format!("{use_entry}{use_instructions}{prog_name}{fuzz_test_content}")
        } else {
            fuzz_test_content
        };

        self.create_file(&fuzzer_test_path, FUZZ_TEST_FILE_NAME, &fuzz_test_content)
            .await?;

        let workspace_path = root.join(TESTS_WORKSPACE);
        let cargo_toml_path = workspace_path.join(CARGO_TOML);
        self.add_feature_to_dep(root, &cargo_toml_path, "trdelnik-client", "fuzzing")
            .await?;
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
    async fn update_workspace(&self, root: &PathBuf) {
        let cargo = Path::new(&root).join(CARGO_TOML);
        let mut content: Value = fs::read_to_string(&cargo).await?.parse()?;
        let test_workspace_value = Value::String(String::from(TESTS_WORKSPACE));
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

    /// Adds programs to Cargo.toml as a dependencies to be able to be used in tests and fuzz targets
    #[throws]
    async fn add_program_deps(&self, root: &Path, cargo_toml_path: &Path) {
        let programs = self.get_programs(root).await?;
        if !programs.is_empty() {
            println!("Adding programs to Cargo.toml ...");
            let mut content: Value = fs::read_to_string(cargo_toml_path).await?.parse()?;
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
            fs::write(cargo_toml_path, content.to_string()).await?;
        } else {
            println!("Skipping adding programs to Cargo.toml ...");
        }
    }

    /// Adds programs to Cargo.toml as a dev dependencies to be able to be used in tests
    #[throws]
    async fn add_feature_to_dep(
        &self,
        root: &Path,
        cargo_toml_path: &Path,
        dependency: &str,
        feature: &str,
    ) {
        let rel_path = cargo_toml_path
            .strip_prefix(root)
            .unwrap_or(Path::new("Cargo.toml"))
            .to_string_lossy()
            .to_string();
        println!("Adding feature {feature} to dependency {dependency} in {rel_path} ...");
        let mut content: Value = fs::read_to_string(cargo_toml_path).await?.parse()?;
        let deps = content
            .get_mut("dependencies")
            .and_then(Value::as_table_mut)
            .ok_or(Error::CannotParseCargoToml)?;

        let values = deps
            .get_mut(dependency)
            .and_then(|f| {
                if f.is_table() {
                    f.as_table_mut()
                } else if f.is_str() {
                    // if the value is only a string with version such as dependency = 0.0, create a new table with that version
                    let version = f.as_str().unwrap();
                    let mut map = Map::new();
                    let _ = map.insert("version".to_string(), Value::String(version.to_string()));
                    let t = Value::Table(map);
                    *f = t.to_owned();
                    f.as_table_mut()
                } else {
                    None
                }
            })
            .ok_or(Error::CannotParseCargoToml)?;

        let fuzzing = Value::String(feature.to_string());
        let value = Value::Array(vec![fuzzing.clone()]);
        let features = values.entry("features").or_insert(value);
        if let Some(features) = features.as_array_mut() {
            if !features.iter().any(|f| *f == fuzzing) {
                features.push(fuzzing);
            };
        }

        fs::write(cargo_toml_path, content.to_string()).await?;
    }

    /// Scans `programs` directory and returns a list of `toml::Value` programs and their paths.
    async fn get_programs(&self, root: &Path) -> Result<Vec<Value>, Error> {
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
            let file_name = file.file_name();
            if file.path().is_dir() {
                let path = file.path().join(CARGO_TOML);
                if path.exists() {
                    let name = file_name.to_str().unwrap();
                    let dependency = self.get_program_dep(&path, name).await?;
                    program_names.push(dependency);
                }
            }
        }
        Ok(program_names)
    }

    /// Scans `programs` directory and returns a list of names of libraries
    async fn get_libs(&self, root: &Path) -> Result<Vec<String>, Error> {
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
                    // let dir = file_name.to_str().unwrap();
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
    async fn get_program_dep<'a>(&self, dir: &Path, dir_name: &'a str) -> Value {
        let content: Value = fs::read_to_string(&dir).await?.parse()?;
        let name = content
            .get("package")
            .and_then(Value::as_table)
            .and_then(|table| table.get("name"))
            .and_then(Value::as_str)
            .ok_or(Error::CannotParseCargoToml)?;
        format!("{name} = {{ path = \"../programs/{dir_name}\" }}")
            .parse()
            .unwrap()
    }
}
