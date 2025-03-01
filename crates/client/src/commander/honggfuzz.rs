use fehler::{throw, throws};
use std::path::Path;
use std::process;
use std::process::Stdio;
use tokio::process::Command;

use trident_config::TridentConfig;

use crate::constants::*;

use super::{get_crash_dir_and_ext, get_crash_files, Commander, Error};

impl Commander {
    /// Runs fuzzer on the given target with exit code option.
    #[throws]
    pub async fn run_honggfuzz_with_exit_code(&self, target: String) {
        let config = TridentConfig::new();

        // obtain hfuzz_run_args from env variable, this variable can contain multiple
        // arguments so we need to parse the variable content.
        let hfuzz_run_args = std::env::var("HFUZZ_RUN_ARGS").unwrap_or_default();

        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();

        rustflags.push_str("--cfg honggfuzz");

        let mut fuzz_args = config.get_honggfuzz_args(hfuzz_run_args);

        let cargo_target_dir =
            std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| config.get_honggfuzz_target_dir());

        let hfuzz_workspace =
            std::env::var("HFUZZ_WORKSPACE").unwrap_or_else(|_| config.get_honggfuzz_workspace());

        let (crash_dir, ext) =
            get_crash_dir_and_ext(&self.root, &target, &fuzz_args, &hfuzz_workspace);

        if let Ok(crash_files) = get_crash_files(&crash_dir, &ext) {
            if !crash_files.is_empty() {
                println!("{ERROR} The crash directory {} already contains crash files from previous runs. \n\nTo run Trident fuzzer with exit code, you must either (backup and) remove the old crash files or alternatively change the crash folder using for example the --crashdir option and the HFUZZ_RUN_ARGS env variable such as:\nHFUZZ_RUN_ARGS=\"--crashdir ./new_crash_dir\"", crash_dir.to_string_lossy());
                process::exit(1);
            }
        }

        match config.get_fuzzing_with_stats() {
            true => {
                // enforce keep output to be true
                fuzz_args.push_str("--keep_output");
                std::env::set_var("FUZZING_METRICS", "1");

                let mut child = Command::new("cargo")
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
        let config = TridentConfig::new();

        let hfuzz_run_args = std::env::var("HFUZZ_RUN_ARGS").unwrap_or_default();

        let cargo_target_dir =
            std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| config.get_honggfuzz_target_dir());
        let hfuzz_workspace =
            std::env::var("HFUZZ_WORKSPACE").unwrap_or_else(|_| config.get_honggfuzz_workspace());

        let mut fuzz_args = config.get_honggfuzz_args(hfuzz_run_args);

        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();

        rustflags.push_str("--cfg honggfuzz");

        match config.get_fuzzing_with_stats() {
            true => {
                // enforce keep output to be true
                std::env::set_var("FUZZING_METRICS", "1");
                fuzz_args.push_str("--keep_output");
                let mut child = Command::new("cargo")
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
        let config = TridentConfig::new();

        let crash_file = Path::new(&crash_file_path);

        let crash_file = if crash_file.is_absolute() {
            crash_file
        } else {
            let cwd = std::env::current_dir()?;

            &cwd.join(crash_file)
        };

        if !crash_file.try_exists()? {
            println!("{ERROR} The crash file [{:?}] not found", crash_file);
            throw!(Error::CrashFileNotFound);
        }

        let cargo_target_dir =
            std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| config.get_honggfuzz_target_dir());

        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();

        rustflags.push_str("--cfg honggfuzz_debug --cfg fuzzing_debug");

        // using exec rather than spawn and replacing current process to avoid unflushed terminal output after ctrl+c signal
        let mut child = tokio::process::Command::new("cargo")
            .env("TRIDENT_LOG", "1")
            .env("CARGO_TARGET_DIR", cargo_target_dir)
            .env("RUSTFLAGS", rustflags)
            .arg("run")
            .arg("--bin")
            .arg(target)
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(crash_file_path.as_bytes()).await?;
        }

        Self::handle_child(&mut child).await?;
    }
}
