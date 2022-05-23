use anyhow::Error;
use fehler::throws;
use trdelnik_client::*;

#[throws]
pub async fn test(root: String) {
    let commander = Commander::with_root(root);
    commander.run_tests().await?;
}
