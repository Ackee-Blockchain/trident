use anyhow::Result;
use vergen::{Config, vergen};

fn main() -> Result<()> {
  // Generate the default 'cargo:' instruction output
  vergen(Config::default())
}
