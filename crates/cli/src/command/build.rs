use anyhow::Error;
use fehler::throws;
use trdelnik_client::*;

#[throws]
pub async fn build(_root: String) {
    // TODO do not forget to check if trdelnik is initialized
    let mut generator = TestGenerator::new();
    generator.build().await?;
}
