pub mod accounts_storage;
pub mod error;
pub mod fuzz_client_impl;
pub mod fuzz_stats;
pub mod traits;

pub mod trident_accounts;

pub mod trident_pubkey;

pub mod types;

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

    /// fuzzing
    pub use afl::fuzz as fuzz_afl;
    pub use arbitrary;
    pub use arbitrary::Arbitrary;
    pub use honggfuzz::fuzz as fuzz_honggfuzz;

    /// trident traits
    pub use super::traits::AccountsMethods;
    pub use super::traits::FuzzClient;
    pub use super::traits::InstructionGetters;
    pub use super::traits::InstructionHooks;
    pub use super::traits::InstructionSetters;

    pub use super::traits::RemainingAccountsMethods;
    pub use super::traits::TransactionGetters;
    pub use super::traits::TransactionHooks;
    pub use super::traits::TransactionMethods;
    pub use super::traits::TransactionSelector;
    pub use super::traits::TransactionSetters;
    /// trident derive
    pub use trident_derive_accounts::TridentAccounts;
    pub use trident_derive_flow_executor::flow;
    pub use trident_derive_flow_executor::flow_executor;
    pub use trident_derive_flow_executor::flow_ignore;
    pub use trident_derive_flow_executor::init;

    pub use trident_derive_instruction::TridentInstruction;
    pub use trident_derive_remaining_accounts::TridentRemainingAccounts;
    pub use trident_derive_transaction::TridentTransaction;
    pub use trident_derive_transaction_selector::TransactionSelector;
    /// trident svm
    pub use trident_svm::processor;

    /// accounts storages
    pub use super::accounts_storage::account_storage::AccountsStorage;
    pub use super::accounts_storage::AccountMetadata;
    pub use super::accounts_storage::PdaSeeds;

    pub use trident_config::TridentConfig;

    pub use super::error::*;
    pub use super::fuzz_stats::FuzzingStatistics;

    pub use std::cell::RefCell;
    pub use std::collections::HashMap;
    pub use trident_svm::trident_svm::TridentSVM;
    pub use trident_svm::utils::ProgramEntrypoint;

    /// types
    pub use crate::types::AccountId;
    pub use crate::types::FuzzerData;

    /// trident accounts
    pub use crate::trident_accounts::TridentAccount;
    pub use crate::trident_pubkey::TridentPubkey;

    pub use borsh::{BorshDeserialize, BorshSerialize};

    pub use arbitrary::Unstructured;
}
