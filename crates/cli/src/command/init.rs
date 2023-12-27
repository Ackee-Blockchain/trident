use anyhow::{bail, Error};
use clap::{Parser, ValueEnum};
use fehler::throws;
use trdelnik_client::__private::WorkspaceBuilder;

use crate::discover;

pub const ANCHOR_TOML: &str = "Anchor.toml";
pub const CARGO_TOML: &str = "Cargo.toml";
pub const TRDELNIK_TOML: &str = "Trdelnik.toml";

#[derive(ValueEnum, Parser, Clone, PartialEq, Eq, Debug)]
pub enum InitCommand {
    Fuzz,
    Poc,
    Both,
}

#[throws]
pub async fn init(template: InitCommand) {
    if let Some(_trdelnik_toml) = discover(TRDELNIK_TOML)? {
        bail!("It seems that Trdelnik Workspace is already initialized because the Trdelnik.toml file was found in parent directory!");
    }
    match template {
        InitCommand::Fuzz => {
            let root = if let Some(r) = discover(ANCHOR_TOML)? {
                r
            } else {
                bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
            };
            let mut generator = WorkspaceBuilder::new_with_root(root);
            generator.initialize_fuzz().await?;
        }
        InitCommand::Poc => {
            let root = if let Some(r) = discover(CARGO_TOML)? {
                r
            } else {
                bail!("It does not seem that project is initialized because the Cargo.toml file was not found in any parent directory!");
            };
            let mut generator = WorkspaceBuilder::new_with_root(root);
            generator.initialize_poc().await?;
        }
        InitCommand::Both => {
            // INFO for both we need Anchor as it is stronger condition of fuzzer
            let root = if let Some(r) = discover(ANCHOR_TOML)? {
                r
            } else {
                bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
            };
            let mut generator = WorkspaceBuilder::new_with_root(root);
            generator.initialize_both().await?;
        }
    };
}
