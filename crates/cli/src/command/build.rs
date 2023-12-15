use crate::{discover, ProgramArch};
use anyhow::{bail, Error};
use fehler::throws;
use trdelnik_client::*;

pub const TRDELNIK_TOML: &str = "Trdelnik.toml";

#[throws]
pub async fn build(root: String, arch: ProgramArch) {
    // FIXME root argument maybe not needed
    let root = if let Some(r) = discover(TRDELNIK_TOML)? {
        r
    } else {
        bail!("It does not seem that Trdelnik is initialized because the Trdelnik.toml file was not found in any parent directory!");
    };
    let mut builder = WorkspaceBuilder::new_with_root(root);
    let arch = arch.build_subcommand();
    builder.build(arch).await?;
}
