use anyhow::Error;
use clap::ValueEnum;
use fehler::throws;
use trdelnik_client::TestGenerator;

#[derive(ValueEnum, Clone)]
pub enum InitTemplate {
    Both,
    Fuzz,
    Poc,
}

#[throws]
pub async fn init(template: InitTemplate) {
    let generator: TestGenerator = TestGenerator::new();
    match template {
        InitTemplate::Poc => generator.generate_poc().await?,
        InitTemplate::Both => generator.generate_both().await?,
        InitTemplate::Fuzz => generator.generate_fuzz().await?,
    };
}
