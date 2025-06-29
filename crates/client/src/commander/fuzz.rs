use fehler::throws;
use tokio::process::Command;

use trident_config::TridentConfig;

use super::{Commander, Error};
use crate::cleaner::discover_root;
use crate::coverage::Coverage;
use std::collections::HashMap;

impl Commander {
    #[throws]
    pub async fn run(
        &self,
        target: String,
        _exit_code: bool,
        generate_coverage: bool,
        notify_extension: bool,
        format: String,
    ) {
        let config = TridentConfig::new();

        if config.get_fuzzing_with_stats() {
            std::env::set_var("FUZZING_METRICS", "1");
        }

        if generate_coverage {
            self.run_with_coverage(&target, notify_extension, format, config.loop_count())
                .await?;
        } else {
            self.run_default(&target).await?;
        }
    }

    #[throws]
    pub async fn run_default(&self, target: &str) {
        let mut child = self.spawn_fuzzer(&target, HashMap::new())?;
        Self::handle_child(&mut child).await?;
    }

    #[throws]
    pub async fn run_with_coverage(
        &self,
        target: &str,
        notify_extension: bool,
        format: String,
        loop_count: u64,
    ) {
        let coverage = Coverage::new(
            &get_target_dir(),
            target,
            notify_extension,
            format,
            loop_count,
        );

        coverage.clean().await?;

        let env_vars = self.setup_coverage_env_vars(&coverage).await?;
        let mut child = self.spawn_fuzzer(target, env_vars)?;

        coverage.notify_extension().await?;
        Self::handle_child(&mut child).await?;

        coverage.generate_report().await?;
    }

    #[throws]
    async fn setup_coverage_env_vars(&self, coverage: &Coverage) -> HashMap<&str, String> {
        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
        rustflags.push_str(&coverage.get_rustflags());

        let mut env_vars: HashMap<&str, String> = HashMap::new();
        env_vars.insert("RUSTFLAGS", rustflags);
        env_vars.insert("LLVM_PROFILE_FILE", coverage.get_profraw_file());
        env_vars.insert("CARGO_LLVM_COV_TARGET_DIR", coverage.get_target_dir());
        env_vars.insert("FUZZER_LOOPCOUNT", coverage.get_loop_count().to_string());

        env_vars
    }

    #[throws]
    fn spawn_fuzzer(&self, target: &str, env_vars: HashMap<&str, String>) -> tokio::process::Child {
        Command::new("cargo")
            .envs(env_vars)
            .arg("run")
            .arg("--bin")
            .arg(target)
            .spawn()?
    }

    #[throws]
    pub async fn run_debug(&self, target: String, seed: String) {
        let config = TridentConfig::new();

        if config.get_fuzzing_with_stats() {
            std::env::set_var("FUZZING_METRICS", "1");
        }

        let mut child = Command::new("cargo")
            .env("TRIDENT_FUZZ_DEBUG", seed)
            .arg("run")
            .arg("--bin")
            .arg(target)
            .spawn()?;

        Self::handle_child(&mut child).await?;
    }
}

fn get_target_dir() -> String {
    let root = discover_root().expect("failed to find the root folder");
    let target_dir = root
        .join("trident-tests/target")
        .to_str()
        .unwrap()
        .to_string();

    target_dir
}
