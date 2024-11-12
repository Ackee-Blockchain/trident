//! Trident is a suite of tools and libraries for testing, auditing and developing
//! [Solana](https://solana.com/) / [Anchor](https://book.anchor-lang.com/chapter_1/what_is_anchor.html) programs (smart contracts).
//!
//! Trident could be useful for writing Rust dApps, too.

/// Aimed for the fuzz tests
pub mod fuzzing {
    /// anchor_lang
    pub use anchor_lang;
    pub use anchor_lang::solana_program::hash::Hash;
    pub use anchor_lang::AccountDeserialize;
    pub use anchor_lang::InstructionData;
    pub use anchor_lang::Key;
    pub use anchor_lang::ToAccountInfo;
    pub use anchor_lang::ToAccountMetas;

    /// solana_sdk
    pub use solana_sdk;
    pub use solana_sdk::account::AccountSharedData;
    pub use solana_sdk::account::ReadableAccount;
    pub use solana_sdk::account_info::AccountInfo;
    pub use solana_sdk::entrypoint::ProcessInstruction;
    pub use solana_sdk::instruction::AccountMeta;
    pub use solana_sdk::instruction::Instruction;
    pub use solana_sdk::native_token::LAMPORTS_PER_SOL;
    pub use solana_sdk::pubkey::Pubkey;
    pub use solana_sdk::signer::keypair::Keypair;
    pub use solana_sdk::signer::Signer;
    pub use solana_sdk::transaction::Transaction;

    pub use afl::fuzz as fuzz_afl;
    pub use arbitrary;
    pub use arbitrary::Arbitrary;
    pub use honggfuzz::fuzz as fuzz_honggfuzz;

    /// trident derive
    pub use trident_derive_displayix::DisplayIx;
    pub use trident_derive_fuzz_test_executor::FuzzTestExecutor;

    /// trident macros
    pub use trident_fuzz::convert_entry;
    pub use trident_fuzz::fuzz_trident;
    pub use trident_fuzz::middle_sequence;
    pub use trident_fuzz::post_sequence;
    pub use trident_fuzz::pre_sequence;
    pub use trident_fuzz::show_account;
    pub use trident_fuzz::*;

    pub use solana_program_test::processor;
    pub use trident_fuzz::program_test_client_blocking::FuzzingProgram;
    pub use trident_fuzz::program_test_client_blocking::ProgramEntry;

    pub use super::temp_clone::*;
    /// trident methods
    pub use trident_fuzz::accounts_storage::*;
    pub use trident_fuzz::config::Config;
    pub use trident_fuzz::error::*;
    pub use trident_fuzz::fuzz_client::FuzzClient;
    pub use trident_fuzz::fuzz_data::build_ix_fuzz_data;
    pub use trident_fuzz::fuzz_data::*;
    pub use trident_fuzz::fuzz_stats::FuzzingStatistics;
    pub use trident_fuzz::fuzz_test_executor::FuzzTestExecutor;
    pub use trident_fuzz::ix_ops::IxOps;
    pub use trident_fuzz::program_test_client_blocking::ProgramTestClientBlocking;
    pub use trident_fuzz::snapshot::Snapshot;
    pub use trident_fuzz::snapshot::SnapshotAccount;
    pub use trident_fuzz::transaction_executor::TransactionExecutor;

    pub use std::cell::RefCell;
    pub use std::collections::HashMap;

    pub use trident_fuzz::accounts_storage::KeypairStore;
    pub use trident_fuzz::accounts_storage::MintStore;
    pub use trident_fuzz::accounts_storage::PdaStore;
    pub use trident_fuzz::accounts_storage::ProgramStore;
    pub use trident_fuzz::accounts_storage::StakeStore;
    pub use trident_fuzz::accounts_storage::TokenStore;
    pub use trident_fuzz::accounts_storage::VoteStore;
}

mod anchor_idl;
mod cleaner;
mod commander;
mod source_code_generators;
mod temp_clone;
mod test_generator;
mod utils;
mod versions_config;

pub mod ___private {
    pub use super::anchor_idl::*;
    pub use super::cleaner::*;
    pub use super::commander::Commander;
    pub use super::commander::Error;
    pub use super::source_code_generators::*;
    pub use super::temp_clone::TempClone;
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
    pub const FUZZ_TEST_DIRECTORY: &str = "fuzz_tests";
    pub const FUZZ_TEST: &str = "test_fuzz.rs";

    // honggfuzz
    pub const CARGO_TARGET_DIR_DEFAULT_HFUZZ: &str =
        "trident-tests/fuzz_tests/fuzzing/honggfuzz/hfuzz_target";

    // afl
    pub const CARGO_TARGET_DIR_DEFAULT_AFL: &str =
        "trident-tests/fuzz_tests/fuzzing/afl/afl_target";

    // workspace
    pub const GIT_IGNORE: &str = ".gitignore";

    // Formatting
    pub const SKIP: &str = "\x1b[33mSkip\x1b[0m";
    pub const FINISH: &str = "\x1b[92mFinished\x1b[0m";
    pub const ERROR: &str = "\x1b[31mError\x1b[0m";
}
