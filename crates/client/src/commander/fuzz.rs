use fehler::throws;
use tokio::process::Command;

use trident_config::TridentConfig;

use super::Commander;
use super::Error;

impl Commander {
    #[throws]
    pub async fn run(
        &self,
        target: String,
        _exit_code: bool,
        _generate_coverage: bool,
        _dynamic_coverage: bool,
    ) {
        let config = TridentConfig::new();

        if config.get_fuzzing_with_stats() {
            std::env::set_var("FUZZING_METRICS", "1");
        }

        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(target)
            .args(["--profile", "release"])
            .spawn()?;

        Self::handle_child(&mut child).await?;
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
