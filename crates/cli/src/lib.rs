use anyhow::Error;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use command::{SnapshotsType, TestsType};
use fehler::throws;

// subcommand functions to call and nested subcommands
mod command;
// bring nested subcommand enums into scope
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
    /// Create or update a `program_client` crate
    Build {
        /// Anchor project root
        #[clap(short, long)]
        root: Option<String>,
    },
    /// Get information about a keypair
    KeyPair {
        #[clap(subcommand)]
        subcmd: KeyPairCommand,
    },
    /// Run program Integration tests
    Test {
        /// Anchor project root
        #[clap(short, long)]
        root: Option<String>,
    },
    /// Run and debug Fuzz tests
    Fuzz {
        /// Anchor project root
        #[clap(short, long)]
        root: Option<String>,
        #[clap(subcommand)]
        subcmd: FuzzCommand,
    },
    /// Initialize test environment
    Init {
        /// Specifies the types of tests for which the frameworks should be initialized.
        #[clap(default_value = "fuzz")]
        tests_type: TestsType,
        /// Specifies type of Accounts Snapshots, i.e used derive macro or generated file
        #[clap(default_value = "file")]
        snapshots_type: SnapshotsType,
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
        Command::Init {
            tests_type,
            snapshots_type,
        } => command::init(tests_type, snapshots_type).await?,
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
