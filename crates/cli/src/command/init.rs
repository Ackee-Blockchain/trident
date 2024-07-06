use anyhow::{bail, Error};
use clap::ValueEnum;
use fehler::throws;
use trident_client::___private::TestGenerator;

use crate::_discover;

pub const ANCHOR_TOML: &str = "Anchor.toml";

#[derive(ValueEnum, Clone)]
pub enum TestsType {
    Both,
    Fuzz,
    Poc,
}
#[derive(ValueEnum, Clone)]
pub enum SnapshotsType {
    Macro,
    File,
}

#[throws]
pub async fn init(tests_type: TestsType, snapshots_type: SnapshotsType) {
    // look for Anchor.toml
    let root = if let Some(r) = _discover(ANCHOR_TOML)? {
        r
    } else {
        bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
    };

    let mut generator: TestGenerator = match snapshots_type {
        SnapshotsType::Macro => TestGenerator::new_with_root(root, false),
        SnapshotsType::File => TestGenerator::new_with_root(root, true),
    };

    match tests_type {
        TestsType::Poc => {
            generator.generate_poc().await?;
        }
        TestsType::Both => {
            generator.generate_both().await?;
        }
        TestsType::Fuzz => {
            generator.generate_fuzz().await?;
        }
    };
}
