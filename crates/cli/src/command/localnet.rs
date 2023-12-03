use anyhow::{bail, Error};
use fehler::throws;
use tokio::signal;
use trdelnik_client::*;

use crate::discover;
pub const TRDELNIK_TOML: &str = "Trdelnik.toml";
#[throws]
pub async fn localnet() {
    // TODO this is only starting local validator , so maybe no need to check for Trdelnik.toml
    let root = if let Some(r) = discover(TRDELNIK_TOML)? {
        r
    } else {
        bail!("It does not seem that Trdelnik is initialized because the {} file was not found in any parent directory!",TRDELNIK_TOML);
    };

    let validator_handle = Commander::start_localnet(&root).await?;

    // wait for SIGINT (^C) signal
    signal::ctrl_c().await.expect("failed to listen for event");

    validator_handle.stop_and_remove_ledger().await?;
}
