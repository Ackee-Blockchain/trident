use std::path::Path;

use anyhow::bail;
use anyhow::Error;
use fehler::throws;

use anyhow::Context;
use anyhow::Result;

mod clean;
mod compare_regression;
mod fuzz;
mod howto;
mod init;
mod server;

pub(crate) use fuzz::FuzzCommand;

pub(crate) use clean::clean;
pub(crate) use compare_regression::compare_regression;
pub(crate) use fuzz::fuzz;
pub(crate) use howto::howto;
pub(crate) use init::init;
pub(crate) use server::server;

pub(crate) const ANCHOR_TOML: &str = "Anchor.toml";
pub(crate) const TRIDENT_TOML: &str = "Trident.toml";
pub(crate) const SKIP: &str = "\x1b[33mSkip\x1b[0m";
pub(crate) const TESTS_WORKSPACE_DIRECTORY: &str = "trident-tests";

#[throws]
fn check_anchor_initialized() -> String {
    let root = if let Some(r) = discover(ANCHOR_TOML)? {
        r
    } else {
        bail!("It does not seem that Anchor is initialized because the Anchor.toml file was not found in any parent directory!");
    };
    root
}

#[throws]
fn check_trident_initialized(root: &str) {
    let trident_tests_dir = Path::new(&root).join(TESTS_WORKSPACE_DIRECTORY);
    let trident_toml_path = trident_tests_dir.join(TRIDENT_TOML);

    if trident_tests_dir.exists() && trident_toml_path.exists() {
        bail!(
            "{SKIP}: It looks like Trident is already initialized.\n\
        Trident.toml was found in {}/{} directory.\n\
        In case you want to reinitialize the workspace use --force/-f flag.",
            root,
            TESTS_WORKSPACE_DIRECTORY
        );
    }
}

#[throws]
fn check_trident_uninitialized(root: &str) {
    let trident_toml_path = Path::new(&root)
        .join(TESTS_WORKSPACE_DIRECTORY)
        .join(TRIDENT_TOML);

    if !trident_toml_path.exists() {
        bail!("It does not seem that Trident is initialized because the Trident.toml file was not found in the trident-tests directory. Please run 'trident init' first.");
    }
}

#[throws]
fn check_fuzz_test_exists(root: &str, fuzz_test_name: &str) {
    let fuzz_test_dir = Path::new(&root)
        .join(TESTS_WORKSPACE_DIRECTORY)
        .join(fuzz_test_name);
    if fuzz_test_dir.exists() {
        bail!(
            "{SKIP} [{}/{}] already exists",
            TESTS_WORKSPACE_DIRECTORY,
            fuzz_test_name
        );
    }
}

fn discover(target: &str) -> Result<Option<String>> {
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
                if filename.to_str() == Some(target) {
                    return Ok(Some(cwd.to_string_lossy().to_string()));
                }
            }
        }

        cwd_opt = cwd.parent();
    }

    Ok(None)
}
