use trident_config::TridentConfig;
use trident_fuzz_metrics::TridentFuzzingData;
use trident_svm::trident_svm::TridentSVM;
use trident_svm::types::trident_account::TridentAccountSharedData;

use solana_sdk::account::ReadableAccount;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

use crate::trident_rng::TridentRng;

mod client;
pub mod flow_executor;
mod system;
pub mod transaction_result;

mod metrics;
mod random;
mod seed;
#[cfg(feature = "stake")]
mod stake;
#[cfg(feature = "token")]
mod token;
#[cfg(feature = "token")]
mod token2022;
#[cfg(feature = "vote")]
mod vote;

// Re-export token2022 types
#[cfg(feature = "token")]
pub use token2022::AccountExtension;
#[cfg(feature = "token")]
pub use token2022::MintExtension;
#[cfg(feature = "token")]
pub use token2022::MintExtensionData;
#[cfg(feature = "token")]
pub use token2022::MintWithExtensions;
#[cfg(feature = "token")]
pub use token2022::TokenAccountExtensionData;
#[cfg(feature = "token")]
pub use token2022::TokenAccountWithExtensions;

pub struct Trident {
    pub(crate) client: TridentSVM,
    pub(crate) fuzzing_data: TridentFuzzingData,
    pub(crate) rng: TridentRng,
}

impl Default for Trident {
    fn default() -> Self {
        Self {
            client: Self::new_client(),
            fuzzing_data: TridentFuzzingData::default(),
            rng: TridentRng::default(),
        }
    }
}

impl Trident {
    /// Extract program data address from a v3 upgradeable program account
    fn get_program_data_address(
        account: &solana_sdk::account::AccountSharedData,
    ) -> Option<Pubkey> {
        use solana_loader_v3_interface::state::UpgradeableLoaderState;

        if let Ok(UpgradeableLoaderState::Program {
            programdata_address,
        }) = bincode::deserialize::<UpgradeableLoaderState>(account.data())
        {
            Some(programdata_address)
        } else {
            None
        }
    }

    /// Extract the actual program bytecode from a program data account
    fn extract_program_data(
        program_data_account: &solana_sdk::account::AccountSharedData,
    ) -> Vec<u8> {
        // Program data account structure:
        // - First part: UpgradeableLoaderState::ProgramData metadata
        // - Second part: actual program bytecode

        const PROGRAM_DATA_METADATA_SIZE: usize = 45; // Size of ProgramData metadata

        let data = program_data_account.data();
        if data.len() > PROGRAM_DATA_METADATA_SIZE {
            data[PROGRAM_DATA_METADATA_SIZE..].to_vec()
        } else {
            vec![]
        }
    }

    /// Extract upgrade authority from a program data account
    fn get_upgrade_authority(
        program_data_account: &solana_sdk::account::AccountSharedData,
    ) -> Option<Pubkey> {
        use solana_loader_v3_interface::state::UpgradeableLoaderState;

        if let Ok(UpgradeableLoaderState::ProgramData {
            upgrade_authority_address,
            ..
        }) = bincode::deserialize::<UpgradeableLoaderState>(program_data_account.data())
        {
            upgrade_authority_address
        } else {
            None
        }
    }

    fn new_client() -> TridentSVM {
        let config = TridentConfig::new();
        let mut genesis_accounts = Vec::new();

        // Add programs from config
        for program in config.programs() {
            let accounts = TridentAccountSharedData::loader_v3_program(
                program.address,
                &program.data,
                program.upgrade_authority,
            );
            genesis_accounts.extend(accounts);
        }

        // Add regular accounts from config
        for account_config in config.accounts() {
            let account = TridentAccountSharedData::new(
                account_config.pubkey,
                account_config.account.clone(),
            );
            genesis_accounts.push(account);
        }

        // Add forked accounts
        // We need to handle v3 upgradeable programs specially:
        // - Program accounts need to be loaded with their program data
        // - Program data accounts should not be added separately
        let forked_accounts = config.get_forked_accounts();
        let mut program_data_accounts = std::collections::HashSet::new();

        // First pass: identify program data accounts
        for (_pubkey, account) in &forked_accounts {
            if *account.owner() == solana_sdk::bpf_loader_upgradeable::ID {
                if let Some(program_data_pubkey) = Self::get_program_data_address(account) {
                    program_data_accounts.insert(program_data_pubkey);
                }
            }
        }

        // Second pass: add accounts, handling programs specially
        let forked_map: HashMap<Pubkey, _> = forked_accounts.into_iter().collect();

        for (pubkey, account) in forked_map.iter() {
            // Skip program data accounts - they'll be added with their program
            if program_data_accounts.contains(pubkey) {
                continue;
            }

            // Check if this is a v3 upgradeable program
            if *account.owner() == solana_sdk::bpf_loader_upgradeable::ID {
                if let Some(program_data_pubkey) = Self::get_program_data_address(account) {
                    // This is a program account, load it properly with its data
                    if let Some(program_data_account) = forked_map.get(&program_data_pubkey) {
                        // Extract the actual program bytecode from the program data account
                        let program_data = Self::extract_program_data(program_data_account);

                        // Get upgrade authority from the program data account
                        let upgrade_authority = Self::get_upgrade_authority(program_data_account);

                        // Load as a proper v3 program
                        let program_accounts = TridentAccountSharedData::loader_v3_program(
                            *pubkey,
                            &program_data,
                            upgrade_authority,
                        );
                        genesis_accounts.extend(program_accounts);
                        continue;
                    }
                }
            }

            // Regular account (not a program)
            let forked_account = TridentAccountSharedData::new(*pubkey, account.clone());
            genesis_accounts.push(forked_account);
        }

        // Build SVM with all accounts
        let mut svm_builder = TridentSVM::builder();
        svm_builder.with_syscalls_v1();
        svm_builder.with_syscalls_v2();
        svm_builder.with_permanent_accounts(genesis_accounts);

        // Configure logging
        if std::env::var("TRIDENT_FUZZ_DEBUG_PATH").is_ok()
            && std::env::var("TRIDENT_FUZZ_DEBUG").is_ok()
        {
            let debug_path = std::env::var("TRIDENT_FUZZ_DEBUG_PATH")
                .unwrap_or_else(|_| "trident_debug.log".to_string());
            svm_builder.with_debug_file_logs(&debug_path);
        } else if std::env::var("TRIDENT_LOG").is_ok() {
            svm_builder.with_cli_logs();
        }

        svm_builder.build()
    }
}
