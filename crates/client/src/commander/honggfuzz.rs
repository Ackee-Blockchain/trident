use fehler::{throw, throws};
use std::collections::HashMap;
use std::path::Path;
use std::process;
use std::process::Stdio;
use tokio::process::Command;

use crate::constants::*;
use crate::coverage::{honggfuzz::HonggfuzzCoverage, Coverage};
use trident_config::TridentConfig;

use super::{get_crash_dir_and_ext, get_crash_files, Commander, Error};

impl Commander {
    /// Runs fuzzer on the given target.
    #[throws]
    pub async fn run_honggfuzz(&self, target: String, exit_code: bool, generate_coverage: bool) {
        let config = TridentConfig::new();

        if generate_coverage {
            let coverage = HonggfuzzCoverage::new(
                &config.get_honggfuzz_target_dir(),
                config.get_honggfuzz_fuzzer_loopcount(),
                &target,
            );

            coverage.clean().await?;
            self.run_hfuzz_target(&target, &config, exit_code, Some(&coverage))
                .await?;
            coverage.generate_report().await?;
            coverage.clean().await?;
        } else {
            self.run_hfuzz_target(&target, &config, exit_code, None)
                .await?;
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

    #[throws]
    async fn run_hfuzz_target(
        &self,
        target: &String,
        config: &TridentConfig,
        exit_code: bool,
        coverage: Option<&HonggfuzzCoverage>,
    ) {
        let env_vars = self.setup_env_vars(config, config.get_fuzzing_with_stats(), coverage)?;

        let mut command = Command::new("cargo");
        command.envs(&env_vars).arg("hfuzz").arg("run").arg(target);

        self.check_crash_dir(target, &env_vars, true, exit_code)?;

        if config.get_fuzzing_with_stats() {
            let mut child = command.stdout(Stdio::piped()).spawn()?;
            Self::handle_child_with_stats(&mut child).await?;
        } else {
            let mut child = command.spawn()?;
            Self::handle_child(&mut child).await?;
        }

        self.check_crash_dir(target, &env_vars, false, exit_code)?;
    }

    #[throws]
    fn setup_env_vars(
        &self,
        config: &TridentConfig,
        stats: bool,
        coverage: Option<&HonggfuzzCoverage>,
    ) -> HashMap<&str, String> {
        let hfuzz_run_args = std::env::var("HFUZZ_RUN_ARGS").unwrap_or_default();
        let mut fuzz_args = config.get_honggfuzz_args(hfuzz_run_args);

        let cargo_target_dir =
            std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| config.get_honggfuzz_target_dir());
        let hfuzz_workspace =
            std::env::var("HFUZZ_WORKSPACE").unwrap_or_else(|_| config.get_honggfuzz_workspace());

        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
        rustflags.push_str("--cfg honggfuzz ");

        let mut env_vars: HashMap<&str, String> = HashMap::new();

        if stats {
            std::env::set_var("FUZZING_METRICS", "1");
            fuzz_args.push_str("--keep_output");
        }

        if let Some(coverage) = coverage {
            rustflags.push_str(&coverage.get_rustflags());
            env_vars.insert("LLVM_PROFILE_FILE", coverage.get_profraw_file());
            env_vars.insert(
                "CARGO_LLVM_COV_TARGET_DIR",
                coverage.get_coverage_target_dir(),
            );
            fuzz_args.push_str(&format!(
                " --env HONGGFUZZ_FUZZER_LOOPCOUNT={}",
                coverage.get_fuzzer_loopcount()
            ));
        }

        env_vars.insert("HFUZZ_RUN_ARGS", fuzz_args);
        env_vars.insert("CARGO_TARGET_DIR", cargo_target_dir);
        env_vars.insert("HFUZZ_WORKSPACE", hfuzz_workspace);
        env_vars.insert("RUSTFLAGS", rustflags);

        env_vars
    }

    #[throws]
    fn check_crash_dir(
        &self,
        target: &str,
        env_vars: &HashMap<&str, String>,
        before_run: bool,
        exit_code: bool,
    ) {
        if !exit_code {
            return;
        }

        // unwrap is safe because we are sure that the env vars are set
        let fuzz_args = env_vars.get("HFUZZ_RUN_ARGS").unwrap();
        let hfuzz_workspace = env_vars.get("HFUZZ_WORKSPACE").unwrap();

        let (crash_dir, ext) =
            get_crash_dir_and_ext(&self.root, target, fuzz_args, hfuzz_workspace);

        if let Ok(crash_files) = get_crash_files(&crash_dir, &ext) {
            if !crash_files.is_empty() {
                match before_run {
                    true => {
                        println!("{ERROR} The crash directory {} already contains crash files from previous runs. \n\nTo run Trident fuzzer with exit code, you must either (backup and) remove the old crash files or alternatively change the crash folder using for example the --crashdir option and the HFUZZ_RUN_ARGS env variable such as:\nHFUZZ_RUN_ARGS=\"--crashdir ./new_crash_dir\"", crash_dir.to_string_lossy());
                        process::exit(1);
                    }
                    false => {
                        println!(
                            "The crash directory {} contains new fuzz test crashes. Exiting!",
                            crash_dir.to_string_lossy()
                        );
                        process::exit(99);
                    }
                }
            }
        }
    }
}
