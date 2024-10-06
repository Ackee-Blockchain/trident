use crate::constants::*;
use fehler::{throw, throws};
use std::io::{Read, Write};
use std::process::Stdio;
use std::{fs::File, path::Path};
use tokio::{io::AsyncWriteExt, process::Command};
use trident_fuzz::config::afl::AflSeed;
use trident_fuzz::config::Config;

use super::{Commander, Error};

impl Commander {
    /// Runs fuzzer on the given target.
    #[throws]
    pub async fn run_afl(&self, target: String) {
        let config = Config::new();

        let build_args = config.get_afl_build_args();
        let fuzz_args = config.get_afl_fuzz_args();

        let mut target_path = config.get_afl_target_path();
        target_path.push_str(&target);

        let afl_workspace_in = config.get_afl_workspace_in();
        let afl_workspace_in_path = Path::new(&afl_workspace_in);
        let initial_seeds = config.get_initial_seed();

        if !afl_workspace_in_path.exists() {
            std::fs::create_dir_all(afl_workspace_in_path)?;

            for x in initial_seeds {
                create_seed_file(afl_workspace_in_path, x)?;
            }
        } else if afl_workspace_in_path.is_dir() {
            for x in initial_seeds {
                create_seed_file(afl_workspace_in_path, x)?;
            }
        } else {
            throw!(Error::BadAFLWorkspace)
        }

        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();

        rustflags.push_str("--cfg afl");

        let mut child = Command::new("cargo")
            .env("RUSTFLAGS", rustflags)
            .arg("afl")
            .arg("build")
            .args(build_args)
            .args(["--bin", &target])
            .spawn()?;
        Self::handle_child(&mut child).await?;

        let mut child = Command::new("cargo")
            .arg("afl")
            .arg("fuzz")
            .args(fuzz_args)
            .arg(&target_path)
            .spawn()?;

        Self::handle_child(&mut child).await?;
    }

    /// Runs fuzzer on the given target.
    #[throws]
    pub async fn run_afl_debug(&self, target: String, crash_file_path: String) {
        let config = Config::new();

        let crash_file = std::path::Path::new(&self.root as &str).join(crash_file_path);

        let build_args = config.get_afl_build_args();

        if !crash_file.try_exists()? {
            println!("{ERROR} The crash file [{:?}] not found", crash_file);
            throw!(Error::CrashFileNotFound);
        }

        let mut target_path = config.get_afl_target_path();
        target_path.push_str(&target);

        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();

        rustflags.push_str("--cfg afl");

        let mut file = File::open(crash_file)?;
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)?;

        // using exec rather than spawn and replacing current process to avoid unflushed terminal output after ctrl+c signal
        let mut child = Command::new("cargo")
            .env("RUSTFLAGS", rustflags)
            .arg("afl")
            .arg("run")
            .args(build_args)
            .arg(target_path)
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(&file_contents).await?;
        }
        child.wait().await?;
    }
}

fn create_seed_file(path: &Path, seed: &AflSeed) -> std::io::Result<()> {
    let file_path = path.join(&seed.file_name);

    if file_path.exists() {
        if seed.override_file {
            let mut file = File::create(file_path)?;
            file.write_all(&seed.seed)?;
        }
    } else {
        let mut file = File::create(file_path)?;
        file.write_all(&seed.seed)?;
    }

    Ok(())
}
