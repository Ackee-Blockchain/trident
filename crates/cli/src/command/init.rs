use anyhow::{bail, Error};
use fehler::throws;
use trident_client::___private::TestGenerator;

use crate::{_discover, show_howto};

pub const ANCHOR_TOML: &str = "Anchor.toml";
#[throws]
pub async fn init() {
    // look for Anchor.toml
    let root = if let Some(r) = _discover(ANCHOR_TOML)? {
        r
    } else {
        bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
    };

    let mut generator: TestGenerator = TestGenerator::new_with_root(root);

    generator.generate_fuzz().await?;

    show_howto();
}
