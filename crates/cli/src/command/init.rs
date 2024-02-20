use anyhow::{bail, Error};
use clap::ValueEnum;
use fehler::throws;
use trdelnik_client::TestGenerator;

use crate::_discover;

pub const CARGO_TOML: &str = "Cargo.toml";
pub const ANCHOR_TOML: &str = "Anchor.toml";

#[derive(ValueEnum, Clone)]
pub enum InitTemplate {
    Both,
    Fuzz,
    Poc,
}

#[throws]
pub async fn init(template: InitTemplate) {
    // based on the selected option, obtain root
    // this means as we only support fuzzing of Anchor programs
    // we will look for Anchor.toml in case of the Both or Fuzz Tests
    match template {
        InitTemplate::Poc => {
            // look for Cargo.toml - as we do not strictly need Anchor.toml
            let root = if let Some(r) = _discover(CARGO_TOML)? {
                r
            } else {
                bail!("It does not seem that Solana Project is initialized because the Cargo.toml file was not found in any parent directory!");
            };
            let mut generator: TestGenerator = TestGenerator::new_with_root(root);
            generator.generate_poc().await?;
        }
        InitTemplate::Both => {
            // look for Anchor.toml - as we support only fuzzing of Anchor Projects
            let root = if let Some(r) = _discover(ANCHOR_TOML)? {
                r
            } else {
                bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
            };
            let mut generator: TestGenerator = TestGenerator::new_with_root(root);
            generator.generate_both().await?;
        }
        InitTemplate::Fuzz => {
            // look for Anchor.toml - fuzzer has stronger privilege here
            let root = if let Some(r) = _discover(ANCHOR_TOML)? {
                r
            } else {
                bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
            };
            let mut generator: TestGenerator = TestGenerator::new_with_root(root);
            generator.generate_fuzz().await?;
        }
    };
}
