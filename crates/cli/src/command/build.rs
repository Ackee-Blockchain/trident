use anyhow::{bail, Error};
use fehler::throws;
use trident_client::___private::TestGenerator;

use crate::_discover;

use super::fuzz::TRIDENT_TOML;

#[throws]
pub async fn build(root: Option<String>) {
    // if the root is present from the command line we will use it
    // if the root is not present we will look for the Cargo.toml file
    // Trident does not have to be already defined to actually create/build
    // program client
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
    let mut generator: TestGenerator = TestGenerator::new_with_root(root, false);
    generator.build().await?;
}
