use anyhow::Error;

use clap::Subcommand;
use fehler::throws;
use heck::ToSnakeCase;
use trident_client::___private::Commander;
use trident_client::___private::ProjectType;
use trident_client::___private::TestGenerator;

use crate::command::check_fuzz_test_exists;
use crate::command::check_fuzz_test_not_exists;
use crate::command::check_trident_uninitialized;
use crate::command::get_project_root_for_fuzz;
use crate::command::is_anchor_project;
use crate::command::validate_program_name_usage;

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
        #[arg(
            long,
            required = false,
            num_args = 1..,
            help = "Path(s) to IDL file(s). Specify multiple files separated by spaces. When provided, default target/idl/ directory is ignored.",
            value_name = "FILE"
        )]
        idl_paths: Vec<String>,
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
        #[arg(
            long,
            required = false,
            num_args = 1..,
            help = "Path(s) to IDL file(s). Specify multiple files separated by spaces. When provided, default target/idl/ directory is ignored.",
            value_name = "FILE"
        )]
        idl_paths: Vec<String>,
    },
}

#[throws]
pub(crate) async fn fuzz(subcmd: FuzzCommand) {
    // For fuzz commands, we need to determine the root based on the subcommand
    // Extract idl_paths early to use in get_project_root_for_fuzz
    let idl_paths = match &subcmd {
        FuzzCommand::Add { idl_paths, .. } => idl_paths.clone(),
        FuzzCommand::Refresh { idl_paths, .. } => idl_paths.clone(),
        FuzzCommand::Run { .. } | FuzzCommand::Debug { .. } => Vec::new(),
    };

    // Get project root
    // - Anchor: directory with Anchor.toml
    // - Vanilla: directory containing trident-tests (search upward from cwd)
    let root = get_project_root_for_fuzz(&idl_paths)?;

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
            idl_paths,
        } => {
            let test_name_snake = test_name.map(|name| name.to_snake_case());

            // Determine project type based on whether Anchor.toml exists
            let is_anchor = is_anchor_project()?;
            let project_type = if is_anchor {
                ProjectType::Anchor
            } else {
                ProjectType::Vanilla
            };

            // Validate program_name usage
            validate_program_name_usage(is_anchor, &program_name)?;

            let mut generator = TestGenerator::new_with_root(&root, skip_build, project_type)?;

            if let Some(test_name) = &test_name_snake {
                check_fuzz_test_exists(&root, test_name)?;
            }

            generator
                .add_fuzz_test(program_name, test_name_snake, idl_paths)
                .await?;
        }
        FuzzCommand::Refresh {
            target,
            program_name,
            skip_build,
            idl_paths,
        } => {
            check_fuzz_test_not_exists(&root, &target)?;

            // Determine project type based on whether Anchor.toml exists
            let is_anchor = is_anchor_project()?;
            let project_type = if is_anchor {
                ProjectType::Anchor
            } else {
                ProjectType::Vanilla
            };

            // Validate program_name usage
            validate_program_name_usage(is_anchor, &program_name)?;

            let mut generator = TestGenerator::new_with_root(&root, skip_build, project_type)?;

            generator
                .refresh_fuzz_test(target, program_name, idl_paths)
                .await?;
        }
    };
}
