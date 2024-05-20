use anyhow::Error;
use fehler::throws;
use trident_client::Cleaner;

#[throws]
pub async fn clean() {
    let cleaner = Cleaner::new();
    cleaner.clean_target().await?;
}
