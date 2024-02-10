use anyhow::{Context, Error, Result};

use clap::{Parser, Subcommand};
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
        /// Flag to skip generating template for fuzzing and activating the fuzzing feature.
        #[arg(short, long)]
        skip_fuzzer: bool,
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
        Command::Init { skip_fuzzer } => command::init(skip_fuzzer).await?,
        Command::Clean => command::clean().await?,
    }
}

// Climbs each parent directory until we find target.
fn _discover(target: &str) -> Result<Option<String>> {
    let _cwd = std::env::current_dir()?;
    let mut cwd_opt = Some(_cwd.as_path());

    while let Some(cwd) = cwd_opt {
        for f in std::fs::read_dir(cwd)
            .with_context(|| format!("Error reading the directory with path: {}", cwd.display()))?
        {
            let p = f
                .with_context(|| {
                    format!("Error reading the directory with path: {}", cwd.display())
                })?
                .path();
            if let Some(filename) = p.file_name() {
                if filename.to_str() == Some(target) {
                    return Ok(Some(cwd.to_string_lossy().to_string()));
                }
            }
        }

        cwd_opt = cwd.parent();
    }

    Ok(None)
}

// Check if dir directory already contains target
fn _check_if_present(dir: &String, target: &str) -> Result<bool> {
    let dir = std::path::Path::new(&dir);
    for f in std::fs::read_dir(dir)
        .with_context(|| format!("Error reading the directory with path: {}", dir.display()))?
    {
        let p = f
            .with_context(|| format!("Error reading the directory with path: {}", dir.display()))?
            .path();
        if let Some(filename) = p.file_name() {
            if filename.to_str() == Some(target) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
