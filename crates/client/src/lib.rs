//! Trident is a suite of tools and libraries for testing, auditing and developing
//! [Solana](https://solana.com/) / [Anchor](https://book.anchor-lang.com/chapter_1/what_is_anchor.html) programs (smart contracts).
//!
//! Trident could be useful for writing Rust dApps, too.

/// Aimed for the fuzz tests
pub mod fuzzing {
    /// anchor_lang
    pub use anchor_lang;
    pub use anchor_lang::solana_program::hash::Hash;
    pub use anchor_lang::InstructionData;
    pub use anchor_lang::Key;
    pub use anchor_lang::ToAccountInfo;
    pub use anchor_lang::ToAccountMetas;

    /// solana_sdk
    pub use solana_sdk;
    pub use solana_sdk::account_info::AccountInfo;
    pub use solana_sdk::entrypoint::ProcessInstruction;
    pub use solana_sdk::instruction::AccountMeta;
    pub use solana_sdk::instruction::Instruction;
    pub use solana_sdk::pubkey::Pubkey;
    pub use solana_sdk::signer::keypair::Keypair;
    pub use solana_sdk::signer::Signer;
    pub use solana_sdk::transaction::Transaction;

    /// arbitrary and honggfuzz
    pub use arbitrary;
    pub use arbitrary::Arbitrary;
    pub use honggfuzz::fuzz;

    /// trident derive
    pub use trident_derive_displayix::DisplayIx;
    pub use trident_derive_fuzz_deserialize::FuzzDeserialize;
    pub use trident_derive_fuzz_test_executor::FuzzTestExecutor;

    /// trident macros
    pub use trident_fuzz::convert_entry;
    pub use trident_fuzz::fuzz_trident;
    pub use trident_fuzz::show_account;
    pub use trident_fuzz::*;

    pub use solana_program_test::processor;
    pub use trident_fuzz::program_test_client_blocking::FuzzingProgram;
    pub use trident_fuzz::program_test_client_blocking::ProgramEntry;

    pub use super::temp_clone::*;
    /// trident methods
    pub use trident_fuzz::accounts_storage::*;
    pub use trident_fuzz::error::*;
    pub use trident_fuzz::fuzz_client::FuzzClient;
    pub use trident_fuzz::fuzz_data::build_ix_fuzz_data;
    pub use trident_fuzz::fuzz_data::*;
    pub use trident_fuzz::fuzz_deserialize::FuzzDeserialize;
    pub use trident_fuzz::fuzz_stats::FuzzingStatistics;
    pub use trident_fuzz::fuzz_test_executor::FuzzTestExecutor;
    pub use trident_fuzz::ix_ops::IxOps;
    pub use trident_fuzz::program_test_client_blocking::ProgramTestClientBlocking;
    pub use trident_fuzz::snapshot::Snapshot;

    pub use std::cell::RefCell;
    pub use std::collections::HashMap;
}

/// Aimed for the integration tests
pub mod test {
    pub use super::client::*;
    pub use super::error_reporter::report_error;
    pub use super::keys::*;
    pub use super::tester::Tester;
    pub use anyhow::{self, Error, Result};
    pub use futures::{self, FutureExt};
    pub use rstest::*;
    pub use serial_test;
    pub use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
    pub use tokio;
    pub use trident_test::trident_test;
}

/// Aimed for general usage
pub mod prelude {
    pub use super::temp_clone::*;
    pub use anchor_client::ClientError;
    pub use anchor_lang;
    pub use anchor_lang::InstructionData;
    pub use anchor_lang::ToAccountMetas;
    pub use solana_sdk;
    pub use solana_sdk::instruction::Instruction;
    pub use solana_sdk::pubkey::Pubkey;
    pub use solana_sdk::signer::keypair::Keypair;
    pub use solana_sdk::signer::Signer;
}

mod cleaner;
mod client;
mod commander;
mod config;
mod error_reporter;
mod idl;
mod keys;
mod reader;
mod source_code_generators;
mod temp_clone;
mod test_generator;
mod tester;

pub mod ___private {
    pub use super::cleaner::*;
    pub use super::client::*;
    pub use super::commander::Commander;
    pub use super::commander::Error;
    pub use super::commander::LocalnetHandle;
    pub use super::error_reporter::*;
    pub use super::idl::*;
    pub use super::keys::*;
    pub use super::reader::*;
    pub use super::source_code_generators::*;
    pub use super::temp_clone::TempClone;
    pub use super::test_generator::ProgramData;
    pub use super::test_generator::TestGenerator;
    pub use super::tester::*;
    // pub use trident_fuzz_test::fuzz_trident;
}

mod constants {
    // program_client
    pub const PROGRAM_CLIENT_DIRECTORY: &str = ".program_client";
    pub const LIB: &str = "lib.rs";
    pub const SRC_DIRECTORY: &str = "src";

    // tomls
    pub const CARGO_TOML: &str = "Cargo.toml";
    pub const TRIDENT_TOML: &str = "Trident.toml";
    pub const ANCHOR_TOML: &str = "Anchor.toml";

    // tests
    pub const TESTS_WORKSPACE_DIRECTORY: &str = "trident-tests";

    // poc
    pub const POC_TEST_DIRECTORY: &str = "poc_tests";
    pub const TESTS_DIRECTORY: &str = "tests";
    pub const POC_TEST: &str = "test.rs";

    // fuzz
    pub const ACCOUNTS_SNAPSHOTS_FILE_NAME: &str = "accounts_snapshots.rs";
    pub const FUZZ_INSTRUCTIONS_FILE_NAME: &str = "fuzz_instructions.rs";
    pub const FUZZ_TEST_DIRECTORY: &str = "fuzz_tests";
    pub const FUZZING: &str = "fuzzing";
    pub const FUZZ_TEST: &str = "test_fuzz.rs";
    pub const HFUZZ_TARGET: &str = "hfuzz_target";
    pub const CARGO_TARGET_DIR_DEFAULT: &str = "trident-tests/fuzz_tests/fuzzing/hfuzz_target";
    pub const HFUZZ_WORKSPACE_DEFAULT: &str = "trident-tests/fuzz_tests/fuzzing/hfuzz_workspace";
    pub const CARGO_TARGET_DIR_ENV: &str = "CARGO_TARGET_DIR";
    pub const HFUZZ_WORKSPACE_ENV: &str = "HFUZZ_WORKSPACE";

    // workspace
    pub const GIT_IGNORE: &str = ".gitignore";

    // client
    pub const RETRY_LOCALNET_EVERY_MILLIS: u64 = 500;
    pub const DEFAULT_KEYPAIR_PATH: &str = "~/.config/solana/id.json";

    // Formatting
    pub const SKIP: &str = "\x1b[33mSkip\x1b[0m";
    pub const WARNING: &str = "\x1b[1;93mWarning\x1b[0m";
    pub const FINISH: &str = "\x1b[92mFinished\x1b[0m";
    pub const ERROR: &str = "\x1b[31mError\x1b[0m";

    // special message for the progress bar
    pub const EXPANDING_PROGRESS_BAR: &str = "\x1b[92mExpanding\x1b[0m";
}
