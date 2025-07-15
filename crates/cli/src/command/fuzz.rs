use anyhow::bail;
use anyhow::Error;

use clap::Subcommand;
use fehler::throws;
use heck::ToSnakeCase;
use trident_client::___private::Commander;
use trident_client::___private::TestGenerator;

use crate::command::check_anchor_initialized;
use crate::command::check_fuzz_test_exists;
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
            help = "Run the fuzzing with exit code, i.e. if it discovers crash the Trident will exit with exit code 1."
        )]
        with_exit_code: bool,
        #[arg(
            short,
            long,
            required = false,
            num_args(0..=1),
            default_missing_value = "json",
            value_name = "FORMAT",
            help = "Tracks code coverage during fuzzing and generates a report upon completion. Specify format: 'json' (default) or 'html'. The json report can be visualized in your source code using our VS Code extension."
        )]
        generate_coverage: Option<String>,
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
pub(crate) async fn fuzz(subcmd: FuzzCommand) {
    let root = check_anchor_initialized()?;

    check_trident_uninitialized(&root)?;

    match subcmd {
        FuzzCommand::Run {
            target,
            with_exit_code,
            generate_coverage,
            attach_extension,
        } => {
            let commander = Commander::new(&root);

            let (should_generate_coverage, format) = match &generate_coverage {
                Some(format_str) => (true, format_str.clone()),
                None => (false, "json".to_string()), // Default format when not generating coverage
            };

            if !should_generate_coverage && attach_extension {
                bail!("Cannot attach extension without generating coverage!");
            }

            if attach_extension && format != "json" {
                bail!("Cannot attach extension with format other than json!");
            }

            commander
                .run(
                    target,
                    with_exit_code,
                    should_generate_coverage,
                    attach_extension,
                    format,
                )
                .await?;
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
    };
}
