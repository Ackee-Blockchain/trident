use trident_config::TridentConfig;
use trident_fuzz_metrics::TridentFuzzingData;
use trident_svm::trident_svm::TridentSVM;
use trident_svm::types::trident_account::TridentAccountSharedData;

use crate::trident_rng::TridentRng;

mod client;
pub mod flow_executor;
mod system;
mod transaction_result;

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
        for (pubkey, account) in config.get_forked_accounts() {
            let forked_account = TridentAccountSharedData::new(pubkey, account.clone());
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
