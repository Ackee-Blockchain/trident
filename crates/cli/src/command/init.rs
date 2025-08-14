use anyhow::Error;
use fehler::throws;
use heck::ToSnakeCase;
use trident_client::___private::TestGenerator;

use crate::command::check_anchor_initialized;
use crate::command::check_trident_initialized;
use crate::command::howto::show_howto;

#[throws]
pub(crate) async fn init(
    force: bool,
    skip_build: bool,
    program_name: Option<String>,
    test_name: Option<String>,
) {
    let root = check_anchor_initialized()?;

    let mut generator: TestGenerator = TestGenerator::new_with_root(&root, skip_build)?;

    let test_name_snake = test_name.map(|name| name.to_snake_case());

    if force {
        generator.initialize(program_name, test_name_snake).await?;
        show_howto();
    } else {
        check_trident_initialized(&root)?;
        generator.initialize(program_name, test_name_snake).await?;
        show_howto();
    }
}
