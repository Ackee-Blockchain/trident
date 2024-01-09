use crate::config::Config;
use crate::fuzzer;
use crate::snapshot_generator::generate_snapshots_code;
use crate::test_generator::ACCOUNTS_SNAPSHOTS_FILE_NAME;
use crate::{
    idl::{self, Idl},
    program_client_generator,
    test_generator::FUZZ_INSTRUCTIONS_FILE_NAME,
    test_generator::TESTS_WORKSPACE,
    Client,
};
use cargo_metadata::{MetadataCommand, Package};
use fehler::{throw, throws};
use futures::future::try_join_all;
use log::debug;
use solana_sdk::signer::keypair::Keypair;
use std::path::PathBuf;
use std::process;
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

pub const PROGRAM_CLIENT_DIRECTORY: &str = ".program_client";
pub const CARGO_TARGET_DIR_DEFAULT: &str = "trdelnik-tests/fuzz_tests/fuzzing/hfuzz_target";
pub const HFUZZ_WORKSPACE_DEFAULT: &str = "trdelnik-tests/fuzz_tests/fuzzing/hfuzz_workspace";

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
}

impl Commander {
    /// Creates a new `Commander` instance with the default root `"../../"`.
    pub fn new() -> Self {
        Self {
            root: "../../".into(),
        }
    }

