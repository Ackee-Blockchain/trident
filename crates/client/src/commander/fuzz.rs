use crate::coverage::Coverage;
use crate::coverage::NotificationType;
use fehler::throw;
use fehler::throws;
use std::collections::HashMap;
use tokio::process::Command;
use trident_config::coverage::Coverage as CoverageConfig;
use trident_config::TridentConfig;

use super::Commander;
use super::Error;

impl Commander {
    #[throws]
    pub async fn run(&self, target: String, _exit_code: bool, seed: Option<String>) {
        let config = TridentConfig::new();

        if config.get_fuzzing_with_stats() {
            std::env::set_var("FUZZING_METRICS", "fuzzing_metrics.json");

            if config.get_state_monitor() {
                std::env::set_var("FUZZING_STATE_MONITOR", "state_monitor.json");
            }

            if config.get_dashboard() {
                std::env::set_var("FUZZING_DASHBOARD", "fuzzing_dashboard.html");
            }
        }

        let coverage_config = config.get_coverage();
        if coverage_config.get_enable() {
            self.run_with_coverage(&target, &config, coverage_config, seed)
                .await?;
        } else {
            self.run_default(&target, seed).await?;
        }
    }

    #[throws]
    pub async fn run_default(&self, target: &str, seed: Option<String>) {
        let mut child = self.spawn_fuzzer(target, HashMap::new(), seed)?;
        Self::handle_child(&mut child).await?;
    }

    #[throws]
    pub async fn run_with_coverage(
        &self,
        target: &str,
        config: &TridentConfig,
        coverage_config: CoverageConfig,
        seed: Option<String>,
    ) {
        if let Err(err) = coverage_config.validate() {
            throw!(Error::Anyhow(anyhow::anyhow!(err)));
        }

        let coverage = Coverage::new(
            &self.get_target_dir()?,
            target,
            coverage_config.get_attach_extension(),
            coverage_config.get_format(),
            coverage_config.get_loopcount(),
            config.coverage_server_port(),
        );

        if coverage.check_llvm_tools_installed().await.is_err() {
            coverage.prompt_and_install_llvm_tools().await?;
        }

        coverage.clean().await?;

        let env_vars = self.setup_coverage_env_vars(&coverage, config).await?;
        let mut child = self.spawn_fuzzer(target, env_vars, seed)?;

        coverage.notify_extension(NotificationType::Setup).await?;
        Self::handle_child(&mut child).await?;

        coverage.generate_report().await?;
    }

    #[throws]
    async fn setup_coverage_env_vars(
        &self,
        coverage: &Coverage,
        config: &TridentConfig,
    ) -> HashMap<&str, String> {
        let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
        rustflags.push_str(&coverage.get_rustflags());

        let mut env_vars: HashMap<&str, String> = HashMap::new();
        env_vars.insert("RUSTFLAGS", rustflags);
        env_vars.insert("LLVM_PROFILE_FILE", coverage.get_profraw_file());
        env_vars.insert("CARGO_LLVM_COV_TARGET_DIR", coverage.get_target_dir());
        env_vars.insert("FUZZER_LOOPCOUNT", coverage.get_loopcount().to_string());
        env_vars.insert(
            "COVERAGE_SERVER_PORT",
            config.coverage_server_port().to_string(),
        );
        // We need this to know whether to generate code for profraw file generation
        env_vars.insert("COLLECT_COVERAGE", "1".to_string());

        env_vars
    }

    #[throws]
    fn spawn_fuzzer(
        &self,
        target: &str,
        mut env_vars: HashMap<&str, String>,
        seed: Option<String>,
    ) -> tokio::process::Child {
        if let Some(seed) = seed {
            // this is just to make sure it will be possible to decode the seed
            // if it is not a valid hex string, it will panic
            let _decoded_seed = hex::decode(&seed)
                .unwrap_or_else(|_| panic!("The seed is not a valid hex string: {}", seed));

            env_vars.insert("TRIDENT_FUZZ_SEED", seed);
        }

        Command::new("cargo")
            .envs(env_vars)
            .arg("run")
            .arg("--bin")
            .arg(target)
            .args(["--profile", "release"])
            .spawn()?
    }

    #[throws]
    pub async fn run_debug(&self, target: String, seed: String) {
        let config = TridentConfig::new();

        if config.get_fuzzing_with_stats() {
            std::env::set_var("FUZZING_METRICS", "fuzzing_metrics_debug.json");

            if config.get_state_monitor() {
                std::env::set_var("FUZZING_STATE_MONITOR", "state_monitor_debug.json");
            }

            if config.get_dashboard() {
                std::env::set_var("FUZZING_DASHBOARD", "fuzzing_dashboard_debug.html");
            }
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
