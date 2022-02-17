use anyhow::Error;
use fehler::throws;

#[throws]
#[tokio::main]
async fn main() {
    trdelnik_cli::start().await?
}