    /// Creates a new `Commander` instance with the provided `root`.
    pub fn with_root(root: impl Into<Cow<'static, str>>) -> Self {
        Self { root: root.into() }
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
            .args(["--package", "poc_tests"])
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
    /// Runs fuzzer on the given target with exit code option.
    #[throws]
    pub async fn run_fuzzer_with_exit_code(&self, target: String) {
        let config = Config::new();

        let hfuzz_run_args = std::env::var("HFUZZ_RUN_ARGS").unwrap_or_default();

        let cargo_target_dir =
            std::env::var("CARGO_TARGET_DIR").unwrap_or(CARGO_TARGET_DIR_DEFAULT.to_string());

        // we read the workspace from the env variable , if not explicitly set, set it to
        // the default directory
        let hfuzz_workspace =
            std::env::var("HFUZZ_WORKSPACE").unwrap_or(HFUZZ_WORKSPACE_DEFAULT.to_string());

        let fuzz_args = config.get_fuzz_args(hfuzz_run_args);

        let (crash_dir, ext) =
            get_crash_dir_and_ext(&self.root, &target, &fuzz_args, &hfuzz_workspace);

        if let Ok(crash_files) = get_crash_files(&crash_dir, &ext) {
            if !crash_files.is_empty() {
                println!("Error: The crash directory {} already contains crash files from previous runs. \n\nTo run Trdelnik fuzzer with exit code, you must either (backup and) remove the old crash files or alternatively change the crash folder using for example the --crashdir option and the HFUZZ_RUN_ARGS env variable such as:\nHFUZZ_RUN_ARGS=\"--crashdir ./new_crash_dir\"", crash_dir.to_string_lossy());
                process::exit(1);
            }
        }

        let mut child = Command::new("cargo")
            .env("HFUZZ_RUN_ARGS", fuzz_args)
            .env("CARGO_TARGET_DIR", cargo_target_dir)
            .env("HFUZZ_WORKSPACE", hfuzz_workspace)
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

        if let Ok(crash_files) = get_crash_files(&crash_dir, &ext) {
            if !crash_files.is_empty() {
                println!(
                    "The crash directory {} contains new fuzz test crashes. Exiting!",
                    crash_dir.to_string_lossy()
                );
                process::exit(1);
            }
        }
    }

    /// Runs fuzzer on the given target.
    #[throws]
    pub async fn run_fuzzer(&self, target: String) {
        let config = Config::new();

        let hfuzz_run_args = std::env::var("HFUZZ_RUN_ARGS").unwrap_or_default();
        let cargo_target_dir =
            std::env::var("CARGO_TARGET_DIR").unwrap_or(CARGO_TARGET_DIR_DEFAULT.to_string());
        let hfuzz_workspace =
            std::env::var("HFUZZ_WORKSPACE").unwrap_or(HFUZZ_WORKSPACE_DEFAULT.to_string());

        let fuzz_args = config.get_fuzz_args(hfuzz_run_args);

        let mut child = Command::new("cargo")
            .env("HFUZZ_RUN_ARGS", fuzz_args)
            .env("CARGO_TARGET_DIR", cargo_target_dir)
            .env("HFUZZ_WORKSPACE", hfuzz_workspace)
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
        let crash_file = std::path::Path::new(&self.root as &str).join(crash_file_path);

        if !crash_file.try_exists()? {
            println!("The crash file {:?} not found!", crash_file);
            throw!(Error::CrashFileNotFound);
        }

        let cargo_target_dir =
            std::env::var("CARGO_TARGET_DIR").unwrap_or(CARGO_TARGET_DIR_DEFAULT.to_string());

        // using exec rather than spawn and replacing current process to avoid unflushed terminal output after ctrl+c signal
        std::process::Command::new("cargo")
            .env("CARGO_TARGET_DIR", cargo_target_dir)
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
        let program_idls_codes = self.program_packages().map(|package| async move {
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
                let mut libs = package.targets.iter().filter(|&t| t.is_lib());
                let lib_path = libs
                    .next()
                    .ok_or(Error::ReadProgramCodeFailed(
                        "Cannot find program library path.".into(),
                    ))?
                    .src_path
                    .clone();
                Ok((
                    idl::parse_to_idl_program(name, &code).await?,
                    (code, lib_path),
                ))
            } else {
                let error_text = String::from_utf8(output.stderr)?;
                Err(Error::ReadProgramCodeFailed(error_text))
            }
        });
        let (program_idls, codes_libs_pairs): (Vec<_>, Vec<_>) =
            try_join_all(program_idls_codes).await?.into_iter().unzip();
        let idl = Idl {
            programs: program_idls,
        };
        let use_tokens = self.parse_program_client_imports().await?;
        let program_client = program_client_generator::generate_source_code(&idl, &use_tokens);
        let program_client = Self::format_program_code(&program_client).await?;

        let program_fuzzer = fuzzer::fuzzer_generator::generate_source_code(&idl);
        let program_fuzzer = Self::format_program_code(&program_fuzzer).await?;

        let fuzzer_snapshots =
            generate_snapshots_code(codes_libs_pairs).map_err(Error::ReadProgramCodeFailed)?;
        let fuzzer_snapshots = Self::format_program_code(&fuzzer_snapshots).await?;

        // TODO do not overwrite files if they already exist to keep user changes
        let rust_file_path = Path::new(self.root.as_ref())
            .join(PROGRAM_CLIENT_DIRECTORY)
            .join("src/lib.rs");
        fs::write(rust_file_path, &program_client).await?;

        let rust_file_path = Path::new(self.root.as_ref())
            .join(TESTS_WORKSPACE)
            .join("src/")
            .join(FUZZ_INSTRUCTIONS_FILE_NAME);
        fs::write(rust_file_path, &program_fuzzer).await?;

        let rust_file_path = Path::new(self.root.as_ref())
            .join(TESTS_WORKSPACE)
            .join("src/")
            .join(ACCOUNTS_SNAPSHOTS_FILE_NAME);
        fs::write(rust_file_path, &fuzzer_snapshots).await?;
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

fn get_crash_dir_and_ext(
    root: &str,
    target: &str,
    hfuzz_run_args: &str,
    hfuzz_workspace: &str,
) -> (PathBuf, String) {
    // FIXME: we split by whitespace without respecting escaping or quotes - same approach as honggfuzz-rs so there is no point to fix it here before the upstream is fixed
    let hfuzz_run_args = hfuzz_run_args.split_whitespace();

    let extension =
        get_cmd_option_value(hfuzz_run_args.clone(), "-e", "--ext").unwrap_or("fuzz".to_string());

    // If we run fuzzer like:
    // HFUZZ_WORKSPACE="./new_hfuzz_workspace" HFUZZ_RUN_ARGS="--crashdir ./new_crash_dir -W ./new_workspace" cargo hfuzz run
    // The structure will be as follows:
    // ./new_hfuzz_workspace - will contain inputs
    // ./new_crash_dir - will contain crashes
    // ./new_workspace - will contain report
    // So finally , we have to give precedence:
    // --crashdir > --workspace > HFUZZ_WORKSPACE
    let crash_dir = get_cmd_option_value(hfuzz_run_args.clone(), "", "--cr")
        .or_else(|| get_cmd_option_value(hfuzz_run_args.clone(), "-W", "--w"));

    let crash_path = if let Some(dir) = crash_dir {
        // INFO If path is absolute, it replaces the current path.
        std::path::Path::new(root).join(dir)
    } else {
        std::path::Path::new(hfuzz_workspace).join(target)
    };

    (crash_path, extension)
}

fn get_cmd_option_value<'a>(
    hfuzz_run_args: impl Iterator<Item = &'a str>,
    short_opt: &str,
    long_opt: &str,
) -> Option<String> {
    let mut args_iter = hfuzz_run_args;
    let mut value: Option<String> = None;

    // ensure short option starts with one dash and long option with two dashes
    let short_opt = format!("-{}", short_opt.trim_start_matches('-'));
    let long_opt = format!("--{}", long_opt.trim_start_matches('-'));

    while let Some(arg) = args_iter.next() {
        match arg.strip_prefix(&short_opt) {
            Some(val) if short_opt.len() > 1 => {
                if !val.is_empty() {
                    // -ecrash for crash extension with no space
                    value = Some(val.to_string());
                } else if let Some(next_arg) = args_iter.next() {
                    // -e crash for crash extension with space
                    value = Some(next_arg.to_string());
                } else {
                    value = None;
                }
            }
            _ => {
                if arg.starts_with(&long_opt) && long_opt.len() > 2 {
                    value = args_iter.next().map(|a| a.to_string());
                }
            }
        }
    }

    value
}

fn get_crash_files(
    dir: &PathBuf,
    extension: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let paths = std::fs::read_dir(dir)?
        // Filter out all those directory entries which couldn't be read
        .filter_map(|res| res.ok())
        // Map the directory entries to paths
        .map(|dir_entry| dir_entry.path())
        // Filter out all paths with extensions other than `extension`
        .filter_map(|path| {
            if path.extension().map_or(false, |ext| ext == extension) {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cmd_options_parsing() {
        let mut command = String::from("-Q -v --extension fuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q --extension fuzz -v");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -e fuzz -v");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q --extension fuzz -v --extension ");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q --extension fuzz -v -e ");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        let mut command = String::from("--extension buzz -e fuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v -e fuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v -efuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v --ext fuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, Some("fuzz".to_string()));

        command = String::from("-Q -v --extfuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q -v --workspace");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q -v -e");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "", "--ext");
        assert_eq!(extension, None);

        command = String::from("-Q -v --ext fuzz");
        let args = command.split_whitespace();

        let extension = get_cmd_option_value(args, "-e", "");
        assert_eq!(extension, None);
    }

    #[test]
    fn test_get_crash_dir_and_ext() {
        pub const TARGET: &str = "fuzz_0";
        pub const TEST_CRASH_PATH: &str = "/home/fuzz/test-crash-path";

        const ROOT: &str = "/home/fuzz/";

        let default_crash_path = std::path::Path::new(HFUZZ_WORKSPACE_DEFAULT).join(TARGET);
        let env_specified_crash_path = std::path::Path::new(TEST_CRASH_PATH).join(TARGET);

        // this is default behavior
        let (crash_dir, ext) = get_crash_dir_and_ext(ROOT, TARGET, "", HFUZZ_WORKSPACE_DEFAULT);

        assert_eq!(crash_dir, default_crash_path);
        assert_eq!(&ext, "fuzz");

        // behavior where path is specified within env variable HFUZZ_WORKSPACE, but not within -W HFUZZ_RUN_ARGS
        let (crash_dir, ext) = get_crash_dir_and_ext(ROOT, TARGET, "-Q -e", TEST_CRASH_PATH);

        assert_eq!(crash_dir, env_specified_crash_path);
        assert_eq!(&ext, "fuzz");

        // behavior as above
        let (crash_dir, ext) = get_crash_dir_and_ext(ROOT, TARGET, "-Q -e crash", TEST_CRASH_PATH);

        assert_eq!(crash_dir, env_specified_crash_path);
        assert_eq!(&ext, "crash");

        // test absolute path
        // HFUZZ_WORKSPACE has default value however -W is set
        let (crash_dir, ext) = get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q -W /home/crash -e crash",
            HFUZZ_WORKSPACE_DEFAULT,
        );

        let expected_crash_path = std::path::Path::new("/home/crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test absolute path
        // HFUZZ_WORKSPACE is set and -W is also set
        let (crash_dir, ext) = get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q --crash /home/crash -e crash",
            TEST_CRASH_PATH,
        );
        let expected_crash_path = std::path::Path::new("/home/crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test absolute path
        // HFUZZ_WORKSPACE is set and -W is also set
        let (crash_dir, ext) = get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q --crash /home/crash/foo/bar/dead/beef -e crash",
            TEST_CRASH_PATH,
        );

        let expected_crash_path = std::path::Path::new("/home/crash/foo/bar/dead/beef");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test relative path
        // HFUZZ_WORKSPACE is set and -W is also set, this time with relative path
        let (crash_dir, ext) =
            get_crash_dir_and_ext(ROOT, TARGET, "-Q -W ../crash -e crash", TEST_CRASH_PATH);

        let expected_crash_path = std::path::Path::new(ROOT).join("../crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test relative path
        // HFUZZ_WORKSPACE is set and -W is also set, this time with relative path
        let (crash_dir, ext) = get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q -W ../../dead/beef/crash -e crash",
            TEST_CRASH_PATH,
        );

        let expected_crash_path = std::path::Path::new(ROOT).join("../../dead/beef/crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test relative path
        let (crash_dir, ext) = get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q --crash ../crash -e crash",
            HFUZZ_WORKSPACE_DEFAULT,
        );

        let expected_crash_path = std::path::Path::new(ROOT).join("../crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // crash directory has precedence before workspace option , which have precedence before
        // HFUZZ_WORKSPACE
        let (crash_dir, ext) = get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q --crash ../bitcoin/to/the/moon -W /workspace -e crash",
            TEST_CRASH_PATH,
        );

        let expected_crash_path = std::path::Path::new(ROOT).join("../bitcoin/to/the/moon");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // crash directory has precedence before workspace HFUZZ_WORKSPACE
        let (crash_dir, ext) = get_crash_dir_and_ext(
            ROOT,
            TARGET,
            "-Q --crash /home/crashes/we/like/solana -e crash",
            TEST_CRASH_PATH,
        );

        // If path is specified as absolute, the join will replace whole path.
        let expected_crash_path = std::path::Path::new("/home/crashes/we/like/solana");

        // let expected_crash_path = root.join("/home/crashes/we/like/solana");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");
    }
}
