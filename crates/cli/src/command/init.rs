use std::path::Path;

use anyhow::{bail, Error};
use fehler::throws;
use trident_client::___private::TestGenerator;

use crate::{_discover, show_howto};

pub const ANCHOR_TOML: &str = "Anchor.toml";
pub const TRIDENT_TOML: &str = "Trident.toml";
pub const SKIP: &str = "\x1b[33mSkip\x1b[0m";

#[throws]
pub async fn init(force: bool) {
    // look for Anchor.toml
    let root = if let Some(r) = _discover(ANCHOR_TOML)? {
        r
    } else {
        bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
    };

    let mut generator: TestGenerator = TestGenerator::new_with_root(&root)?;

    if force {
        generator.initialize().await?;
        show_howto();
    } else {
        let root_path = Path::new(&root).join(TRIDENT_TOML);
        if root_path.exists() {
            println!(
                "{SKIP}: It looks like Trident is already initialized.\n\
            Trident.toml was found in {} directory.\n\
            In case you want to reinitialize the workspace use --force/-f flag.",
                root
            );
        } else {
            generator.initialize().await?;
            show_howto();
        }
    }
}
