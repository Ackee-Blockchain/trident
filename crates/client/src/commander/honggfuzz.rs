use crate::config::Config;
use fehler::{throw, throws};
use std::path::PathBuf;
use std::process;
use std::{os::unix::process::CommandExt, process::Stdio};
use tokio::process::Command;

use crate::constants::*;

use super::{get_crash_dir_and_ext, get_crash_files, Commander, Error};

impl Commander {
    /// Runs fuzzer on the given target with exit code option.
    #[throws]
    pub async fn run_honggfuzz_with_exit_code(&self, target: String) {
        let config = Config::new();

        // obtain hfuzz_run_args from env variable, this variable can contain multiple
        // arguments so we need to parse the variable content.
        let hfuzz_run_args = std::env::var("HFUZZ_RUN_ARGS").unwrap_or_default();

        let genesis_folder = PathBuf::from(self.root.to_string()).join("trident-genesis");

        let rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();

        let mut rustflags = config.get_rustflags_args(rustflags);

        rustflags.push_str("--cfg honggfuzz");

        let mut fuzz_args = config.get_honggfuzz_args(hfuzz_run_args);

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
                println!("{ERROR} The crash directory {} already contains crash files from previous runs. \n\nTo run Trident fuzzer with exit code, you must either (backup and) remove the old crash files or alternatively change the crash folder using for example the --crashdir option and the HFUZZ_RUN_ARGS env variable such as:\nHFUZZ_RUN_ARGS=\"--crashdir ./new_crash_dir\"", crash_dir.to_string_lossy());
                process::exit(1);
            }
        }

        match rustflags.contains("fuzzing_with_stats") {
            true => {
                // enforce keep output to be true
                fuzz_args.push_str("--keep_output");
                let mut child = Command::new("cargo")
                    .env("GENESIS_FOLDER", genesis_folder)
                    .env("HFUZZ_RUN_ARGS", fuzz_args)
                    .env("CARGO_TARGET_DIR", cargo_target_dir)
                    .env("HFUZZ_WORKSPACE", hfuzz_workspace)
                    .env("RUSTFLAGS", rustflags)
                    .arg("hfuzz")
                    .arg("run")
                    .arg(target)
                    .stdout(Stdio::piped())
                    .spawn()?;
                Self::handle_child_with_stats(&mut child).await?;
            }
            false => {
                let mut child = Command::new("cargo")
                    .env("GENESIS_FOLDER", genesis_folder)
                    .env("HFUZZ_RUN_ARGS", fuzz_args)
                    .env("CARGO_TARGET_DIR", cargo_target_dir)
                    .env("HFUZZ_WORKSPACE", hfuzz_workspace)
                    .env("RUSTFLAGS", rustflags)
                    .arg("hfuzz")
                    .arg("run")
                    .arg(target)
                    .spawn()?;
                Self::handle_child(&mut child).await?;
            }
        }

        if let Ok(crash_files) = get_crash_files(&crash_dir, &ext) {
            if !crash_files.is_empty() {
                println!(
                    "The crash directory {} contains new fuzz test crashes. Exiting!",
                    crash_dir.to_string_lossy()
                );
                process::exit(99);
            }
        }
    }

    /// Runs fuzzer on the given target.
    #[throws]
    pub async fn run_honggfuzz(&self, target: String) {
        let config = Config::new();

        let hfuzz_run_args = std::env::var("HFUZZ_RUN_ARGS").unwrap_or_default();

        let genesis_folder = PathBuf::from(self.root.to_string()).join("trident-genesis");

        let cargo_target_dir = std::env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| config.get_env_arg("CARGO_TARGET_DIR"));
        let hfuzz_workspace = std::env::var("HFUZZ_WORKSPACE")
            .unwrap_or_else(|_| config.get_env_arg("HFUZZ_WORKSPACE"));

        let mut fuzz_args = config.get_honggfuzz_args(hfuzz_run_args);

        let rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();

        let mut rustflags = config.get_rustflags_args(rustflags);

        rustflags.push_str("--cfg honggfuzz");

        match rustflags.contains("fuzzing_with_stats") {
            true => {
                // enforce keep output to be true
                fuzz_args.push_str("--keep_output");
                let mut child = Command::new("cargo")
                    .env("GENESIS_FOLDER", genesis_folder)
                    .env("HFUZZ_RUN_ARGS", fuzz_args)
                    .env("CARGO_TARGET_DIR", cargo_target_dir)
                    .env("HFUZZ_WORKSPACE", hfuzz_workspace)
                    .env("RUSTFLAGS", rustflags)
                    .arg("hfuzz")
                    .arg("run")
                    .arg(target)
                    .stdout(Stdio::piped())
                    .spawn()?;
                Self::handle_child_with_stats(&mut child).await?;
            }
            false => {
                let mut child = Command::new("cargo")
                    .env("GENESIS_FOLDER", genesis_folder)
                    .env("HFUZZ_RUN_ARGS", fuzz_args)
                    .env("CARGO_TARGET_DIR", cargo_target_dir)
                    .env("HFUZZ_WORKSPACE", hfuzz_workspace)
                    .env("RUSTFLAGS", rustflags)
                    .arg("hfuzz")
                    .arg("run")
                    .arg(target)
                    .spawn()?;
                Self::handle_child(&mut child).await?;
            }
        }
    }
    /// Runs fuzzer on the given target.
    #[throws]
    pub async fn run_hfuzz_debug(&self, target: String, crash_file_path: String) {
        let config = Config::new();

        let crash_file = std::path::Path::new(&self.root as &str).join(crash_file_path);

        let genesis_folder = PathBuf::from(self.root.to_string()).join("trident-genesis");

        if !crash_file.try_exists()? {
            println!("{ERROR} The crash file [{:?}] not found", crash_file);
            throw!(Error::CrashFileNotFound);
        }

        let cargo_target_dir = std::env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| config.get_env_arg("CARGO_TARGET_DIR"));

        let rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();

        let mut rustflags = config.get_rustflags_args(rustflags);

        rustflags.push_str("--cfg honggfuzz");

        // using exec rather than spawn and replacing current process to avoid unflushed terminal output after ctrl+c signal
        std::process::Command::new("cargo")
            .env("GENESIS_FOLDER", genesis_folder)
            .env("CARGO_TARGET_DIR", cargo_target_dir)
            .env("RUSTFLAGS", rustflags)
            .arg("hfuzz")
            .arg("run-debug")
            .arg(target)
            .arg(crash_file)
            .exec();

        eprintln!("cannot execute \"cargo hfuzz run-debug\" command");
    }
}
