use anyhow::Error;
use clap::{Parser, Subcommand};
use command::InitTemplate;
use fehler::throws;

// subcommand functions to call and nested subcommands
mod command;
// bring nested subcommand enums into scope
use command::ExplorerCommand;
use command::FuzzCommand;

use command::KeyPairCommand;

#[derive(Parser)]
#[clap(version, propagate_version = true)]
struct Cli {
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
    /// Get information about a keypair
    KeyPair {
        #[clap(subcommand)]
        subcmd: KeyPairCommand,
    },
    /// Run program Integration tests
    Test {
        /// Anchor project root
        #[clap(short, long, default_value = "./")]
        root: String,
    },
    /// Run and debug Fuzz tests
    Fuzz {
        /// Anchor project root
        #[clap(short, long)]
        root: Option<String>,
        #[clap(subcommand)]
        subcmd: FuzzCommand,
    },
    /// Run local test validator
    Localnet,
    /// The Hacker's Explorer
    Explorer {
        #[clap(subcommand)]
        subcmd: ExplorerCommand,
    },
    /// Initialize test environment
    Init {
        /// Generates Tests (Fuzz, PoC) based on the provided template option.
        #[clap(default_value = "both")]
        template: InitTemplate,
    },
    /// Removes target contents except for KeyPair and removes hfuzz_target folder
    Clean,
}

#[throws]
pub async fn start() {
    let cli = Cli::parse();

    match cli.command {
        Command::Build { root } => command::build(root).await?,
        Command::KeyPair { subcmd } => command::keypair(subcmd)?,
        Command::Test { root } => command::test(root).await?,
        Command::Fuzz { root, subcmd } => command::fuzz(root, subcmd).await?,
        Command::Localnet => command::localnet().await?,
        Command::Explorer { subcmd } => command::explorer(subcmd).await?,
        Command::Init { template } => command::init(template).await?,
        Command::Clean => command::clean().await?,
    }
}
