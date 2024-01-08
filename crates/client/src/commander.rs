use std::io::Write;
use std::os::unix::process::CommandExt;

use crate::constants::*;
use crate::Client;
use crate::Config;
use crate::{Idl, IdlError};
use fehler::{throw, throws};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Io(#[from] std::io::Error),
    #[error("{0:?}")]
    Utf8(#[from] std::string::FromUtf8Error),
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
    Idl(#[from] IdlError),
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
    #[error("The Anchor project does not contain any programs")]
    NoProgramsFound,
}

/// Localnet (the validator process) handle.
pub struct LocalnetHandle {
    solana_test_validator_process: tokio::process::Child,
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
        if Client::new(solana_sdk::signature::Keypair::new())
            .is_localnet_running(false)
            .await
        {
            throw!(Error::LocalnetIsStillRunning);
        }
        log::debug!("localnet stopped");
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
        tokio::fs::remove_dir_all("test-ledger").await?;
        log::debug!("ledger removed");
    }
}

/// `Commander` allows you to start localnet, build programs,
/// run tests and do other useful operations.
pub struct Commander {}

impl Commander {
    // TODO maybe remove unnecesarry async
    #[throws]
    pub async fn build_programs() {
        let exit = std::process::Command::new("cargo")
            .arg("build-sbf")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
            .unwrap();

        if !exit.status.success() {
            std::process::exit(exit.status.code().unwrap_or(1));
        }
    }
    /// Returns an [Iterator] of program [Package]s read from `Cargo.toml` files.
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

    fn build_package(package_name: &str) -> std::process::Output {
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

    fn build_progress_bar(
        package_name: &str,
        mutex: &std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) {
        let progress_bar = indicatif::ProgressBar::new_spinner();
        progress_bar.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{spinner} {wide_msg}")
                .unwrap(),
        );

        let msg = format!("Building: {package_name}...");
        progress_bar.set_message(msg);
        while mutex.load(std::sync::atomic::Ordering::SeqCst) {
            progress_bar.inc(1);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        progress_bar.finish_and_clear();
    }

    #[throws]
    pub async fn build_program_packages(packages: &[cargo_metadata::Package]) -> Idl {
        let shared_mutex = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        for package in packages.iter() {
            let mutex = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
            let c_mutex = std::sync::Arc::clone(&mutex);

            let name = package.name.clone();
            let c_shared_mutex = std::sync::Arc::clone(&shared_mutex);
            let cargo_thread = std::thread::spawn(move || -> Result<(), Error> {
                let output = Self::build_package(&name);

                if output.status.success() {
                    let code = String::from_utf8(output.stdout).unwrap();
                    let idl_program = Idl::parse_to_idl_program(&name, &code).unwrap();
                    let mut vec = c_shared_mutex.lock().unwrap();
                    vec.push(idl_program);

                    c_mutex.store(false, std::sync::atomic::Ordering::SeqCst);
                    Ok(())
                } else {
                    let error_text = String::from_utf8(output.stderr).unwrap();
                    c_mutex.store(false, std::sync::atomic::Ordering::SeqCst);
                    Err(Error::ReadProgramCodeFailed(error_text))
                }
            });

            Self::build_progress_bar(&package.name, &mutex);
            cargo_thread.join().unwrap()?;
        }
        let idl_programs = shared_mutex.lock().unwrap().to_vec();

        if idl_programs.is_empty() {
            throw!(Error::NoProgramsFound);
        } else {
            Idl {
                programs: idl_programs,
            }
        }
    }
    #[throws]
    pub async fn clean_anchor_target() {
        // INFO perform anchor clean so no keys will be removed
        tokio::process::Command::new("anchor")
            .arg("clean")
            .spawn()?
            .wait()
            .await?;
    }
    #[throws]
    pub async fn clean_hfuzz_target(root: &std::path::Path) {
        // INFO hfuzz target can be of course located somewhere else
        // but as we leave it within the root, we also expect it within the root

        let hfuzz_target_path = root
            .join(TESTS_WORKSPACE_DIRECTORY)
            .join(FUZZ_TEST_DIRECTORY)
            .join(FUZZING)
            .join(HFUZZ_TARGET);
        if hfuzz_target_path.exists() {
            tokio::fs::remove_dir_all(hfuzz_target_path).await?;
        } else {
            println!("skipping {} directory: not found", HFUZZ_TARGET)
        }
    }

    /// Returns `use` modules / statements
    /// The goal of this method is to find all `use` statements defined by the user in the `.program_client`
    /// crate. It solves the problem with regenerating the program client and removing imports defined by
    /// the user.
    #[throws]
    pub async fn build_program_client() -> Vec<syn::ItemUse> {
        let shared_mutex = std::sync::Arc::new(std::sync::Mutex::new(String::new()));

        let mutex = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let c_mutex = std::sync::Arc::clone(&mutex);

        let c_shared_mutex = std::sync::Arc::clone(&shared_mutex);

        let cargo_thread = std::thread::spawn(move || -> Result<(), Error> {
            let output = Self::build_package("program_client");

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

        Self::build_progress_bar("program_client", &mutex);

        cargo_thread.join().unwrap()?;

        let code = shared_mutex.lock().unwrap();
        let code = code.as_str();
        let mut use_modules: Vec<syn::ItemUse> = vec![];
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
        if use_modules.is_empty() {
            use_modules.push(syn::parse_quote! { use trdelnik_client::prelude::*; })
        }
        use_modules
    }

    /// Runs standard Rust tests.
    ///
    /// _Note_: The [--nocapture](https://doc.rust-lang.org/cargo/commands/cargo-test.html#display-options) argument is used
    /// to allow you read `println` outputs in your terminal window.
    #[throws]
    pub async fn run_tests() {
        // no capture is blocking color format of the command,
        // I do not see difference in functionality without it
        // however without nocapture , debugging with println! is not possible
        let success = tokio::process::Command::new("cargo")
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

    /// Runs fuzzer on the given target.
    #[throws]
    pub async fn run_fuzzer(target: String) {
        // INFO we do not check anything here , as we leave it on honggfuzz , simply
        // if the target does not exists, honggfuzz will throw an error
        let config = Config::new();

        // INFO This was tested on independant hongfuzz example, but if we specify HFUZZ_WORKSPACE,
        // it will update the workspace directory , however HFUZZ_RUN_ARGS will still have stronger
        // word and will be used for report and crash files if specified

        let hfuzz_run_args = std::env::var("HFUZZ_RUN_ARGS").unwrap_or_default();

        let cargo_target_dir =
            std::env::var("CARGO_TARGET_DIR").unwrap_or(CARGO_TARGET_DIR_DEFAULT.to_string());
        let hfuzz_workspace =
            std::env::var("HFUZZ_WORKSPACE").unwrap_or(HFUZZ_WORKSPACE_DEFAULT.to_string());

        let fuzz_args = config.get_fuzz_args(hfuzz_run_args);

        let mut child = tokio::process::Command::new("cargo")
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
            _ = tokio::signal::ctrl_c() => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            },
        }
    }
    #[throws]
    pub async fn run_fuzzer_with_exit_code(target: String, root: String) {
        let root = std::path::Path::new(&root);

        // obtain config data
        let config = Config::new();
        // obtain hfuzz_run_args
        let hfuzz_run_args = std::env::var("HFUZZ_RUN_ARGS").unwrap_or_default();

        let cargo_target_dir =
            std::env::var("CARGO_TARGET_DIR").unwrap_or(CARGO_TARGET_DIR_DEFAULT.to_string());

        let hfuzz_workspace =
            std::env::var("HFUZZ_WORKSPACE").unwrap_or(HFUZZ_WORKSPACE_DEFAULT.to_string());

        // obtain string from config and hfuzz_run_args
        let fuzz_args = config.get_fuzz_args(hfuzz_run_args);
        // Parse the fuzz_args arguments to find out if the crash folder and crash files extension was modified.
        // This will give precedence to latter
        let (crash_dir, ext) = get_crash_dir_and_ext(root, &target, &fuzz_args, &hfuzz_workspace);

        if let Ok(crash_files) = get_crash_files(&crash_dir, &ext) {
            if !crash_files.is_empty() {
                println!("Error: The crash directory {} already contains crash files from previous runs. \n\nTo run Trdelnik fuzzer with exit code, you must either (backup and) remove the old crash files or alternatively change the crash folder using for example the --crashdir option and the HFUZZ_RUN_ARGS env variable such as:\nHFUZZ_RUN_ARGS=\"--crashdir ./new_crash_dir\"", crash_dir.to_string_lossy());
                std::process::exit(1);
            }
        }

        let mut child = tokio::process::Command::new("cargo")
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
            _ = tokio::signal::ctrl_c() => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            },
        }

        if let Ok(crash_files) = get_crash_files(&crash_dir, &ext) {
            if !crash_files.is_empty() {
                println!(
                    "The crash directory {} contains new fuzz test crashes. Exiting!",
                    crash_dir.to_string_lossy()
                );
                std::process::exit(1);
            }
        }
    }

    /// Runs fuzzer on the given target.
    #[throws]
    pub async fn run_fuzzer_debug(target: String, crash_file_path: String, root: String) {
        let root = std::path::Path::new(&root);

        let cur_dir = root.join(TESTS_WORKSPACE_DIRECTORY);
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

    /// Starts the localnet (Solana validator).
    #[throws]
    pub async fn start_localnet(root: &String) -> LocalnetHandle {
        let mut process = tokio::process::Command::new("solana-test-validator")
            .arg("-C")
            .arg([root, "config.yml"].concat())
            .arg("-r")
            .arg("-q")
            .spawn()?;

        if !Client::new(solana_sdk::signature::Keypair::new())
            .is_localnet_running(true)
            .await
        {
            // The validator might not be running, but the process might be still alive (very slow start, some bug, ...),
            // therefore we want to kill it if it's still running so ports aren't held.
            process.kill().await.ok();
            throw!(Error::LocalnetIsNotRunning);
        }
        log::debug!("localnet started");
        LocalnetHandle {
            solana_test_validator_process: process,
        }
    }

    /// Formats program code.
    #[throws]
    pub async fn format_program_code(code: &str) -> String {
        let mut rustfmt = std::process::Command::new("rustfmt")
            .args(["--edition", "2018"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        if let Some(stdio) = &mut rustfmt.stdin {
            stdio.write_all(code.as_bytes())?;
        }
        let output = rustfmt.wait_with_output()?;
        String::from_utf8(output.stdout)?
    }
}

fn get_crash_dir_and_ext(
    root: &std::path::Path,
    target: &str,
    hfuzz_run_args: &str,
    hfuzz_workspace: &str,
) -> (std::path::PathBuf, String) {
    // FIXME: we split by whitespace without respecting escaping or quotes - same approach as honggfuzz-rs so there is no point to fix it here before the upstream is fixed
    let hfuzz_run_args = hfuzz_run_args.split_whitespace();

    let extension =
        get_cmd_option_value(hfuzz_run_args.clone(), "-e", "--ext").unwrap_or("fuzz".to_string());

    let crash_dir = get_cmd_option_value(hfuzz_run_args.clone(), "", "--cr")
        .or_else(|| get_cmd_option_value(hfuzz_run_args.clone(), "-W", "--w"));

    //INFO -W is stronger option when honggfuzz is executed, so if -W is specified
    // we take it as crashdir, if not we take hfuzz_workspace which is set by user
    // or default is set to fuzzing within trdelink-tests
    let crash_path = if let Some(dir) = crash_dir {
        std::path::Path::new(root).join(dir)
    } else {
        std::path::Path::new(hfuzz_workspace).join(target)
    };

    (crash_path, extension)
}

fn get_crash_files(
    dir: &std::path::PathBuf,
    extension: &str,
) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
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

        let root = std::path::Path::new("/home/fuzz/");

        let default_crash_path = std::path::Path::new(HFUZZ_WORKSPACE_DEFAULT).join(TARGET);
        let env_specified_crash_path = std::path::Path::new(TEST_CRASH_PATH).join(TARGET);

        // this is default behavior
        let (crash_dir, ext) = get_crash_dir_and_ext(root, TARGET, "", HFUZZ_WORKSPACE_DEFAULT);

        assert_eq!(crash_dir, default_crash_path);
        assert_eq!(&ext, "fuzz");

        // behavior where path is specified within env variable HFUZZ_WORKSPACE, but not within -W HFUZZ_RUN_ARGS
        let (crash_dir, ext) = get_crash_dir_and_ext(root, TARGET, "-Q -e", TEST_CRASH_PATH);

        assert_eq!(crash_dir, env_specified_crash_path);
        assert_eq!(&ext, "fuzz");

        // behavior as above
        let (crash_dir, ext) = get_crash_dir_and_ext(root, TARGET, "-Q -e crash", TEST_CRASH_PATH);

        assert_eq!(crash_dir, env_specified_crash_path);
        assert_eq!(&ext, "crash");

        // test absolute path
        // HFUZZ_WORKSPACE has default value however -W is set
        let (crash_dir, ext) = get_crash_dir_and_ext(
            root,
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
            root,
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
            root,
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
            get_crash_dir_and_ext(root, TARGET, "-Q -W ../crash -e crash", TEST_CRASH_PATH);

        let expected_crash_path = root.join("../crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test relative path
        // HFUZZ_WORKSPACE is set and -W is also set, this time with relative path
        let (crash_dir, ext) = get_crash_dir_and_ext(
            root,
            TARGET,
            "-Q -W ../../dead/beef/crash -e crash",
            TEST_CRASH_PATH,
        );

        let expected_crash_path = root.join("../../dead/beef/crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // test relative path
        let (crash_dir, ext) = get_crash_dir_and_ext(
            root,
            TARGET,
            "-Q --crash ../crash -e crash",
            HFUZZ_WORKSPACE_DEFAULT,
        );

        let expected_crash_path = root.join("../crash");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // crash directory has precedence before workspace option , which have precedence before
        // HFUZZ_WORKSPACE
        let (crash_dir, ext) = get_crash_dir_and_ext(
            root,
            TARGET,
            "-Q --crash ../bitcoin/to/the/moon -W /workspace -e crash",
            TEST_CRASH_PATH,
        );

        let expected_crash_path = root.join("../bitcoin/to/the/moon");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");

        // crash directory has precedence before workspace HFUZZ_WORKSPACE
        let (crash_dir, ext) = get_crash_dir_and_ext(
            root,
            TARGET,
            "-Q --crash /home/crashes/we/like/solana -e crash",
            TEST_CRASH_PATH,
        );

        let expected_crash_path = root.join("/home/crashes/we/like/solana");
        assert_eq!(crash_dir, expected_crash_path);
        assert_eq!(&ext, "crash");
    }
}
