pub mod accounts_storage;
pub mod error;
pub mod fuzz_data;
pub mod fuzz_stats;
pub mod snapshot;
pub type AccountId = u8;
pub mod fuzz_client;
pub mod fuzz_client_impl;
pub mod fuzz_test_executor;
pub mod fuzz_trident;
pub mod instructions_sequence;
pub mod ix_ops;
pub mod transaction_executor;

pub mod fuzzing {
    /// solana_sdk
    pub use solana_sdk;
    pub use solana_sdk::account::AccountSharedData;
    pub use solana_sdk::account::ReadableAccount;
    pub use solana_sdk::account_info::AccountInfo;
    pub use solana_sdk::entrypoint::ProcessInstruction;
    pub use solana_sdk::instruction::AccountMeta;
    pub use solana_sdk::instruction::Instruction;
    pub use solana_sdk::native_token::LAMPORTS_PER_SOL;
    pub use solana_sdk::pubkey;
    pub use solana_sdk::pubkey::Pubkey;
    pub use solana_sdk::signer::keypair::Keypair;
    pub use solana_sdk::signer::Signer;
    pub use solana_sdk::transaction::Transaction;
    pub use solana_sdk::transaction::TransactionError;

    pub use afl::fuzz as fuzz_afl;
    pub use arbitrary;
    pub use arbitrary::Arbitrary;
    pub use honggfuzz::fuzz as fuzz_honggfuzz;

    /// trident derive
    pub use trident_derive_displayix::DisplayIx;
    pub use trident_derive_fuzz_test_executor::FuzzTestExecutor;
    pub use trident_svm::processor;

    /// trident macros
    pub use super::fuzz_trident;
    pub use super::middle_sequence;
    pub use super::post_sequence;
    pub use super::pre_sequence;
    pub use super::show_account;
    pub use super::*;

    /// trident methods
    pub use super::accounts_storage::*;
    pub use trident_config::Config;

    pub use super::error::*;
    pub use super::fuzz_client::FuzzClient;
    pub use super::fuzz_data::build_ix_fuzz_data;
    pub use super::fuzz_data::*;
    pub use super::fuzz_stats::FuzzingStatistics;
    pub use super::fuzz_test_executor::FuzzTestExecutor;
    pub use super::ix_ops::IxOps;
    pub use super::snapshot::Snapshot;
    pub use super::snapshot::SnapshotAccount;
    pub use super::transaction_executor::TransactionExecutor;

    pub use std::cell::RefCell;
    pub use std::collections::HashMap;
    pub use trident_svm::trident_svm::TridentSVM;
    pub use trident_svm::utils::ProgramEntrypoint;

    pub use super::accounts_storage::KeypairStore;
    pub use super::accounts_storage::PdaStore;
}
