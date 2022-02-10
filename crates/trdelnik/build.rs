use anyhow::Result;
use vergen::{vergen, Config};

fn main() -> Result<()> {
    // `vergen` creates env variables with info about
    // the current commit hash or the crate version.
    vergen(Config::default())
}
