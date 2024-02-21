use crate::config::Config;
use crate::{
    idl::{self, Idl},
    Client,
};
use fehler::{throw, throws};
use log::debug;
use solana_sdk::signer::keypair::Keypair;
use std::path::PathBuf;
use std::process;
use std::{borrow::Cow, io, os::unix::process::CommandExt, process::Stdio, string::FromUtf8Error};
use thiserror::Error;
use tokio::{
    fs,
    io::AsyncWriteExt,
    process::{Child, Command},
    signal,
};

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
    #[error("The Solana project does not contain any programs")]
    NoProgramsFound,
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

        // obtain hfuzz_run_args from env variable, this variable can contain multiple
        // arguments so we need to parse the variable content.
        let hfuzz_run_args = std::env::var("HFUZZ_RUN_ARGS").unwrap_or_default();

        let fuzz_args = config.get_fuzz_args(hfuzz_run_args);

        // let cargo_target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_default();

        // obtain cargo_target_dir, as this variable contains only 1 string
        // which corresponds to desired path, we can compare it to the Config
        // the default/desired value is set inside Config, however variable entered
        // form CLI has always precedence
        let cargo_target_dir = std::env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| config.get_env_arg("CARGO_TARGET_DIR"));

        // obtain hfuzz_workspace, as this variable contains only 1 string
        // which corresponds to desired path, we can compare it to the Config
        // the default/desired value is set inside Config, however variable entered
        // form CLI has always precedence
        let hfuzz_workspace = std::env::var("HFUZZ_WORKSPACE")
            .unwrap_or_else(|_| config.get_env_arg("HFUZZ_WORKSPACE"));

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

        let cargo_target_dir = std::env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| config.get_env_arg("CARGO_TARGET_DIR"));
        let hfuzz_workspace = std::env::var("HFUZZ_WORKSPACE")
            .unwrap_or_else(|_| config.get_env_arg("HFUZZ_WORKSPACE"));

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
        let config = Config::new();

        let crash_file = std::path::Path::new(&self.root as &str).join(crash_file_path);

        if !crash_file.try_exists()? {
            println!("The crash file {:?} not found!", crash_file);
            throw!(Error::CrashFileNotFound);
        }

        let cargo_target_dir = std::env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| config.get_env_arg("CARGO_TARGET_DIR"));

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
    pub fn program_packages() -> impl Iterator<Item = cargo_metadata::Package> {
        let cargo_toml_data = cargo_metadata::MetadataCommand::new()
            .no_deps()
            .exec()
            .expect("Cargo.toml reading failed");

        cargo_toml_data.packages.into_iter().filter(|package| {
            // TODO less error-prone test if the package is a _program_?
            // This will only consider Packages where path:
            // /home/xyz/xyz/trdelnik/trdelnik/examples/example_project/programs/package1
            // NOTE we can obtain more important information here, only to remember
            if let Some("programs") = package.manifest_path.iter().nth_back(2) {
                return true;
            }
            false
        })
    }

    #[throws]
    pub async fn collect_program_packages() -> Vec<cargo_metadata::Package> {
        let packages: Vec<cargo_metadata::Package> = Commander::program_packages().collect();
        if packages.is_empty() {
            throw!(Error::NoProgramsFound)
        } else {
            packages
        }
    }
    fn expand_progress_bar(
        package_name: &str,
        mutex: &std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) {
        let progress_bar = indicatif::ProgressBar::new_spinner();
        progress_bar.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{spinner} {wide_msg}")
                .unwrap(),
        );

        let msg = format!("Expanding: {package_name}... this may take a while");
        progress_bar.set_message(msg);
        while mutex.load(std::sync::atomic::Ordering::SeqCst) {
            progress_bar.inc(1);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        progress_bar.finish_and_clear();
    }
    #[throws]
    pub async fn expand_program_packages(
        packages: &[cargo_metadata::Package],
    ) -> (Idl, Vec<(String, cargo_metadata::camino::Utf8PathBuf)>) {
        let shared_mutex = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let shared_mutex_fuzzer = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

        for package in packages.iter() {
            let mutex = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
            let c_mutex = std::sync::Arc::clone(&mutex);

            let name = package.name.clone();

            let mut libs = package.targets.iter().filter(|&t| t.is_lib());
            let lib_path = libs
                .next()
                .ok_or(Error::ReadProgramCodeFailed(
                    "Cannot find program library path.".into(),
                ))?
                .src_path
                .clone();

            let c_shared_mutex = std::sync::Arc::clone(&shared_mutex);
            let c_shared_mutex_fuzzer = std::sync::Arc::clone(&shared_mutex_fuzzer);

            let cargo_thread = std::thread::spawn(move || -> Result<(), Error> {
                let output = Self::expand_package(&name);

                if output.status.success() {
                    let code = String::from_utf8(output.stdout).unwrap();

                    let idl_program = idl::parse_to_idl_program(name, &code)?;
                    let mut vec = c_shared_mutex.lock().unwrap();
                    let mut vec_fuzzer = c_shared_mutex_fuzzer.lock().unwrap();

                    vec.push(idl_program);
                    vec_fuzzer.push((code, lib_path));

                    c_mutex.store(false, std::sync::atomic::Ordering::SeqCst);
                    Ok(())
                } else {
                    let error_text = String::from_utf8(output.stderr).unwrap();
                    c_mutex.store(false, std::sync::atomic::Ordering::SeqCst);
                    Err(Error::ReadProgramCodeFailed(error_text))
                }
            });

            Self::expand_progress_bar(&package.name, &mutex);
            cargo_thread.join().unwrap()?;
        }
        let idl_programs = shared_mutex.lock().unwrap().to_vec();
        let codes_libs_pairs = shared_mutex_fuzzer.lock().unwrap().to_vec();

        if idl_programs.is_empty() {
            throw!(Error::NoProgramsFound);
        } else {
            (
                Idl {
                    programs: idl_programs,
                },
                codes_libs_pairs,
            )
        }
    }
    fn expand_package(package_name: &str) -> std::process::Output {
        std::process::Command::new("cargo")
            .arg("+nightly")
            .arg("rustc")
            .args(["--package", package_name])
            .arg("--profile=check")
            .arg("--")
            .arg("-Zunpretty=expanded")
            .output()
            .unwrap()
    }

    /// Returns `use` modules / statements
    /// The goal of this method is to find all `use` statements defined by the user in the `.program_client`
    /// crate. It solves the problem with regenerating the program client and removing imports defined by
    /// the user.
    #[throws]
    pub async fn expand_program_client() -> Vec<syn::ItemUse> {
        let shared_mutex = std::sync::Arc::new(std::sync::Mutex::new(String::new()));

        let mutex = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let c_mutex = std::sync::Arc::clone(&mutex);

        let c_shared_mutex = std::sync::Arc::clone(&shared_mutex);

        let cargo_thread = std::thread::spawn(move || -> Result<(), Error> {
            let output = Self::expand_package("program_client");

            if output.status.success() {
                let mut code = c_shared_mutex.lock().unwrap();
                code.push_str(&String::from_utf8(output.stdout)?);

                c_mutex.store(false, std::sync::atomic::Ordering::SeqCst);
                Ok(())
            } else {
                // command failed leave unmodified
                c_mutex.store(false, std::sync::atomic::Ordering::SeqCst);
                Ok(())
            }
        });

        Self::expand_progress_bar("program_client", &mutex);

        cargo_thread.join().unwrap()?;

        let code = shared_mutex.lock().unwrap();
        let code = code.as_str();
        let mut use_modules: Vec<syn::ItemUse> = vec![];
        if code.is_empty() {
            use_modules.push(syn::parse_quote! { use trdelnik_client::*; })
        } else {
            Self::get_use_statements(code, &mut use_modules)?;
            if use_modules.is_empty() {
                use_modules.push(syn::parse_quote! { use trdelnik_client::*; })
            }
        }
        use_modules
    }

    #[throws]
    pub fn get_use_statements(code: &str, use_modules: &mut Vec<syn::ItemUse>) {
        for item in syn::parse_file(code).unwrap().items.into_iter() {
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
    pub const HFUZZ_WORKSPACE_DEFAULT: &str = "trdelnik-tests/fuzz_tests/fuzzing/hfuzz_workspace";
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
