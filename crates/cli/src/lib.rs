use anyhow::Error;
use clap::{Parser, Subcommand};
use fehler::throws;

mod command;

#[derive(Parser)]
#[clap(version, propagate_version = true)]
pub struct Cli {
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
    /// Run local test validator
    Localnet,
}

#[throws]
pub async fn start() {
    let cli = Cli::parse();

    match cli.command {
        Command::Test { root } => command::test(root).await?,
        Command::Localnet => command::localnet().await?,
    }
}
