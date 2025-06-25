use fehler::throws;
use tokio::process::Command;

use trident_config::TridentConfig;

use super::{Commander, Error};

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
            .spawn()?;

        Self::handle_child(&mut child).await?;
    }
}
