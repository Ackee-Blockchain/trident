use crate::constants::*;
use fehler::{throw, throws};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::process::Stdio;
use std::{fs::File, path::Path};
use tokio::{io::AsyncWriteExt, process::Command};

use crate::coverage::{afl::AflCoverage, Coverage};
use trident_config::afl::AflSeed;
use trident_config::TridentConfig;

use super::{Commander, Error};
use rand::RngCore;

impl Commander {
    #[throws]
    pub async fn run_afl(&self, target: String, generate_coverage: bool, dynamic_coverage: bool) {
        let config = TridentConfig::new();

        if config.get_fuzzing_with_stats() {
            std::env::set_var("FUZZING_METRICS", "1");
        }

        if generate_coverage {
            self.run_afl_with_coverage(&target, &config, dynamic_coverage)
                .await?;
        } else {
            self.build_afl_target(&target, &config, None).await?;
            self.run_afl_target(&target, &config, None).await?;
        }
    }

    #[throws]
    pub async fn run_afl_with_coverage(
        &self,
        target: &str,
        config: &TridentConfig,
        dynamic_coverage: bool,
    ) {
        let coverage = AflCoverage::new(
            &config.get_afl_target_dir(),
            config.get_afl_fuzzer_loopcount(),
            target,
            dynamic_coverage,
        );

        coverage.clean().await?;
        self.build_afl_target(target, config, Some(&coverage))
            .await?;
        self.run_afl_target(target, config, Some(&coverage)).await?;
        coverage.generate_report().await?;
        coverage.clean().await?;
    }

    #[throws]
    pub async fn run_afl_debug(&self, target: String, crash_file: String) {
        let config = TridentConfig::new();

        let crash_file_path = Path::new(&crash_file);
        let crash_file = if crash_file_path.is_absolute() {
            crash_file_path
        } else {
            let cwd = std::env::current_dir()?;
            &cwd.join(crash_file_path)
        };

        if !crash_file.try_exists()? {
            println!("{ERROR} The crash file [{:?}] not found", crash_file);
            throw!(Error::CrashFileNotFound);
        }

        let mut file = File::open(crash_file)?;
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)?;

        let cargo_target_dir = config.get_afl_target_dir();

        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
        rustflags.push_str("--cfg afl --cfg fuzzing_debug");

        let mut child = Command::new("cargo")
            .env("TRIDENT_LOG", "1")
            .env("RUSTFLAGS", rustflags)
            .arg("afl")
            .arg("run")
            .args(["--target-dir", &cargo_target_dir])
            .args(["--bin", &target])
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(&file_contents).await?;
        }
        child.wait().await?;
    }

    #[throws]
    async fn build_afl_target(
        &self,
        target: &str,
        config: &TridentConfig,
        coverage: Option<&AflCoverage>,
    ) {
        // build args without cargo target dir
        let build_args = config.get_afl_build_args();
        let cargo_target_dir = config.get_afl_target_dir();

        let mut env_vars = HashMap::new();
        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
        rustflags.push_str("--cfg afl ");

        if let Some(coverage) = coverage {
            rustflags.push_str(&coverage.get_rustflags());
            env_vars.insert("LLVM_PROFILE_FILE", coverage.get_profraw_file());
            env_vars.insert(
                "CARGO_LLVM_COV_TARGET_DIR",
                coverage.get_coverage_target_dir(),
            );
        }

        env_vars.insert("RUSTFLAGS", rustflags);
        let mut child = Command::new("cargo")
            .envs(env_vars)
            .arg("afl")
            .arg("build")
            .args(["--target-dir", &cargo_target_dir])
            .args(build_args)
            .args(["--bin", target])
            .spawn()?;

        Self::handle_child(&mut child).await?;
    }

    #[throws]
    async fn run_afl_target(
        &self,
        target: &str,
        config: &TridentConfig,
        coverage: Option<&AflCoverage>,
    ) {
        let afl_workspace_in = config.get_afl_workspace_in();
        let afl_workspace_out = config.get_afl_workspace_out();
        let full_target_path = config.get_afl_target_path(target);
        let afl_workspace_in_path = Path::new(&afl_workspace_in);

        validate_afl_workspace(afl_workspace_in_path, config)?;

        // fuzz args without afl workspace in and out
        let fuzz_args = config.get_afl_fuzz_args();

        let mut env_vars = HashMap::new();
        env_vars.insert("AFL_KILL_SIGNAL", "2".to_string());

        if let Some(coverage) = coverage {
            env_vars.insert("LLVM_PROFILE_FILE", coverage.get_profraw_file());
            env_vars.insert(
                "CARGO_LLVM_COV_TARGET_DIR",
                coverage.get_coverage_target_dir(),
            );
            env_vars.insert("AFL_FUZZER_LOOPCOUNT", coverage.get_fuzzer_loopcount());
        }

        let mut child = Command::new("cargo")
            .envs(env_vars)
            .arg("afl")
            .arg("fuzz")
            .args(["-i", &afl_workspace_in])
            .args(["-o", &afl_workspace_out])
            .args(fuzz_args)
            .arg(&full_target_path)
            .spawn()?;

        if let Some(coverage) = coverage {
            coverage.notify_dynamic_coverage_start().await?;
        }

        Self::handle_child(&mut child).await?;
    }
}

fn create_seed_file(path: &Path, seed: &AflSeed) -> std::io::Result<()> {
    let (bytes, override_file) = obtain_seed(seed);

    let file_path = path.join(&seed.file_name);

    if file_path.exists() {
        if override_file {
            let mut file = File::create(file_path)?;
            file.write_all(&bytes)?;
        }
    } else {
        let mut file = File::create(file_path)?;
        file.write_all(&bytes)?;
    }

    Ok(())
}

fn obtain_seed(value: &AflSeed) -> (Vec<u8>, bool) {
    match value.bytes_count {
        Some(number_of_random_bytes) => {
            if number_of_random_bytes > 0 {
                let mut rng = rand::rngs::OsRng;
                let mut seed = vec![0u8; number_of_random_bytes];
                rng.fill_bytes(&mut seed);
                (seed, value.override_file.unwrap_or_default())
            } else {
                let seed_as_bytes = value
                    .seed
                    .clone()
                    .unwrap_or_else(|| panic!("Seed value is empty for seed {}", value.file_name));

                (
                    seed_as_bytes.as_bytes().to_vec(),
                    value.override_file.unwrap_or_default(),
                )
            }
        }
        None => {
            let seed_as_bytes = value
                .seed
                .clone()
                .unwrap_or_else(|| panic!("Seed value is empty for seed {}", value.file_name));
            (
                seed_as_bytes.as_bytes().to_vec(),
                value.override_file.unwrap_or_default(),
            )
        }
    }
}

#[throws]
fn validate_afl_workspace(workspace: &Path, config: &TridentConfig) {
    let initial_seeds = config.get_initial_seed();
    if !workspace.exists() {
        std::fs::create_dir_all(workspace)?;

        for x in initial_seeds {
            create_seed_file(workspace, &x)?;
        }
    } else if workspace.is_dir() {
        for x in initial_seeds {
            create_seed_file(workspace, &x)?;
        }
    } else {
        throw!(Error::BadAFLWorkspace)
    }
}
