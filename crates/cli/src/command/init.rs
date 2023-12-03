use anyhow::{bail, Error};
use fehler::throws;
use trdelnik_client::WorkspaceBuilder;

use crate::{discover, ProgramArch};

pub const ANCHOR_TOML: &str = "Anchor.toml";
pub const CARGO_TOML: &str = "Cargo.toml";

#[throws]
pub async fn init(skip_fuzzer: bool, arch: ProgramArch) {
    // TODO maybe remove skip fuzzer option ?
    if skip_fuzzer {
        // if skipping fuzzer no need to look for anchor.toml
        let root = if let Some(r) = discover(CARGO_TOML)? {
            r
        } else {
            bail!("It does not seem that project is initialized because the Cargo.toml file was not found in any parent directory!");
        };
        let mut generator = WorkspaceBuilder::new_with_root(root);
        let arch = arch.build_subcommand();
        generator.initialize_without_fuzzer(arch).await?;
    } else {
        // fuzzer only supported with anchor
        let root = if let Some(r) = discover(ANCHOR_TOML)? {
            r
        } else {
            bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
        };
        let mut generator = WorkspaceBuilder::new_with_root(root);
        let arch = arch.build_subcommand();
        generator.initialize_with_fuzzer(arch).await?;
    }
}
