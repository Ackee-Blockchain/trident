//! Trident is a suite of tools and libraries for testing, auditing and developing
//! [Solana](https://solana.com/) / [Anchor](https://book.anchor-lang.com/chapter_1/what_is_anchor.html) programs (smart contracts).
//!
//! Trident could be useful for writing Rust dApps, too.

mod cleaner;
mod commander;
mod idl_loader;
// mod source_code_generators;
mod test_generator;
mod utils;
mod versions_config;

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
    pub const CARGO_TOML: &str = "Cargo.toml";
    pub const TRIDENT_TOML: &str = "Trident.toml";
    pub const ANCHOR_TOML: &str = "Anchor.toml";

    // tests
    pub const TESTS_WORKSPACE_DIRECTORY: &str = "trident-tests";

    // fuzz
    pub const FUZZ_INSTRUCTIONS_FILE_NAME: &str = "fuzz_instructions.rs";
    pub const FUZZ_TEST: &str = "test_fuzz.rs";

    // honggfuzz
    pub const CARGO_TARGET_DIR_DEFAULT_HFUZZ: &str = "trident-tests/fuzzing/honggfuzz/hfuzz_target";

    // afl
    pub const CARGO_TARGET_DIR_DEFAULT_AFL: &str = "trident-tests/fuzzing/afl/afl_target";

    // workspace
    pub const GIT_IGNORE: &str = ".gitignore";

    // Formatting
    pub const SKIP: &str = "\x1b[33mSkip\x1b[0m";
    pub const FINISH: &str = "\x1b[92mFinished\x1b[0m";
    pub const ERROR: &str = "\x1b[31mError\x1b[0m";
}
