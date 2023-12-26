// use crate::commander::{Commander, Error as CommanderError};

use cargo_metadata::Package;
use fehler::{throw, throws};
// use pathdiff;
use std::{fs::File, io::prelude::*};
use std::{
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
};
use syn::ItemUse;
use thiserror::Error;
use tokio::fs;
use toml::{value::Table, Value};

use crate::commander::Error as CommanderError;
use crate::constants::*;
use crate::program_client_generator::ProgramCLientGenerator;
use crate::Commander;
use crate::Idl;

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
    #[error("The Anchor project does not contain any programs")]
    NoProgramsFound,
    #[error("parsing Cargo.toml dependencies failed")]
    ParsingCargoTomlDependenciesFailed,
}

pub struct WorkspaceBuilder {
    root: PathBuf,
    idl: Idl,
    use_tokens: Vec<ItemUse>,
    packages: Vec<Package>,
}
impl Default for WorkspaceBuilder {
    fn default() -> Self {
        Self::new_with_root("../../".to_string())
    }
}
impl WorkspaceBuilder {
    /// ## Initializes WorkspaceBuilder with specified root
    /// - 'use_tokens' is set to default use statement
    ///     - trdelnik_client::prelude::*;
    pub fn new_with_root(root: String) -> Self {
        Self {
            root: Path::new(&root).to_path_buf(),
            idl: Idl::default(),
            use_tokens: vec![syn::parse_quote! { use trdelnik_client::prelude::*; }],
            packages: vec![],
        }
    }
    /// ## Build program packages, accordingly updates program client
    /// - builds program packages
    /// - builds program client and obtains custom use statements
    /// - accordingly updates program client
    #[throws]
    pub async fn build(&mut self, _arch: &str) {
        self.packages = Commander::collect_program_packages().await?;
        self.idl = Commander::build_program_packages(&self.packages).await?;
        self.use_tokens = Commander::build_program_client().await?;
        self.update_program_client_crate().await?;

        // self.update_program_client().await?;
        //self.update_program_stubs().await?;
        //self.add_invoked_program_deps().await?;
    }
    /// ## Calls anchor clean and cleans corresponding hfuzz-target if exists
    #[throws]
    pub async fn clean(&self) {
        Commander::clean_anchor_target().await?;
        Commander::clean_hfuzz_target(&self.root).await?;
    }
    /// ## Initializes template for FUZZ Tests
    /// - builds current project
    /// - generates program client
    /// - generates fuzz template
    /// - generates Trdelnik manifest file
    /// - updates .gitignore with hfuzz_target folder
    #[throws]
    pub async fn initialize_fuzz(&mut self, arch: &str) {
        self.generate_program_client(arch).await?;
        self.create_trdelnik_tests_crate().await?;
        self.add_new_fuzz_test().await?;
        self.create_trdelnik_manifest().await?;
        self.update_gitignore("hfuzz_target")?;
    }
    /// ## Initializes template for PoC Tests
    /// - builds current project
    /// - generates program client
    /// - build program client
    /// - generates PoC template
    /// - generates Trdelnik manifest file
    #[throws]
    pub async fn initialize_poc(&mut self, arch: &str) {
        self.generate_program_client(arch).await?;
        self.create_trdelnik_tests_crate().await?;
        self.add_new_poc_test().await?;
        self.create_trdelnik_manifest().await?;
    }
    /// ## Initializes template for FUZZ and PoC Tests
    /// - builds current project
    /// - generates program client
    /// - build program client
    /// - generates Fuzz template
    /// - generates PoC template
    /// - generates Trdelnik manifest file
    /// - updates .gitignore with hfuzz_target folder
    #[throws]
    pub async fn initialize_both(&mut self, arch: &str) {
        self.generate_program_client(arch).await?;
        self.create_trdelnik_tests_crate().await?;
        self.add_new_poc_test().await?;
        self.add_new_fuzz_test().await?;

        self.create_trdelnik_manifest().await?;
        self.update_gitignore("hfuzz_target")?;
    }
    /// ## Builds program packages, generates source code for program client and builds program client
    /// - obtains program packages names and paths
    /// - obtains data for generating program client
    /// - generates program client
    /// - builds program client
    ///
    /// Building program client at the end will ensure that during 'trdelnik build'
    /// command we will not need to wait long time
    #[throws]
    async fn generate_program_client(&mut self, _arch: &str) {
        self.packages = Commander::collect_program_packages().await?;
        self.idl = Commander::build_program_packages(&self.packages).await?;
        // INFO even though this is ok , currently default cargo toml status is that
        // it will have trdelnik_client version set to 0.5.0 that means it will not actually
        // build, but once updated it should be ok
        self.create_program_client_crate().await?;
        self.use_tokens = Commander::build_program_client().await?;
    }
    /// ## Adds new Fuzz test template to the trdelnik-tests folder
    #[throws]
    pub async fn add_fuzz_test(&mut self) {
        self.packages = Commander::collect_program_packages().await?;
        self.add_new_fuzz_test().await?;
    }
    /// ## Creates program client folder and generates source code
    #[throws]
    async fn create_program_client_crate(&self) {
        let crate_path = self.root.join(PROGRAM_CLIENT_DIRECTORY);
        let cargo_path = crate_path.join(CARGO);
        let src_path = crate_path.join(SRC);
        let lib_path = src_path.join(LIB);

        self.create_directory_all(&src_path).await?;

        let cargo_toml_content = load_template("/src/templates/program_client/Cargo.toml.tmpl")?;

        self.create_file(&cargo_path, &cargo_toml_content).await?;

        self.add_program_dependencies(&crate_path, "dependencies")
            .await?;

        let program_client =
            ProgramCLientGenerator::generate_source_code(&self.idl, &self.use_tokens);
        let program_client = Commander::format_program_code(&program_client).await?;

        self.create_file(&lib_path, &program_client).await?;
    }
    /// ## Create trdelnik-test folder
    #[throws]
    async fn create_trdelnik_tests_crate(&self) {
        let workspace_path = self.root.join(TESTS_WORKSPACE_DIRECTORY);
        self.create_directory(&workspace_path).await?;
    }
    /// ## Generates new folder and contents for new Fuzz test Template
    #[throws]
    async fn add_new_fuzz_test(&self) {
        // this check should be ensured within package collection , but
        // double check wont hurt
        let program_name = if !&self.packages.is_empty() {
            &self.packages.first().unwrap().name
        } else {
            throw!(Error::NoProgramsFound)
        };

        let fuzz_dir_path = self
            .root
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(FUZZ_TEST_DIRECTORY);
        self.create_directory(&fuzz_dir_path).await?;

        let fuzz_tests_dir = self
            .root
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(FUZZ_TEST_DIRECTORY);

        let fuzz_id = if fuzz_tests_dir.read_dir()?.next().is_none() {
            0
        } else {
            let mut directories: Vec<_> = fuzz_tests_dir
                .read_dir()
                .unwrap()
                .map(|r| r.unwrap())
                .collect();
            directories.sort_by_key(|dir| dir.path());

            // INFO this is kind of spaghetti, but esentially we are:
            // taking last element from the sorted list
            // splitting its name by '_' as this is expected delimeter
            // for names such "fuzz_0", and then take the number and add 1, this should ensure
            // that the name will be unique
            String::from(
                directories
                    .last()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .split('_')
                    .last()
                    .unwrap(),
            )
            .parse::<u32>()
            .unwrap()
                + 1
        };

        let new_fuzz_test = format!("fuzz_{fuzz_id}");

        let new_fuzz_test_dir = self
            .root
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(FUZZ_TEST_DIRECTORY)
            .join(&new_fuzz_test);

        self.create_directory(&new_fuzz_test_dir).await?;

        let cargo_path = self
            .root
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(FUZZ_TEST_DIRECTORY)
            .join(&new_fuzz_test)
            .join(CARGO);

        let cargo_toml_content =
            load_template("/src/templates/trdelnik-tests/Cargo_fuzz.toml.tmpl")?;
        let cargo_content = cargo_toml_content.replace("###FUZZ_ID###", &fuzz_id.to_string());

        self.create_file(&cargo_path, &cargo_content).await?;

        let fuzz_test_path = self
            .root
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(FUZZ_TEST_DIRECTORY)
            .join(&new_fuzz_test)
            .join(FUZZ_TEST);

        let fuzz_test_content = load_template("/src/templates/trdelnik-tests/fuzz_test.tmpl.rs")?;

        let use_entry = format!("use {}::entry;\n", program_name);
        let use_instructions = format!("use program_client::{}_instruction::*;\n", program_name);
        let mut template = format!("{use_entry}{use_instructions}{fuzz_test_content}");
        template = template.replace("###PROGRAM_NAME###", program_name);

        self.create_file(&fuzz_test_path, &template).await?;

        // add this new fuzz test to the workspace
        self.add_workspace_member(&format!(
            "{TESTS_WORKSPACE_DIRECTORY}/{FUZZ_TEST_DIRECTORY}/{new_fuzz_test}",
        ))
        .await?;

        // add program dependencies
        self.add_program_dependencies(&new_fuzz_test_dir, "dependencies")
            .await?;

        // add fuzzing feature
        // this should be not necessary as we have fuzzing feature already within the template
        // self.add_feature_to_dep("trdelnik-client", "fuzzing", &new_fuzz_test_dir)
        //     .await?;
    }

