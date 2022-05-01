use anyhow::Error;
use fehler::throws;
use trdelnik_client::TestGenerator;

#[throws]
pub async fn init() {
    let generator = TestGenerator::new();
    generator.generate().await?;
}
