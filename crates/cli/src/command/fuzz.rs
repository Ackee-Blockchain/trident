use std::path::Path;

use anyhow::{bail, Error};

use clap::Subcommand;
use fehler::throws;
use heck::ToSnakeCase;
use trident_client::___private::{Commander, TestGenerator};

use crate::_discover;

pub const ANCHOR_TOML: &str = "Anchor.toml";
pub const TRIDENT_TOML: &str = "Trident.toml";
pub const TRIDENT_TESTS: &str = "trident-tests";
pub const SKIP: &str = "\x1b[33mSkip\x1b[0m";
pub const TESTS_WORKSPACE_DIRECTORY: &str = "trident-tests";

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
pub enum FuzzCommand {
    #[command(about = "Generate new Fuzz Test template.")]
    Add {
        #[arg(
            short,
            long,
            required = false,
            help = "Specify the name of the program for which the fuzz test will be generated.",
            value_name = "FILE"
        )]
        program_name: Option<String>,
        #[arg(
            short,
            long,
            required = false,
            help = "Name of the fuzz test to add.",
            value_name = "NAME"
        )]
        test_name: Option<String>,
        #[arg(
            short,
            long,
            required = false,
            help = "Skip building the program before adding new fuzz test."
        )]
        skip_build: bool,
    },
    Run {
        #[arg(
            required = true,
            help = "Name of the desired fuzz template to execute (for example fuzz_0)."
        )]
        target: String,
        #[arg(
            short,
            long,
            required = false,
            help = "Run the fuzzing with exit code, i.e. if it discovers crash the Trident will exit with exit code 1."
        )]
        with_exit_code: bool,
        #[arg(
            short,
            long,
            required = false,
            help = "Tracks code coverage during fuzzing and generates a JSON report upon completion. The coverage data can be visualized in your source code using our VS Code extension."
        )]
        generate_coverage: bool,
        #[arg(
            short,
            long = "attach-extension",
            required = false,
            help = "Enables real-time coverage visualization in VS Code during fuzzing. The VS Code extension must be actively running to utilize this feature."
        )]
        attach_extension: bool,
    },
    Debug {
        #[arg(
            required = true,
            help = "Name of the desired fuzz template to execute (for example fuzz_0)."
        )]
        target: String,
        #[arg(
            required = true,
            help = "Seed of the desired fuzz template to execute (for example fuzz_0)."
        )]
        seed: String,
    },
}

#[throws]
pub async fn fuzz(subcmd: FuzzCommand) {
    // First, look for Anchor.toml to find the project root
    let anchor_root = match _discover(ANCHOR_TOML)? {
        Some(root) => root,
        None => {
            bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
        }
    };

    // Then check if Trident.toml exists in the trident-tests directory
    let trident_toml_path = Path::new(&anchor_root)
        .join(TESTS_WORKSPACE_DIRECTORY)
        .join(TRIDENT_TOML);

    if !trident_toml_path.exists() {
        bail!("It does not seem that Trident is initialized because the Trident.toml file was not found in the trident-tests directory. Please run 'trident init' first.");
    }

    let commander = Commander::new();

    match subcmd {
        FuzzCommand::Run {
            target,
            with_exit_code,
            generate_coverage,
            attach_extension,
        } => {
            if !generate_coverage && attach_extension {
                bail!("Cannot attach extension without generating coverage!");
            }
            commander
                .run(target, with_exit_code, generate_coverage, attach_extension)
                .await?;
        }
        FuzzCommand::Debug { target, seed } => {
            commander.run_debug(target, seed).await?;
        }

        FuzzCommand::Add {
            program_name,
            test_name,
            skip_build,
        } => {
            let test_name_snake = test_name.map(|name| name.to_snake_case());
            if let Some(name) = &test_name_snake {
                let fuzz_test_dir = Path::new(&anchor_root).join(TRIDENT_TESTS).join(name);
                if fuzz_test_dir.exists() {
                    println!("{SKIP} [{}/{}] already exists", TRIDENT_TESTS, name);
                    return;
                }
            }
            let mut generator = TestGenerator::new_with_root(&anchor_root, skip_build)?;
            generator
                .add_fuzz_test(program_name, test_name_snake)
                .await?;
        }
    };
}
