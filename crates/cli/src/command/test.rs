use anyhow::{bail, Error};
use fehler::throws;
use trdelnik_client::*;

use crate::discover;

pub const TRDELNIK_TOML: &str = "Trdelnik.toml";

#[throws]
pub async fn test(root: String) {
    // TODO root argument maybe not needed

    match discover(TRDELNIK_TOML)? {
        Some(_) => Commander::run_tests().await?,
        _ => {
            bail!("It does not seem that Trdelnik is initialized because the Trdelnik.toml file was not found in any parent directory!")
        }
    }
}
