use anyhow::Error;
use clap::Parser;
use clap::Subcommand;

use fehler::throws;

mod command;

use crate::command::FuzzCommand;

#[derive(Parser)]
#[command(
    name = "Trident",
    about = "Trident is Rust based fuzzer for Solana programs written using Anchor framework.",
    version = env!("CARGO_PKG_VERSION")
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
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
            help = "Skip building the program before initializing Trident."
        )]
        skip_build: bool,
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
        \n    trident fuzz add\
        \n    trident fuzz run fuzz_0\
        \n    trident fuzz debug \x1b[92m<FUZZ_TARGET>\x1b[0m \x1b[92m<SEED>\x1b[0m\
        \n    trident fuzz refresh fuzz_0"
    )]
    Fuzz {
        #[clap(subcommand)]
        subcmd: FuzzCommand,
    },
    #[command(about = "Clean build target, additionally perform `anchor clean`")]
    Clean,
    #[command(about = "Start HTTP server to serve fuzzing dashboards")]
    Server {
        #[arg(
            short,
            long,
            required = false,
            help = "Directory to monitor for dashboard files",
            value_name = "DIR",
            default_value = ".fuzz-artifacts"
        )]
        directory: String,
        #[arg(
            short,
            long,
            required = false,
            help = "Port to run the server on",
            value_name = "PORT",
            default_value = "8000"
        )]
        port: u16,
        #[arg(
            long,
            required = false,
            help = "Host to bind the server to",
            value_name = "HOST",
            default_value = "localhost"
        )]
        host: String,
    },
    #[command(about = "Compare two regression JSON files and identify differing iteration seeds")]
    Compare {
        #[arg(help = "Path to the first regression JSON file", value_name = "FILE1")]
        file1: String,
        #[arg(help = "Path to the second regression JSON file", value_name = "FILE2")]
        file2: String,
    },
}

#[throws]
pub async fn start() {
    let cli = Cli::parse();

    match cli.command {
        Command::How => command::howto()?,
        Command::Fuzz { subcmd } => command::fuzz(subcmd).await?,
        Command::Init {
            force,
            skip_build,
            program_name,
            test_name,
        } => command::init(force, skip_build, program_name, test_name).await?,
        Command::Clean => command::clean().await?,
        Command::Server {
            directory,
            port,
            host,
        } => command::server(directory, port, host).await?,
        Command::Compare { file1, file2 } => command::compare_regression(file1, file2)?,
    }
}
