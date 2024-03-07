use anyhow::Error;
use fehler::throws;

#[throws]
#[tokio::main]
async fn main() {
    trident_cli::start().await?
}
