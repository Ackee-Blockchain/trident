pub mod address_storage;
pub mod error;
pub mod trident;
pub mod trident_rng;

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

    /// Trident derive
    pub use trident_derive_flow_executor::end;
    pub use trident_derive_flow_executor::flow;
    pub use trident_derive_flow_executor::flow_executor;
    pub use trident_derive_flow_executor::init;
    pub use trident_derive_fuzz_test_methods::FuzzTestMethods;

    /// Trident svm
    pub use trident_svm::prelude;
    pub use trident_svm::processor;
    pub use trident_svm::trident_svm::TridentSVM;
    #[cfg(feature = "syscall-v2")]
    pub use trident_svm::types::trident_entrypoint::TridentEntrypoint;
    pub use trident_svm::types::trident_program::TridentProgram;

    /// Accounts storages
    pub use super::address_storage::AddressStorage;
    pub use super::address_storage::PdaSeeds;

    /// Trident config
    pub use trident_config::TridentConfig;

    /// Trident
    pub use super::trident::Trident;
    pub use trident_fuzz_metrics::TridentFuzzingData;

    /// Error
    pub use super::error::*;

    pub use borsh;
    pub use borsh::BorshDeserialize;
    pub use borsh::BorshSerialize;

    pub use getrandom;
    pub use indicatif;

    // coverage
    pub use reqwest;
    pub use tokio;

    #[cfg(feature = "token")]
    pub use super::trident::token2022::*;

    extern "C" {
        pub fn __llvm_profile_set_filename(filename: *const i8);
        pub fn __llvm_profile_write_file() -> i32;
        pub fn __llvm_profile_reset_counters();
    }
}
