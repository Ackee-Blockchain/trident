use anyhow::Error;
use fehler::throws;
use tokio::signal;
use trident_client::*;

#[throws]
pub async fn localnet() {
    let commander = Commander::new();
    let validator_handle = commander.start_localnet().await?;

    // wait for SIGINT (^C) signal
    signal::ctrl_c().await.expect("failed to listen for event");

    validator_handle.stop_and_remove_ledger().await?;
}