    /// ## Generates new folder and contents for new PoC test Template
    #[throws]
    async fn add_new_poc_test(&self) {
        // INFO only one POC test file needed
        // as we can implement multiple test paths within one file so no need to create
        // or add new test files, however can be added in the future
        let program_name = if !&self.packages.is_empty() {
            &self.packages.first().unwrap().name
        } else {
            throw!(Error::NoProgramsFound)
        };

        let poc_dir_path = self
            .root
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(POC_TEST_DIRECTORY);

        self.create_directory(&poc_dir_path).await?;

        let new_poc_test_dir = self
            .root
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(POC_TEST_DIRECTORY)
            .join(TESTS);

        self.create_directory(&new_poc_test_dir).await?;

        let cargo_path = self
            .root
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(POC_TEST_DIRECTORY)
            .join(CARGO);

        let cargo_toml_content =
            load_template("/src/templates/trdelnik-tests/Cargo_poc.toml.tmpl")?;

        self.create_file(&cargo_path, &cargo_toml_content).await?;

        let poc_test_path = self
            .root
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(POC_TEST_DIRECTORY)
            .join(TESTS)
            .join(POC_TEST);

        let poc_test_content = load_template("/src/templates/trdelnik-tests/poc_test.tmpl.rs")?;

        let test_content = poc_test_content.replace("###PROGRAM_NAME###", program_name);
        let use_instructions = format!("use program_client::{}_instruction::*;\n", program_name);
        let template = format!("{use_instructions}{test_content}");

        self.create_file(&poc_test_path, &template).await?;

        self.add_workspace_member(&format!("{TESTS_WORKSPACE_DIRECTORY}/{POC_TEST_DIRECTORY}",))
            .await?;

        self.add_program_dependencies(&poc_dir_path, "dev-dependencies")
            .await?;
    }

