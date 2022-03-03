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
    /// Create a `program_client` crate 
    Build {
        /// Anchor project root
        #[clap(short, long, default_value = "./")]
        root: String,
    },
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
        Command::Build { root } => command::build(root).await?,
        Command::Test { root } => command::test(root).await?,
        Command::Localnet => command::localnet().await?,
    }
}
