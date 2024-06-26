use anyhow::{bail, Error};
use fehler::throws;
use trident_client::___private::Commander;

use crate::_discover;

use super::fuzz::TRIDENT_TOML;

#[throws]
pub async fn test(root: Option<String>) {
    // if the root is present from the command line we will use it
    // if the root is not present we will look for the Trident.toml file
    let root = match root {
        Some(r) => r,
        _ => {
            if let Some(r) = _discover(TRIDENT_TOML)? {
                r
            } else {
                bail!("It does not seem that Trident is initialized because the Trident.toml file was not found in any parent directory!");
            }
        }
    };
    let commander = Commander::with_root(root);
    commander.build_anchor_project().await?;
    commander.run_tests().await?;
}
