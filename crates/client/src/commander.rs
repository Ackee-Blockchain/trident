use crate::config::Config;
use crate::{
    idl::{self, Idl},
    program_client_generator,
    test_generator::TESTS_WORKSPACE,
    Client,
};
use cargo_metadata::{MetadataCommand, Package};
use fehler::{throw, throws};
use futures::future::try_join_all;
use log::debug;
use solana_sdk::signer::keypair::Keypair;
use std::{
    borrow::Cow, io, iter, os::unix::process::CommandExt, path::Path, process::Stdio,
    string::FromUtf8Error,
};
use thiserror::Error;
use tokio::{
    fs,
    io::AsyncWriteExt,
    process::{Child, Command},
    signal,
};

pub static PROGRAM_CLIENT_DIRECTORY: &str = ".program_client";

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    Utf8(#[from] FromUtf8Error),
    #[error("localnet is not running")]
    LocalnetIsNotRunning,
    #[error("localnet is still running")]
    LocalnetIsStillRunning,
    #[error("build programs failed")]
    BuildProgramsFailed,
    #[error("testing failed")]
    TestingFailed,
    #[error("read program code failed: '{0}'")]
    ReadProgramCodeFailed(String),
    #[error("{0:?}")]
    Idl(#[from] idl::Error),
    #[error("{0:?}")]
    TomlDeserialize(#[from] toml::de::Error),
    #[error("parsing Cargo.toml dependencies failed")]
    ParsingCargoTomlDependenciesFailed,
    #[error("fuzzing failed")]
    FuzzingFailed,
    #[error("Trdelnik it not correctly initialized! The trdelnik-tests folder in the root of your project does not exist")]
    NotInitialized,
    #[error("the crash file does not exist")]
    CrashFileNotFound,
}

/// Localnet (the validator process) handle.
pub struct LocalnetHandle {
    solana_test_validator_process: Child,
}

impl LocalnetHandle {
    /// Stops the localnet.
    ///
    /// _Note_: Manual kill: `kill -9 $(lsof -t -i:8899)`
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - killing the process failed.
    /// - process is still running after the kill command has been performed.
    #[throws]
    pub async fn stop(mut self) {
        self.solana_test_validator_process.kill().await?;
        if Client::new(Keypair::new()).is_localnet_running(false).await {
            throw!(Error::LocalnetIsStillRunning);
        }
        debug!("localnet stopped");
    }

    /// Stops the localnet and removes the ledger.
    ///
    /// _Note_: Manual kill: `kill -9 $(lsof -t -i:8899)`
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - killing the process failed.
    /// - process is still running after the kill command has been performed.
    /// - cannot remove localnet data (the `test-ledger` directory).
    #[throws]
    pub async fn stop_and_remove_ledger(self) {
        self.stop().await?;
        fs::remove_dir_all("test-ledger").await?;
        debug!("ledger removed");
    }
}

/// `Commander` allows you to start localnet, build programs,
/// run tests and do other useful operations.
pub struct Commander {
    root: Cow<'static, str>,
    config: Config,
}

impl Commander {
    /// Creates a new `Commander` instance with the default root `"../../"`.
    pub fn new() -> Self {
        Self {
            root: "../../".into(),
            config: Config::new(),
        }
    }

    /// Creates a new `Commander` instance with the provided `root`.
    pub fn with_root(root: impl Into<Cow<'static, str>>) -> Self {
        Self {
            root: root.into(),
            config: Config::new(),
        }
    }

    /// Builds programs (smart contracts).
    #[throws]
    pub async fn build_programs(&self) {
        let success = Command::new("cargo")
            .arg("build-bpf")
            .arg("--")
            // prevent prevent dependency loop:
            // program tests -> program_client -> program
            .args(["-Z", "avoid-dev-deps"])
            .spawn()?
            .wait()
            .await?
            .success();
        if !success {
            throw!(Error::BuildProgramsFailed);
        }
    }

    /// Runs standard Rust tests.
    ///
    /// _Note_: The [--nocapture](https://doc.rust-lang.org/cargo/commands/cargo-test.html#display-options) argument is used
    /// to allow you read `println` outputs in your terminal window.
    #[throws]
    pub async fn run_tests(&self) {
        let success = Command::new("cargo")
            .arg("test")
            .arg("--")
            .arg("--nocapture")
            .spawn()?
            .wait()
            .await?
            .success();
        if !success {
            throw!(Error::TestingFailed);
        }
    }
    /// Runs fuzzer on the given target.
    #[throws]
    pub async fn run_fuzzer(&mut self, target: String) {
        if let Ok(var) = std::env::var("HFUZZ_RUN_ARGS") {
            self.config.merge_with_cli(&var)
        }

        let env_variables = self.config.get_env_variables();

        let cur_dir = Path::new(&self.root.to_string()).join(TESTS_WORKSPACE);
        if !cur_dir.try_exists()? {
            throw!(Error::NotInitialized);
        }

        println!("{:?}", env_variables);
        let mut child = Command::new("cargo")
            .env("HFUZZ_RUN_ARGS", env_variables)
            .current_dir(cur_dir)
            .arg("hfuzz")
            .arg("run")
            .arg(target)
            .spawn()?;

        tokio::select! {
            res = child.wait() =>
                match res {
                    Ok(status) => if !status.success() {
                        println!("Honggfuzz exited with an error!");
                    },
                    Err(_) => throw!(Error::FuzzingFailed),
            },
            _ = signal::ctrl_c() => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            },
        }
    }

    /// Runs fuzzer on the given target.
    #[throws]
    pub async fn run_fuzzer_debug(&self, target: String, crash_file_path: String) {
        let cur_dir = Path::new(&self.root.to_string()).join(TESTS_WORKSPACE);
        let crash_file = std::env::current_dir()?.join(crash_file_path);

        if !cur_dir.try_exists()? {
            throw!(Error::NotInitialized);
        }

        if !crash_file.try_exists()? {
            println!("The crash file {:?} not found!", crash_file);
            throw!(Error::CrashFileNotFound);
        }

        // using exec rather than spawn and replacing current process to avoid unflushed terminal output after ctrl+c signal
        std::process::Command::new("cargo")
            .current_dir(cur_dir)
            .arg("hfuzz")
            .arg("run-debug")
            .arg(target)
            .arg(crash_file)
            .exec();

        eprintln!("cannot execute \"cargo hfuzz run-debug\" command");
    }

    /// Creates the `program_client` crate.
    ///
    /// It's used internally by the [`#[trdelnik_test]`](trdelnik_test::trdelnik_test) macro.
    #[throws]
    pub async fn create_program_client_crate(&self) {
        let crate_path = Path::new(self.root.as_ref()).join(PROGRAM_CLIENT_DIRECTORY);
        if fs::metadata(&crate_path).await.is_ok() {
            return;
        }

        // @TODO Would it be better to:
        // zip the template folder -> embed the archive to the binary -> unzip to a given location?

        fs::create_dir(&crate_path).await?;

        let cargo_toml_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/program_client/Cargo.toml.tmpl"
        ));
        fs::write(crate_path.join("Cargo.toml"), &cargo_toml_content).await?;

        let src_path = crate_path.join("src");
        fs::create_dir(&src_path).await?;

        let lib_rs_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/templates/program_client/lib.rs"
        ));
        fs::write(src_path.join("lib.rs"), &lib_rs_content).await?;

        debug!("program_client crate created")
    }

    /// Returns an [Iterator] of program [Package]s read from `Cargo.toml` files.
    pub fn program_packages(&self) -> impl Iterator<Item = Package> {
        let cargo_toml_data = MetadataCommand::new()
            .no_deps()
            .exec()
            .expect("Cargo.toml reading failed");

        cargo_toml_data.packages.into_iter().filter(|package| {
            // @TODO less error-prone test if the package is a _program_?
            if let Some("programs") = package.manifest_path.iter().nth_back(2) {
                return true;
            }
            false
        })
    }

    /// Updates the `program_client` dependencies.
    ///
    /// It's used internally by the [`#[trdelnik_test]`](trdelnik_test::trdelnik_test) macro.
    #[throws]
    pub async fn generate_program_client_deps(&self) {
        let trdelnik_dep = r#"trdelnik-client = "0.5.0""#.parse().unwrap();
        // @TODO replace the line above with the specific version or commit hash
        // when Trdelnik is released or when its repo is published.
        // Or use both variants - path for Trdelnik repo/dev and version/commit for users.
        // Some related snippets:
        //
        // println!("Trdelnik Version: {}", std::env!("VERGEN_BUILD_SEMVER"));
        // println!("Trdelnik Commit: {}", std::env!("VERGEN_GIT_SHA"));
        // https://docs.rs/vergen/latest/vergen/#environment-variables
        //
        // `trdelnik = "0.1.0"`
        // `trdelnik = { git = "https://github.com/Ackee-Blockchain/trdelnik.git", rev = "cf867aea87e67d7be029982baa39767f426e404d" }`

        let absolute_root = fs::canonicalize(self.root.as_ref()).await?;

        let program_deps = self.program_packages().map(|package| {
            let name = package.name;
            let path = package
                .manifest_path
                .parent()
                .unwrap()
                .strip_prefix(&absolute_root)
                .unwrap();
            format!(r#"{name} = {{ path = "../{path}", features = ["no-entrypoint"] }}"#)
                .parse()
                .unwrap()
        });

        let cargo_toml_path = Path::new(self.root.as_ref())
            .join(PROGRAM_CLIENT_DIRECTORY)
            .join("Cargo.toml");

        let mut cargo_toml_content: toml::Value =
            fs::read_to_string(&cargo_toml_path).await?.parse()?;

        let cargo_toml_deps = cargo_toml_content
            .get_mut("dependencies")
            .and_then(toml::Value::as_table_mut)
            .ok_or(Error::ParsingCargoTomlDependenciesFailed)?;

        for dep in iter::once(trdelnik_dep).chain(program_deps) {
            if let toml::Value::Table(table) = dep {
                let (name, value) = table.into_iter().next().unwrap();
                cargo_toml_deps.entry(name).or_insert(value);
            }
        }

        // @TODO remove renamed or deleted programs from deps?

        fs::write(cargo_toml_path, cargo_toml_content.to_string()).await?;
    }

    /// Updates the `program_client` `lib.rs`.
    ///
    /// It's used internally by the [`#[trdelnik_test]`](trdelnik_test::trdelnik_test) macro.
    #[throws]
    pub async fn generate_program_client_lib_rs(&self) {
        let idl_programs = self.program_packages().map(|package| async move {
            let name = package.name;
            let output = Command::new("cargo")
                .arg("+nightly")
                .arg("rustc")
                .args(["--package", &name])
                .arg("--profile=check")
                .arg("--")
                .arg("-Zunpretty=expanded")
                .output()
                .await?;
            if output.status.success() {
                let code = String::from_utf8(output.stdout)?;
                Ok(idl::parse_to_idl_program(name, &code).await?)
            } else {
                let error_text = String::from_utf8(output.stderr)?;
                Err(Error::ReadProgramCodeFailed(error_text))
            }
        });
        let idl = Idl {
            programs: try_join_all(idl_programs).await?,
        };
        let use_tokens = self.parse_program_client_imports().await?;
        let program_client = program_client_generator::generate_source_code(idl, &use_tokens);
        let program_client = Self::format_program_code(&program_client).await?;

        let rust_file_path = Path::new(self.root.as_ref())
            .join(PROGRAM_CLIENT_DIRECTORY)
            .join("src/lib.rs");
        fs::write(rust_file_path, &program_client).await?;
    }

    /// Formats program code.
    #[throws]
    pub async fn format_program_code(code: &str) -> String {
        let mut rustfmt = Command::new("rustfmt")
            .args(["--edition", "2018"])
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        if let Some(stdio) = &mut rustfmt.stdin {
            stdio.write_all(code.as_bytes()).await?;
        }
        let output = rustfmt.wait_with_output().await?;
        String::from_utf8(output.stdout)?
    }

    /// Starts the localnet (Solana validator).
    #[throws]
    pub async fn start_localnet(&self) -> LocalnetHandle {
        let mut process = Command::new("solana-test-validator")
            .arg("-C")
            .arg([&self.root, "config.yml"].concat())
            .arg("-r")
            .arg("-q")
            .spawn()?;

        if !Client::new(Keypair::new()).is_localnet_running(true).await {
            // The validator might not be running, but the process might be still alive (very slow start, some bug, ...),
            // therefore we want to kill it if it's still running so ports aren't held.
            process.kill().await.ok();
            throw!(Error::LocalnetIsNotRunning);
        }
        debug!("localnet started");
        LocalnetHandle {
            solana_test_validator_process: process,
        }
    }

    /// Returns `use` modules / statements
    /// The goal of this method is to find all `use` statements defined by the user in the `.program_client`
    /// crate. It solves the problem with regenerating the program client and removing imports defined by
    /// the user.
    #[throws]
    pub async fn parse_program_client_imports(&self) -> Vec<syn::ItemUse> {
        let output = Command::new("cargo")
            .arg("+nightly")
            .arg("rustc")
            .args(["--package", "program_client"])
            .arg("--profile=check")
            .arg("--")
            .arg("-Zunpretty=expanded")
            .output()
            .await?;
        let code = String::from_utf8(output.stdout)?;
        let mut use_modules: Vec<syn::ItemUse> = vec![];
        for item in syn::parse_file(code.as_str()).unwrap().items.into_iter() {
            if let syn::Item::Mod(module) = item {
                let modules = module
                    .content
                    .ok_or("account mod: empty content")
                    .unwrap()
                    .1
                    .into_iter();
                for module in modules {
                    if let syn::Item::Use(u) = module {
                        use_modules.push(u);
                    }
                }
            }
        }
        if use_modules.is_empty() {
            use_modules.push(syn::parse_quote! { use trdelnik_client::*; })
        }
        use_modules
    }
}

impl Default for Commander {
    /// Creates a new `Commander` instance with the default root `"../../"`.
    fn default() -> Self {
        Self::new()
    }
}
