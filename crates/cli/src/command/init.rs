use anyhow::Error;
use fehler::throws;
use trdelnik_client::TestGenerator;

#[throws]
pub async fn init(skip_fuzzer: bool) {
    let mut generator = TestGenerator::new();
    generator.generate(skip_fuzzer).await?;
}
