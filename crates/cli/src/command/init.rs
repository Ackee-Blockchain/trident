use anyhow::{bail, Error};

use fehler::throws;
use trdelnik_client::TestGenerator;

use crate::{_check_if_present, _discover};
const CARGO_TOML: &str = "Cargo.toml";
const TRDELNIK_TOML: &str = "Trdelnik.toml";

#[throws]
pub async fn init(skip_fuzzer: bool) {
    // find parent directory with Cargo.toml
    let root = _discover(CARGO_TOML)?;
    if let Some(r) = root {
        // check if the Trdelnik.toml is already present in the root directory
        let present = _check_if_present(&r, TRDELNIK_TOML)?;
        match present {
            true => {
                bail!("It seems that Trdelnik is already initialized because the Trdelnik.toml file was found in the current or parent directory!");
            }
            false => {
                let generator = TestGenerator::new_with_root(&r);
                generator.generate(skip_fuzzer).await?;
            }
        }
    } else {
        bail!("It does not seem that Solana Program is initialized because the Cargo.toml file was not found in any parent directory!");
    };
}
