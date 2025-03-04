//! Trident is a suite of tools and libraries for testing, auditing and developing
//! [Solana](https://solana.com/) / [Anchor](https://book.anchor-lang.com/chapter_1/what_is_anchor.html) programs (smart contracts).
//!
//! Trident could be useful for writing Rust dApps, too.

mod cleaner;
mod commander;
mod idl_loader;
// mod source_code_generators;
mod test_generator;
mod test_generator_gitignore;
mod test_generator_manifest;
mod test_generator_template;
mod utils;

pub mod ___private {
    pub use super::cleaner::*;
    pub use super::commander::Commander;
    pub use super::commander::Error;
    pub use super::idl_loader::*;
    // pub use super::source_code_generators::*;
    pub use super::test_generator::TestGenerator;
}

mod constants {
    // tomls
    pub(crate) const CARGO_TOML: &str = "Cargo.toml";
    pub(crate) const TRIDENT_TOML: &str = "Trident.toml";
    pub(crate) const ANCHOR_TOML: &str = "Anchor.toml";

    // tests
    pub(crate) const TESTS_WORKSPACE_DIRECTORY: &str = "trident-tests";
    pub(crate) const INSTRUCTIONS_DIRECTORY: &str = "instructions";
    pub(crate) const TRANSACTIONS_DIRECTORY: &str = "transactions";

    // fuzz
    // fuzz
    pub(crate) const FUZZ_TRANSACTIONS_FILE_NAME: &str = "fuzz_transactions.rs";
    pub(crate) const TYPES_FILE_NAME: &str = "types.rs";
    pub(crate) const FUZZ_TEST: &str = "test_fuzz.rs";

    // honggfuzz
    pub(crate) const CARGO_TARGET_DIR_DEFAULT_HFUZZ: &str =
        "trident-tests/fuzzing/honggfuzz/hfuzz_target";

    // afl
    pub(crate) const CARGO_TARGET_DIR_DEFAULT_AFL: &str = "trident-tests/fuzzing/afl/afl_target";

    // workspace
    pub(crate) const GIT_IGNORE: &str = ".gitignore";

    // Formatting
    pub(crate) const SKIP: &str = "\x1b[33mSkip\x1b[0m";
    pub(crate) const FINISH: &str = "\x1b[92mFinished\x1b[0m";
    pub(crate) const ERROR: &str = "\x1b[31mError\x1b[0m";
}
