use anyhow::{bail, Error};
use clap::{Parser, ValueEnum};
use fehler::throws;
use trdelnik_client::WorkspaceBuilder;

use crate::{discover, ProgramArch};

pub const ANCHOR_TOML: &str = "Anchor.toml";
pub const CARGO_TOML: &str = "Cargo.toml";

#[derive(ValueEnum, Parser, Clone, PartialEq, Eq, Debug)]
pub enum InitCommand {
    Fuzz,
    Poc,
    Both,
}

#[throws]
pub async fn init(template: InitCommand, arch: ProgramArch) {
    match template {
        InitCommand::Fuzz => {
            let root = if let Some(r) = discover(ANCHOR_TOML)? {
                r
            } else {
                bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
            };
            let mut generator = WorkspaceBuilder::new_with_root(root);
            let arch = arch.build_subcommand();
            generator.initialize_fuzz(arch).await?;
        }
        InitCommand::Poc => {
            let root = if let Some(r) = discover(CARGO_TOML)? {
                r
            } else {
                bail!("It does not seem that project is initialized because the Cargo.toml file was not found in any parent directory!");
            };
            let mut generator = WorkspaceBuilder::new_with_root(root);
            let arch = arch.build_subcommand();
            generator.initialize_poc(arch).await?;
        }
        InitCommand::Both => {
            // INFO for both we need Anchor as it is stronger condition of fuzzer
            let root = if let Some(r) = discover(ANCHOR_TOML)? {
                r
            } else {
                bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
            };
            let mut generator = WorkspaceBuilder::new_with_root(root);
            let arch = arch.build_subcommand();
            generator.initialize_both(arch).await?;
        }
    };
}
