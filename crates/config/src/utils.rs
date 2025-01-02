use crate::{argument::Argument, Error, ANCHOR_TOML};
use anyhow::Context;
use fehler::throw;
use std::{
    env,
    path::{Path, PathBuf},
};

pub(crate) fn resolve_path(filename: &str) -> PathBuf {
    let path = Path::new(filename);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        discover_root()
            .map(|cwd| cwd.join(path))
            .unwrap_or_else(|_| panic!("Failed to resolve relative path: {}", path.display()))
    }
}

pub(crate) fn arg_to_string(arg: &Argument) -> Vec<String> {
    let val = arg.value.clone().unwrap_or_default();
    if let Some(opt) = &arg.short_opt {
        vec![opt.clone(), val]
    } else if let Some(opt) = &arg.long_opt {
        vec![opt.clone(), val]
    } else {
        vec![]
    }
}

/// Tries to find the root directory with the `Anchor.toml` file.
/// Throws an error when there is no directory with the `Anchor.toml` file
pub fn discover_root() -> Result<PathBuf, Error> {
    let current_dir = env::current_dir()?;
    let mut dir = Some(current_dir.as_path());
    while let Some(cwd) = dir {
        for file in std::fs::read_dir(cwd)
            .with_context(|| format!("Error reading the directory with path: {}", cwd.display()))?
        {
            let path = file
                .with_context(|| {
                    format!("Error reading the directory with path: {}", cwd.display())
                })?
                .path();
            if let Some(filename) = path.file_name() {
                if filename.to_str() == Some(ANCHOR_TOML) {
                    return Ok(PathBuf::from(cwd));
                }
            }
        }
        dir = cwd.parent();
    }
    throw!(Error::BadWorkspace)
}
