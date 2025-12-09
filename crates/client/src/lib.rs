//! Trident is a suite of tools and libraries for testing, auditing and developing.
//! [Solana](https://solana.com/) / [Anchor](https://book.anchor-lang.com/chapter_1/what_is_anchor.html) programs (smart contracts).
//!
//! Trident could be useful for writing Rust dApps, too.

mod commander;
mod coverage;
mod error;
mod idl_loader;
mod server;
mod test_generator;
mod test_generator_manifest;
mod test_generator_template;
mod utils;

pub mod ___private {
    pub use super::commander::Commander;
    pub use super::commander::Error;
    pub use super::idl_loader::load_idls;
    pub use super::idl_loader::load_idls_from_files;
    pub use super::idl_loader::IdlError;
    pub use super::server::DashboardServer;
    pub use super::test_generator::ProjectType;
    pub use super::test_generator::TestGenerator;
}

mod constants {
    // Tomls
    pub(crate) const CARGO_TOML: &str = "Cargo.toml";
    pub(crate) const TRIDENT_TOML: &str = "Trident.toml";

    // Tests
    /// Directory name for the Trident tests workspace
    /// To customize: change this value and VSCODE_TESTS_WORKSPACE_PATH accordingly
    pub(crate) const TESTS_WORKSPACE_DIRECTORY: &str = "trident-tests";

    // VSCode - Paths relative to project root
    pub(crate) const VSCODE_DIRECTORY: &str = ".vscode";
    pub(crate) const VSCODE_SETTINGS: &str = "settings.json";
    /// Path to the tests workspace for VSCode settings (relative to project root)
    /// This is used in rust-analyzer.linkedProjects configuration
    /// To customize: ensure this matches TESTS_WORKSPACE_DIRECTORY
    /// Format: "./{TESTS_WORKSPACE_DIRECTORY}/Cargo.toml"
    pub(crate) const VSCODE_TESTS_WORKSPACE_PATH: &str = "./trident-tests/Cargo.toml";

    // Fuzzing
    pub(crate) const FUZZ_ACCOUNTS_FILE_NAME: &str = "fuzz_accounts.rs";
    pub(crate) const TYPES_FILE_NAME: &str = "types.rs";
    pub(crate) const FUZZ_TEST: &str = "test_fuzz.rs";

    // Formatting
    pub(crate) const SKIP: &str = "\x1b[33mSkip\x1b[0m";
    pub(crate) const FINISH: &str = "\x1b[92mFinished\x1b[0m";
    pub(crate) const UPDATED: &str = "\x1b[94mUpdated\x1b[0m";
}
