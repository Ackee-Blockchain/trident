use anyhow::Error;

use clap::Subcommand;
use fehler::throws;
use heck::ToSnakeCase;
use trident_client::___private::Commander;
use trident_client::___private::TestGenerator;

use crate::command::check_anchor_initialized;
use crate::command::check_fuzz_test_exists;
use crate::command::check_fuzz_test_not_exists;
use crate::command::check_trident_uninitialized;

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
pub(crate) enum FuzzCommand {
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
            help = "Run the fuzzing with exit code, i.e. if it discovers invariant failures or panics the Trident will exit with exit code."
        )]
        with_exit_code: bool,
        #[arg(
            required = false,
            help = "Master seed used for fuzzing, if not provided it will be generated randomly."
        )]
        seed: Option<String>,
    },
    Debug {
        #[arg(
            required = true,
            help = "Name of the desired fuzz template to execute (for example fuzz_0)."
        )]
        target: String,
        #[arg(
            required = true,
            help = "Master seed of the desired fuzz template to execute."
        )]
        seed: String,
    },
    Refresh {
        #[arg(
            required = true,
            help = "Name of the fuzz test to refresh (for example fuzz_0)."
        )]
        target: String,
        #[arg(
            short,
            long,
            required = false,
            help = "Specify the name of the program for which the fuzz test will be refreshed.",
            value_name = "FILE"
        )]
        program_name: Option<String>,
        #[arg(
            short,
            long,
            required = false,
            help = "Skip building the program before refreshing the types file."
        )]
        skip_build: bool,
    },
}

#[throws]
pub(crate) async fn fuzz(subcmd: FuzzCommand) {
    let root = check_anchor_initialized()?;

    check_trident_uninitialized(&root)?;

    match subcmd {
        FuzzCommand::Run {
            target,
            with_exit_code,
            seed,
        } => {
            let commander = Commander::new(&root);

            commander.run(target, with_exit_code, seed).await?;
        }
        FuzzCommand::Debug { target, seed } => {
            let commander = Commander::new(&root);

            commander.run_debug(target, seed).await?;
        }

        FuzzCommand::Add {
            program_name,
            test_name,
            skip_build,
        } => {
            let test_name_snake = test_name.map(|name| name.to_snake_case());

            let mut generator = TestGenerator::new_with_root(&root, skip_build)?;

            if let Some(test_name) = &test_name_snake {
                check_fuzz_test_exists(&root, test_name)?;
            }

            generator
                .add_fuzz_test(program_name, test_name_snake)
                .await?;
        }
        FuzzCommand::Refresh {
            target,
            program_name,
            skip_build,
        } => {
            check_fuzz_test_not_exists(&root, &target)?;

            let mut generator = TestGenerator::new_with_root(&root, skip_build)?;

            generator.refresh_fuzz_test(target, program_name).await?;
        }
    };
}