    // Creates the `trdelnik-tests` workspace with `src/bin` directory and empty `fuzz_target.rs` file
    // #[throws]
    // async fn create_trdelnik_tests_fuzz(&self) {
    //     let fuzzer_path = self
    //         .root
    //         .join(TESTS_WORKSPACE_DIRECTORY)
    //         .join(FUZZ_TEST_DIRECTORY);
    //     //let program_stubs_path = fuzzer_path.join(PROGRAM_STUBS);
    //     let fuzzer_test_path = fuzzer_path.join(FUZZ_TEST);

    //     self.create_directory_all(&fuzzer_path).await?;

    //     let fuzz_test_content = include_str!(concat!(
    //         env!("CARGO_MANIFEST_DIR"),
    //         "/src/templates/trdelnik-tests/fuzz_target.rs"
    //     ))
    //     .to_string();

    //     // let program_stubs_content = include_str!(concat!(
    //     //     env!("CARGO_MANIFEST_DIR"),
    //     //     "/src/templates/trdelnik-tests/program_stubs.rs"
    //     // ))
    //     // .to_string();

    //     match &self.idl {
    //         Some(idl) => {
    //             let fuzz_test_content = get_fuzz_test_content(idl, fuzz_test_content)?;
    //             self.create_file(&fuzzer_test_path, &fuzz_test_content)
    //                 .await?;

    //             // let program_stubs_content = get_program_stubs_content(idl, program_stubs_content)?;
    //             // self.create_file(&program_stubs_path, &program_stubs_content)
    //             //     .await?;
    //         }
    //         None => {
    //             throw!(Error::NoProgramsFound)
    //         }
    //     }

    //     self.add_feature_to_dep("trdelnik-client", "fuzzing")
    //         .await?;
    // }

