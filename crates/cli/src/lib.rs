use anyhow::{Context, Error, Result};
use clap::{Parser, Subcommand};
use fehler::throws;

// subcommand functions to call and nested subcommands
mod command;
// bring nested subcommand enums into scope
use command::ExplorerCommand;
use command::FuzzCommand;
use command::InitCommand;
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
    /// Run program tests
    Test {
        /// Anchor project root
        #[clap(short, long, default_value = "./")]
        root: String,
    },
    /// Run and debug fuzz tests
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
        #[clap(value_enum, short, long, default_value = "both")]
        template: InitCommand,
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

pub fn discover(searching_for: &str) -> Result<Option<String>> {
    let current_dir = std::env::current_dir()?;
    let mut dir = Some(current_dir.as_path());
    while let Some(cwd) = dir {
        for file in std::fs::read_dir(cwd)
            .with_context(|| format!("Error reading the directory with path: {}", cwd.display()))?
        {
            let path = file
                .with_context(|| {
                    format!("Error reading the directory with path: {}", cwd.display())
                })?
                .path();
            if let Some(filename) = path.file_name() {
                if filename.to_str() == Some(searching_for) {
                    return Ok(Some(cwd.to_string_lossy().to_string()));
                }
            }
        }
        dir = cwd.parent();
    }
    Ok(None)
}
