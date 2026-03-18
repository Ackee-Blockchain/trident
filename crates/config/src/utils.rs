use crate::constants::TESTS_WORKSPACE_DIRECTORY;
use crate::constants::TRIDENT_TOML;
use crate::Error;

use anyhow::Context;
use fehler::throw;
use std::env;
use std::path::Path;
use std::path::PathBuf;

pub(crate) fn resolve_path(filename: &str) -> Result<PathBuf, Error> {
    let path = Path::new(filename);
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        discover_root().map(|cwd| cwd.join(path))
    }
}

/// Tries to find the root directory with the `Trident.toml` file.
/// Throws an error when there is no directory with the `Trident.toml` file
pub fn discover_root() -> Result<PathBuf, Error> {
    let current_dir = env::current_dir()?;
    let mut dir = Some(current_dir.as_path());
    while let Some(cwd) = dir {
        // 1) Direct layout: <cwd>/Trident.toml
        let direct_trident_toml = cwd.join(TRIDENT_TOML);
        if direct_trident_toml.exists() {
            return Ok(PathBuf::from(cwd));
        }

        // 2) Workspace layout: <cwd>/trident-tests/Trident.toml
        let nested_trident_toml = cwd.join(TESTS_WORKSPACE_DIRECTORY).join(TRIDENT_TOML);
        if nested_trident_toml.exists() {
            return Ok(cwd.join(TESTS_WORKSPACE_DIRECTORY));
        }

        for file in std::fs::read_dir(cwd)
            .with_context(|| format!("Error reading the directory with path: {}", cwd.display()))?
        {
            let path = file
                .with_context(|| {
                    format!("Error reading the directory with path: {}", cwd.display())
                })?
                .path();
            if let Some(filename) = path.file_name() {
                if filename.to_str() == Some(TRIDENT_TOML) {
                    return Ok(PathBuf::from(cwd));
                }
            }
        }
        dir = cwd.parent();
    }
    throw!(Error::BadWorkspace)
}
