use anyhow::{bail, Error};
use fehler::throws;
use trdelnik_client::*;

use crate::_discover;

use super::fuzz::TRDELNIK_TOML;

#[throws]
pub async fn build(root: Option<String>) {
    // if the root is present from the command line we will use it
    // if the root is not present we will look for the Cargo.toml file
    // Trdelnik does not have to be already defined to actually create/build
    // program client
    let root = match root {
        Some(r) => r,
        _ => {
            if let Some(r) = _discover(TRDELNIK_TOML)? {
                r
            } else {
                bail!("It does not seem that Trdelnik is initialized because the Trdelnik.toml file was not found in any parent directory!");
            }
        }
    };
    let mut generator: TestGenerator = TestGenerator::new_with_root(root);
    generator.build().await?;
}
