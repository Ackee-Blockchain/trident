use anyhow::{bail, Context, Error, Result};

use fehler::throws;
use trdelnik_client::Commander;

#[throws]
pub async fn fuzz(root: Option<String>, target: String) {
    let root = match root {
        Some(r) => r,
        _ => {
            let root = _discover()?;
            if let Some(r) = root {
                r
            } else {
                bail!("It does not seem that Trdelnik is initialized because the Trdelnik.toml file was not found in any parent directory!");
            }
        }
    };

    let commander = Commander::with_root(root);
    commander.run_fuzzer(target).await?;
}

// Climbs each parent directory until we find Trdelnik.toml.
fn _discover() -> Result<Option<String>> {
    let _cwd = std::env::current_dir()?;
    let mut cwd_opt = Some(_cwd.as_path());

    while let Some(cwd) = cwd_opt {
        for f in std::fs::read_dir(cwd)
            .with_context(|| format!("Error reading the directory with path: {}", cwd.display()))?
        {
            let p = f
                .with_context(|| {
                    format!("Error reading the directory with path: {}", cwd.display())
                })?
                .path();
            if let Some(filename) = p.file_name() {
                if filename.to_str() == Some("Trdelnik.toml") {
                    return Ok(Some(cwd.to_string_lossy().to_string()));
                }
            }
        }

        cwd_opt = cwd.parent();
    }

    Ok(None)
}