    /// ## Creates Trdelnik manifest from template
    #[throws]
    async fn create_trdelnik_manifest(&self) {
        let trdelnik_toml_path = self.root.join(TRDELNIK);
        let trdelnik_toml_content = load_template("/src/templates/Trdelnik.toml.tmpl")?;
        self.create_file(&trdelnik_toml_path, &trdelnik_toml_content)
            .await?;
    }
    /// ## Updates program client generated source code
    #[throws]
    async fn update_program_client_crate(&self) {
        let crate_path = self.root.join(PROGRAM_CLIENT_DIRECTORY);
        let cargo_path = crate_path.join(CARGO);
        let src_path = crate_path.join(SRC);
        let lib_path = src_path.join(LIB);

        self.create_directory_all(&src_path).await?;

        let cargo_toml_content = load_template("/src/templates/program_client/Cargo.toml.tmpl")?;

        self.create_file(&cargo_path, &cargo_toml_content).await?;

        self.add_program_dependencies(&crate_path, "dependencies")
            .await?;

        let program_client =
            ProgramCLientGenerator::generate_source_code(&self.idl, &self.use_tokens);
        let program_client = Commander::format_program_code(&program_client).await?;

        self.update_file(&lib_path, &program_client).await?;
    }
    /// ## Add workspace member to the project root Cargo.toml
    #[throws]
    async fn add_workspace_member(&self, member: &str) {
        let cargo = self.root.join(CARGO);
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
                println!(
                    "\x1b[93m--> Skipping <--\x1b[0m \x1b[93m{CARGO}\x1b[0m, already contains {member}."
                )
            }
            None => {
                members.push(new_member);
                println!("\x1b[92mSuccesfully\x1b[0m updated: \x1b[93m{CARGO}\x1b[0m with \x1b[93m{member}\x1b[0m member.");
            }
        };
        fs::write(cargo, content.to_string()).await?;
    }
    /// ## Updates .gitignore file in the `root` directory and appends `ignored_path` to the end of the file
    #[throws]
    fn update_gitignore(&self, ignored_path: &str) {
        let file_path = self.root.join(GIT_IGNORE);
        if file_path.exists() {
            let file = File::open(&file_path)?;
            for line in io::BufReader::new(file).lines().flatten() {
                if line == ignored_path {
                    // INFO do not add the ignored path again if it is already in the .gitignore file
                    println!(
                        "\x1b[93m--> Skipping <--\x1b[0m \x1b[93m{GIT_IGNORE}\x1b[0m, already contains \x1b[93m{ignored_path}\x1b[0m."
                    );

                    return;
                }
            }
            let file = OpenOptions::new().write(true).append(true).open(file_path);

            if let Ok(mut file) = file {
                writeln!(file, "{}", ignored_path)?;
                println!("\x1b[92mSuccesfully\x1b[0m updated: \x1b[93m{GIT_IGNORE}\x1b[0m with \x1b[93m{ignored_path}\x1b[0m.");
            }
        } else {
            println!("\x1b[93m--> Skipping <--\x1b[0m \x1b[93m{GIT_IGNORE}\x1b[0m, not found.")
        }
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
                println!("\x1b[93m--> Skipping <--\x1b[0m \x1b[93m{file}\x1b[0m, already exists.")
            }
            false => {
                fs::write(path, content).await?;
                println!("\x1b[92mSuccesfully\x1b[0m created: \x1b[93m{file}\x1b[0m.");
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
                println!("\x1b[92mSuccesfully\x1b[0m updated: \x1b[93m{file}\x1b[0m.");
            }
            false => {
                fs::write(path, content).await?;
                println!("\x1b[92mSuccesfully\x1b[0m created: \x1b[93m{file}\x1b[0m.");
            }
        };
    }

    // #[throws]
    // async fn add_feature_to_dep(&self, dependency: &str, feature: &str, cargo_dir: &Path) {
    //     let cargo_toml_path = cargo_dir.join(CARGO);
    //     let rel_path = &cargo_toml_path
    //         .strip_prefix(&self.root)
    //         .unwrap()
    //         .to_str()
    //         .unwrap();
    //     let mut content: Value = fs::read_to_string(&cargo_toml_path).await?.parse()?;
    //     let deps = content
    //         .get_mut("dependencies")
    //         .and_then(Value::as_table_mut)
    //         .ok_or(Error::CannotParseCargoToml)?;

    //     let values = deps
    //         .get_mut(dependency)
    //         .and_then(|f| {
    //             if f.is_table() {
    //                 f.as_table_mut()
    //             } else if f.is_str() {
    //                 // if the value is only a string with version such as dependency = 0.0, create a new table with that version
    //                 let version = f.as_str().unwrap();
    //                 let mut map = Map::new();
    //                 let _ = map.insert("version".to_string(), Value::String(version.to_string()));
    //                 let t = Value::Table(map);
    //                 *f = t.to_owned();
    //                 f.as_table_mut()
    //             } else {
    //                 None
    //             }
    //         })
    //         .ok_or(Error::CannotParseCargoToml)?;

    //     let fuzzing = Value::String(feature.to_string());
    //     let value = Value::Array(vec![]);
    //     let features = values.entry("features").or_insert(value);
    //     if let Some(features) = features.as_array_mut() {
    //         if !features.iter().any(|f| *f == fuzzing) {
    //             features.push(fuzzing);
    //             fs::write(&cargo_toml_path, content.to_string()).await?;
    //             println!("\x1b[92mSuccesfully\x1b[0m updated: \x1b[93m{rel_path}\x1b[0m {feature} feature added.");
    //         } else {
    //             println!("\x1b[93m--> Skipping <--\x1b[0m \x1b[93m{rel_path}\x1b[0m, already contains {feature} feature.")
    //         }
    //     }
    // }

    /// ## Adds program dependency to specified Cargo.toml
    /// - for example, we need to use program entry within the fuzzer
    #[throws]
    async fn add_program_dependencies(&self, cargo_dir: &PathBuf, deps: &str) {
        let cargo_path = cargo_dir.join(CARGO);
        let mut cargo_toml_content: toml::Value = fs::read_to_string(&cargo_path).await?.parse()?;

        let client_toml_deps = cargo_toml_content
            .get_mut(deps)
            .and_then(toml::Value::as_table_mut)
            .ok_or(Error::ParsingCargoTomlDependenciesFailed)?;

        if !&self.packages.is_empty() {
            for package in self.packages.iter() {
                let manifest_path = package.manifest_path.parent().unwrap().as_std_path();
                // INFO this will obtain relative path
                // TODO fuzzer needs no entry point feature here for program client cargo.toml
                let relative_path = pathdiff::diff_paths(manifest_path, cargo_dir).unwrap();
                let dep: Value = format!(
                    r#"{} = {{ path = "{}" }}"#,
                    package.name,
                    relative_path.to_str().unwrap()
                )
                .parse()
                .unwrap();
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

    // Adds programs to Cargo.toml as a dependencies to be able to be used in tests and fuzz targets
    // TODO may be used with program stubs
    // #[throws]
    // async fn add_invoked_program_deps(&self) {
    //     let cargo_toml_path = self.root.join(TESTS_WORKSPACE_DIRECTORY).join(CARGO);

    //     match &self.idl {
    //         Some(idl) => {
    //             let mut content: Value = fs::read_to_string(&cargo_toml_path).await?.parse()?;
    //             let deps: &mut Map<String, Value> = content
    //                 .get_mut("dependencies")
    //                 .and_then(Value::as_table_mut)
    //                 .ok_or(Error::CannotParseCargoToml)?;

    //             for program in idl.programs.iter() {
    //                 for x in program.program_invocations.iter() {
    //                     if PROCESS_INSTRUCTIONS.contains_key(x.as_str()) {
    //                         let name = PROCESS_INSTRUCTIONS.get(x.as_str()).unwrap().2;
    //                         let version = PROCESS_INSTRUCTIONS.get(x.as_str()).unwrap().3;
    //                         let version = Value::String(version.to_string());
    //                         deps.entry(name).or_insert(version);
    //                     }
    //                 }
    //             }
    //             fs::write(cargo_toml_path, content.to_string()).await?;
    //         }
    //         None => {
    //             throw!(Error::NoProgramsFound)
    //         }
    //     }
    // }
    // pub fn new() -> Self {
    //     Self
    // }

    // TODO may be used with program stubs
    // #[throws]
    // async fn update_program_stubs(&self) {
    //     let program_stubs_path = self
    //         .root
    //         .join(TESTS_WORKSPACE_DIRECTORY)
    //         .join(FUZZ_DIRECTORY)
    //         .join(PROGRAM_STUBS);

    //     let program_stubs_content = include_str!(concat!(
    //         env!("CARGO_MANIFEST_DIR"),
    //         "/src/templates/trdelnik-tests/program_stubs.rs"
    //     ))
    //     .to_string();

    //     match &self.idl {
    //         Some(idl) => {
    //             let program_stubs_content = get_program_stubs_content(idl, program_stubs_content)?;
    //             self.update_file(&program_stubs_path, &program_stubs_content)
    //                 .await?;
    //         }
    //         None => {
    //             throw!(Error::NoProgramsFound)
    //         }
    //     }
    // }
}

pub fn load_template(file_path: &str) -> Result<String, std::io::Error> {
    let mut _path = String::from(MANIFEST_PATH);
    _path.push_str(file_path);
    let full_path = Path::new(&_path);

    std::fs::read_to_string(full_path)
}
