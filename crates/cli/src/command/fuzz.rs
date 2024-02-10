use anyhow::{bail, Error};

use clap::Subcommand;
use fehler::throws;
use trdelnik_client::{Commander, TestGenerator};

use crate::_discover;
const TRDELNIK_TOML: &str = "Trdelnik.toml";

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
pub enum FuzzCommand {
    /// Run fuzz target
    Run {
        /// Name of the fuzz target
        target: String,
        /// Trdelnik will return exit code 1 in case of found crash files in the crash folder. This is checked before and after the fuzz test run.
        #[arg(short, long)]
        with_exit_code: bool,
    },
    /// Debug fuzz target with crash file
    Run_Debug {
        /// Name of the fuzz target
        target: String,
        /// Path to the crash file
        crash_file_path: String,
    },
    Add,
}

#[throws]
pub async fn fuzz(root: Option<String>, subcmd: FuzzCommand) {
    let root = match root {
        Some(r) => r,
        _ => {
            let root = _discover(TRDELNIK_TOML)?;
            if let Some(r) = root {
                r
            } else {
                bail!("It does not seem that Trdelnik is initialized because the Trdelnik.toml file was not found in any parent directory!");
            }
        }
    };

    let commander = Commander::with_root(root.clone());

    match subcmd {
        FuzzCommand::Run {
            target,
            with_exit_code,
        } => {
            if with_exit_code {
                commander.run_fuzzer_with_exit_code(target).await?;
            } else {
                commander.run_fuzzer(target).await?;
            }
        }
        FuzzCommand::Run_Debug {
            target,
            crash_file_path,
        } => {
            commander.run_fuzzer_debug(target, crash_file_path).await?;
        }

        FuzzCommand::Add => {
            let generator = TestGenerator::new_with_root(&root);
            generator.add_new_fuzz_test().await?;
        }
    };
}
