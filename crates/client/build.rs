use anyhow::Result;
// todo: use vergen before you want to uncomment comments in `generate_program_client_deps`
//  (commander.rs lines 198 - 207)
//  - this code is commented because of a problem with vergen's `git` feature during publishing the crate
//  - the problem was: it wasn't able to find the repository (commit hashes) because the published crate
//      does not contain the .git folder (make sure this is fixed before the publish)
//  - before you uncomment the mentioned code, don't forget to add the `git` feature to Cargo.toml
// use vergen::{vergen, Config};

fn main() -> Result<()> {
    // `vergen` creates env variables with info about
    // the current commit hash or the crate version.
    // vergen(Config::default())
    Ok(())
}
