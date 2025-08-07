use anyhow::Error;
use fehler::throws;
use trident_client::___private::Commander;

use crate::command::check_anchor_initialized;
use crate::command::check_trident_uninitialized;

#[throws]
pub(crate) async fn clean() {
    let root = check_anchor_initialized()?;

    check_trident_uninitialized(&root)?;
    let commander = Commander::new(&root);
    commander.clean_target().await?;
}
