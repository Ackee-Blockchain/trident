use anyhow::Error;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use fehler::throws;

// subcommand functions to call and nested subcommands
mod command;
// bring nested subcommand enums into scope
use command::FuzzCommand;
use termimad::MadSkin;

macro_rules! load_template {
    ($file:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $file))
    };
}

/// Simple program to greet a person
#[derive(Parser)]
#[command(
    name = "Trident",
    about = "Trident is Rust based fuzzer for Solana programs written using Anchor framework."
)]
struct Cli {
    #[arg(short = 'v', long, help = "Print version information")]
    version: bool,
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Show the HowTo message.")]
    How,
    #[command(
        about = "Initialize Trident in the current Anchor workspace.",
        override_usage = "\nTrident will skip initialization if Trident.toml already exists."
    )]
    Init {
        #[arg(
            short,
            long,
            required = false,
            help = "Force Trident initialization. Trident dependencies will be updated based on the version of Trident CLI."
        )]
        force: bool,
        #[arg(
            short,
            long,
            required = false,
            help = "Specify the name of the program for which fuzz test will be generated.",
            value_name = "FILE"
        )]
        program_name: Option<String>,
        #[arg(
            short,
            long,
            required = false,
            help = "Name of the fuzz test to initialize.",
            value_name = "NAME"
        )]
        test_name: Option<String>,
    },
    #[command(
        about = "Run fuzz subcommands.",
        override_usage = "With fuzz subcommands you can add new fuzz test \
        template or you can run fuzz test on already initialzied one.\
        \n\n\x1b[1m\x1b[4mEXAMPLE:\x1b[0m\
        \n    trident add\
        \n    trident fuzz run-hfuzz fuzz_0\
        \n    trident fuzz debug-hfuzz \x1b[92m<FUZZ_TARGET>\x1b[0m \x1b[92m<PATH_TO_CRASHFILE>\x1b[0m"
    )]
    Fuzz {
        #[clap(subcommand)]
        subcmd: FuzzCommand,
    },
    #[command(about = "Clean Honggfuzz build targets ,additionally perform `anchor clean`")]
    Clean,
}

#[throws]
pub async fn start() {
    let cli = Cli::parse();

    match (cli.version, cli.command) {
        (true, _) => {
            println!(
                "{} - {} \n{}",
                "version",
                env!("CARGO_PKG_VERSION"),
                "https://ackee.xyz/trident/docs/latest/"
            );
            return;
        }
        (false, Some(command)) => match command {
            Command::How => command::howto()?,
            Command::Fuzz { subcmd } => command::fuzz(subcmd).await?,
            Command::Init {
                force,
                program_name,
                test_name,
            } => command::init(force, program_name, test_name).await?,
            Command::Clean => command::clean().await?,
        },
        (false, None) => {
            // No command provided, show help
            Cli::parse_from(["trident", "--help"]);
        }
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

fn show_howto() {
    let markdown_input = load_template!("/src/howto.md");

    // Create a MadSkin for styling the Markdown.
    let skin = MadSkin::default();

    // Print the markdown content to the terminal.
    skin.print_text(markdown_input);
}
