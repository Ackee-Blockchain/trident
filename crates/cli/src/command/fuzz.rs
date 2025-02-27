use std::path::Path;

use anyhow::{bail, Error};

use clap::Subcommand;
use fehler::throws;
use heck::ToSnakeCase;
use trident_client::___private::{Commander, TestGenerator};

use crate::_discover;

pub const TRIDENT_TOML: &str = "Trident.toml";
pub const TRIDENT_TESTS: &str = "trident-tests";
pub const SKIP: &str = "\x1b[33mSkip\x1b[0m";

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
    },
    #[command(
        about = "Run the AFL on desired fuzz test.",
        override_usage = "Specify the desired fuzz \x1b[92m<TARGET>\x1b[0m.\
            \n      \x1b[1m\x1b[4m<TARGET>:\x1b[0m Name of the desired fuzz template to execute (for example fuzz_0).\
            \n\n\x1b[1m\x1b[4mEXAMPLE:\x1b[0m\
            \n      trident fuzz run-afl fuzz_0"
    )]
    Run_Afl {
        #[arg(
            required = true,
            help = "Name of the desired fuzz template to execute (for example fuzz_0)."
        )]
        target: String,
    },
    #[command(
        about = "Run the Honggfuzz on desired fuzz test.",
        override_usage = "Specify the desired fuzz \x1b[92m<TARGET>\x1b[0m.\
            \n      \x1b[1m\x1b[4m<TARGET>:\x1b[0m Name of the desired fuzz template to execute (for example fuzz_0).\
            \n\n\x1b[1m\x1b[4mEXAMPLE:\x1b[0m\
            \n      trident fuzz run-hfuzz fuzz_0"
    )]
    Run_Hfuzz {
        #[arg(
            required = true,
            help = "Name of the desired fuzz template to execute (for example fuzz_0)."
        )]
        target: String,
        #[arg(
            short,
            long,
            required = false,
            help = "Run the Honggfuzz with exit code, i.e. if it discovers crash the Trident will exit with exit code 1."
        )]
        with_exit_code: bool,
    },

    #[command(
        about = "Debug found crash using the AFL on desired fuzz test.",
        override_usage = "Specify the desired fuzz \x1b[92m<TARGET>\x1b[0m and \x1b[92m<PATH_TO_CRASHFILE>\x1b[0m.\
            \n      \x1b[1m\x1b[4m<TARGET>:\x1b[0m Name of the desired fuzz template to debug (for example fuzz_0).\
            \n      \x1b[1m\x1b[4m<PATH_TO_CRASHFILE>:\x1b[0m Path to the crash found during fuzzing.\
            \n\n\x1b[1m\x1b[4mHINT:\x1b[0m By default crashfiles will be stored in the following folders:\
            \n      \x1b[1m\x1b[4mHonggfuzz:\x1b[0m trident-tests/fuzzing/honggfuzz/hfuzz_workspace/<TARGET>\
            \n      \x1b[1m\x1b[4mAFL:\x1b[0m trident-tests/fuzzing/afl/afl_workspace/out/default/crashes\
            \n\n\x1b[1m\x1b[4mEXAMPLE:\x1b[0m\
            \n      trident fuzz debug-afl fuzz_0 trident-tests/fuzzing/afl/afl_workspace/out/default/crashes/id...\
            \n\n\x1b[1m\x1b[33mWarning\x1b[0m:\
            \n      Do not mix fuzz templates and crashfiles. If the crash was found with fuzz_0, then debug it with fuzz_0."
    )]
    Debug_Afl {
        #[arg(
            required = true,
            help = "Name of the desired fuzz template to execute (for example fuzz_0)"
        )]
        target: String,
        #[arg(required = true, help = "Path to the crash found during fuzzing")]
        crash_file_path: String,
    },
    #[command(
        about = "Debug found crash using the Honggfuzz on desired fuzz test.",
        override_usage = "Specify the desired fuzz \x1b[92m<TARGET>\x1b[0m and \x1b[92m<PATH_TO_CRASHFILE>\x1b[0m.\
            \n      \x1b[1m\x1b[4m<TARGET>:\x1b[0m Name of the desired fuzz template to debug (for example fuzz_0).\
            \n      \x1b[1m\x1b[4m<PATH_TO_CRASHFILE>:\x1b[0m Path to the crash found during fuzzing.\
            \n\n\x1b[1m\x1b[4mHINT:\x1b[0m By default crashfiles will be stored in the following folders:\
            \n      \x1b[1m\x1b[4mHonggfuzz:\x1b[0m trident-tests/fuzzing/honggfuzz/hfuzz_workspace/<TARGET>\
            \n\n\x1b[1m\x1b[4mEXAMPLE:\x1b[0m\
            \n      trident fuzz debug-hfuzz fuzz_0 trident-tests/fuzzing/honggfuzz/hfuzz_workspace/fuzz_0/SIGAR...\
            \n\n\x1b[1m\x1b[33mWarning\x1b[0m:\
            \n      Do not mix fuzz templates and crashfiles. If the crash was found with fuzz_0, then debug it with fuzz_0."
    )]
    Debug_Hfuzz {
        #[arg(
            required = true,
            help = "Name of the desired fuzz template to execute (for example fuzz_0)"
        )]
        target: String,
        #[arg(required = true, help = "Path to the crash found during fuzzing")]
        crash_file_path: String,
    },
}

#[throws]
pub async fn fuzz(subcmd: FuzzCommand) {
    let root = match _discover(TRIDENT_TOML)? {
        Some(root) => root,
        None => {
            bail!("It does not seem that Trident is initialized because the Trident.toml file was not found in any parent directory!");
        }
    };

    let commander = Commander::with_root(&Path::new(&root).to_path_buf());

    match subcmd {
        FuzzCommand::Run_Afl { target } => {
            commander.run_afl(target).await?;
        }
        FuzzCommand::Run_Hfuzz {
            target,
            with_exit_code,
        } => {
            if with_exit_code {
                commander.run_honggfuzz_with_exit_code(target).await?;
            } else {
                commander.run_honggfuzz(target).await?;
            }
        }
        FuzzCommand::Debug_Afl {
            target,
            crash_file_path,
        } => {
            commander.run_afl_debug(target, crash_file_path).await?;
        }
        FuzzCommand::Debug_Hfuzz {
            target,
            crash_file_path,
        } => {
            commander.run_hfuzz_debug(target, crash_file_path).await?;
        }
        FuzzCommand::Add {
            program_name,
            test_name,
        } => {
            let test_name_snake = test_name.map(|name| name.to_snake_case());
            if let Some(name) = &test_name_snake {
                let fuzz_test_dir = Path::new(&root).join(TRIDENT_TESTS).join(name);
                if fuzz_test_dir.exists() {
                    println!("{SKIP} [{}/{}] already exists", TRIDENT_TESTS, name);
                    return;
                }
            }
            let mut generator = TestGenerator::new_with_root(&root)?;
            generator
                .add_fuzz_test(program_name, test_name_snake)
                .await?;
        }
    };
}
