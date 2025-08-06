pub mod accounts_storage;
pub mod error;
pub mod fuzz_client_impl;
pub mod traits;

pub mod trident_accounts;
pub mod trident_pubkey;

pub mod trident_rng;

pub mod types;

pub mod trident;

pub mod fuzzing {
    /// solana_sdk
    pub use solana_sdk;
    pub use solana_sdk::account::AccountSharedData;
    pub use solana_sdk::account::ReadableAccount;
    pub use solana_sdk::account::WritableAccount;
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

    /// Trident RNG
    pub use super::trident_rng::TridentRng;
    pub use hex;

    /// Trident traits
    pub use super::traits::AccountsMethods;
    pub use super::traits::FuzzClient;
    pub use super::traits::InstructionGetters;
    pub use super::traits::InstructionHooks;
    pub use super::traits::InstructionSetters;
    pub use super::traits::RemainingAccountsMethods;
    pub use super::traits::TransactionGetters;
    pub use super::traits::TransactionHooks;
    pub use super::traits::TransactionPrivateMethods;
    pub use super::traits::TransactionSetters;

    /// Trident derive
    pub use trident_derive_accounts::TridentAccounts;
    pub use trident_derive_flow_executor::end;
    pub use trident_derive_flow_executor::flow;
    pub use trident_derive_flow_executor::flow_executor;
    pub use trident_derive_flow_executor::init;
    pub use trident_derive_fuzz_test_methods::FuzzTestMethods;

    pub use trident_derive_instruction::TridentInstruction;
    pub use trident_derive_remaining_accounts::TridentRemainingAccounts;
    pub use trident_derive_transaction::TridentTransaction;

    /// Trident svm
    pub use trident_svm::prelude;
    pub use trident_svm::processor;
    pub use trident_svm::trident_svm::TridentSVM;
    #[cfg(any(feature = "syscall-v1", feature = "syscall-v2"))]
    pub use trident_svm::types::trident_entrypoint::TridentEntrypoint;
    pub use trident_svm::types::trident_program::TridentProgram;

    /// Accounts storages
    pub use super::accounts_storage::account_storage::AccountsStorage;
    pub use super::accounts_storage::AccountMetadata;
    pub use super::accounts_storage::PdaSeeds;

    /// Trident config
    pub use trident_config::TridentConfig;

    /// Trident
    pub use super::trident::Trident;
    pub use trident_fuzz_metrics::TridentFuzzingData;

    /// Error
    pub use super::error::*;

    /// Trident accounts
    pub use crate::trident_accounts::TridentAccount;
    pub use crate::trident_pubkey::TridentPubkey;

    pub use borsh;
    pub use borsh::BorshDeserialize;
    pub use borsh::BorshSerialize;

    pub use getrandom;
    pub use indicatif;

    // coverage
    pub use reqwest;
    pub use tokio;

    extern "C" {
        pub fn __llvm_profile_set_filename(filename: *const i8);
        pub fn __llvm_profile_write_file() -> i32;
        pub fn __llvm_profile_reset_counters();
    }
}
