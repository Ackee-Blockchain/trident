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

/// Determine if this is an Anchor project by checking for Anchor.toml
#[throws]
fn is_anchor_project() -> bool {
    discover(ANCHOR_TOML)?.is_some()
}

/// Get project root for init command
/// - Anchor: Root = directory with Anchor.toml
/// - Vanilla: Root = current directory
#[throws]
fn get_project_root_for_init(idl_paths: &[String]) -> String {
    if let Some(root) = discover(ANCHOR_TOML)? {
        // Anchor project found
        root
    } else if !idl_paths.is_empty() {
        // Vanilla Solana project - use current directory as root
        std::env::current_dir()?.to_string_lossy().to_string()
    } else {
        // Neither Anchor nor custom IDLs provided
        bail!(
            "No Anchor.toml found in current or parent directories.\n\n\
            For vanilla Solana programs, you must provide IDL file(s) using --idl-path:\n\
            \x1b[92m  trident init --idl-path ./path/to/program.json\x1b[0m\n\n\
            Note: IDLs for vanilla Solana programs must be generated using external tools\n\
            or written manually following the Anchor IDL format."
        );
    }
}

/// Get project root for fuzz commands (add/refresh/run/debug)
/// - Anchor: Root = directory with Anchor.toml
/// - Vanilla: Root = directory containing trident-tests (search upward from cwd)
#[throws]
fn get_project_root_for_fuzz(idl_paths: &[String]) -> String {
    if let Some(root) = discover(ANCHOR_TOML)? {
        // Anchor project found
        root
    } else if !idl_paths.is_empty() {
        // Vanilla Solana project - find directory containing trident-tests
        if let Some(root) = discover_trident_root()? {
            root
        } else {
            bail!(
                "Could not find trident-tests directory.\n\
                Please run this command from the project root or from within trident-tests directory."
            );
        }
    } else {
        // No Anchor.toml and no IDL paths - must be uninitialized
        bail!(
            "No Anchor.toml found in current or parent directories.\n\n\
            For vanilla Solana programs, you must provide IDL file(s) using --idl-path:\n\
            \x1b[92m  trident fuzz add --idl-path ./path/to/program.json\x1b[0m"
        );
    }
}

/// Discover the project root by finding the directory that contains trident-tests
fn discover_trident_root() -> Result<Option<String>> {
    let _cwd = std::env::current_dir()?;
    let mut cwd_opt = Some(_cwd.as_path());

    while let Some(cwd) = cwd_opt {
        let trident_tests_path = cwd.join(TESTS_WORKSPACE_DIRECTORY);
        if trident_tests_path.exists() && trident_tests_path.is_dir() {
            return Ok(Some(cwd.to_string_lossy().to_string()));
        }
        cwd_opt = cwd.parent();
    }

    Ok(None)
}

/// Validate that program_name is not used with vanilla Solana projects
#[throws]
fn validate_program_name_usage(is_anchor: bool, program_name: &Option<String>) {
    if !is_anchor && program_name.is_some() {
        bail!(
            "The --program-name option is only supported for Anchor projects.\n\
            Vanilla Solana projects do not support selective program building."
        );
    }
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

#[throws]
pub(crate) fn check_fuzz_test_not_exists(root: &str, fuzz_test_name: &str) {
    let fuzz_test_dir = Path::new(&root)
        .join(TESTS_WORKSPACE_DIRECTORY)
        .join(fuzz_test_name);
    if !fuzz_test_dir.exists() {
        bail!(
            "Fuzz test [{}/{}] does not exist",
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
