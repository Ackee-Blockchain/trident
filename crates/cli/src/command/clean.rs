use anyhow::Error;
use fehler::throws;
use trident_client::___private::Commander;

use crate::command::check_trident_uninitialized;
use crate::command::get_project_root_for_fuzz;

#[throws]
pub(crate) async fn clean() {
    // Clean command works for both Anchor and vanilla Solana
    // Use empty idl_paths - will find Anchor.toml or trident-tests directory
    let root = get_project_root_for_fuzz(&[])?;

    check_trident_uninitialized(&root)?;
    let commander = Commander::new(&root);
    commander.clean_target().await?;
}
