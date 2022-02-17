use anyhow::Error;
use clap::{Parser, Subcommand};
use fehler::throws;

mod command;

mod config;
use crate::config::ConfigOverride;

#[derive(Parser)]
#[clap(version, propagate_version = true)]
pub struct Cli {
    #[clap(flatten)]
    pub cfg_override: ConfigOverride,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run program tests
    Test {
        /// Anchor project root
        #[clap(short, long, default_value = "./")]
        root: String,
    },
}

#[throws]
pub async fn start() {
    let cli = Cli::parse();

    match cli.command {
        Command::Test { root } => command::test(root).await?,
    }
}
