use anyhow::Error;
use fehler::throws;
use heck::ToSnakeCase;
use trident_client::___private::ProjectType;
use trident_client::___private::TestGenerator;

use crate::command::check_trident_initialized;
use crate::command::get_project_root_for_init;
use crate::command::howto::show_howto;
use crate::command::is_anchor_project;
use crate::command::validate_program_name_usage;

#[throws]
pub(crate) async fn init(
    force: bool,
    skip_build: bool,
    program_name: Option<String>,
    test_name: Option<String>,
    idl_paths: Vec<String>,
) {
    // Get project root
    // - Anchor: directory with Anchor.toml
    // - Vanilla: current directory
    let root = get_project_root_for_init(&idl_paths)?;

    // Determine project type based on whether Anchor.toml exists
    let is_anchor = is_anchor_project()?;
    let project_type = if is_anchor {
        ProjectType::Anchor
    } else {
        ProjectType::Vanilla
    };

    // Validate program_name usage
    validate_program_name_usage(is_anchor, &program_name)?;

    let mut generator: TestGenerator =
        TestGenerator::new_with_root(&root, skip_build, project_type)?;

    let test_name_snake = test_name.map(|name| name.to_snake_case());

    if force {
        generator
            .initialize(
                program_name.clone(),
                test_name_snake.clone(),
                idl_paths.clone(),
            )
            .await?;
        show_howto();
    } else {
        check_trident_initialized(&root)?;
        generator
            .initialize(program_name, test_name_snake, idl_paths)
            .await?;
        show_howto();
    }
}
