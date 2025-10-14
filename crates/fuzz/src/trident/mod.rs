use trident_config::TridentConfig;
use trident_fuzz_metrics::TridentFuzzingData;
use trident_svm::trident_svm::TridentSVM;
use trident_svm::types::trident_account::TridentAccountSharedData;
use trident_svm::types::trident_program::TridentProgram;

use crate::fuzzing::TridentRng;

mod client;
mod metrics;
mod random;
mod seed;
#[cfg(feature = "stake")]
mod stake;
#[cfg(feature = "token")]
pub mod token;
#[cfg(feature = "token")]
pub mod token2022;
#[cfg(feature = "vote")]
mod vote;

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
        let program_binaries =
            config
                .programs()
                .iter()
                .fold(Vec::new(), |mut sbf_programs, config_program| {
                    let target = TridentProgram::new(
                        config_program.address,
                        config_program.upgrade_authority,
                        config_program.data.clone(),
                    );

                    sbf_programs.push(target);
                    sbf_programs
                });

        let permanent_accounts =
            config
                .accounts()
                .iter()
                .fold(Vec::new(), |mut permanent_accounts, config_account| {
                    let account = TridentAccountSharedData::new(
                        config_account.pubkey,
                        config_account.account.clone(),
                    );
                    permanent_accounts.push(account);
                    permanent_accounts
                });

        let mut svm_builder = TridentSVM::builder();
        svm_builder.with_syscalls_v1();
        svm_builder.with_syscalls_v2();
        svm_builder.with_sbf_programs(program_binaries);
        svm_builder.with_permanent_accounts(permanent_accounts);

        if std::env::var("TRIDENT_FUZZ_DEBUG_PATH").is_ok()
            && std::env::var("TRIDENT_FUZZ_DEBUG").is_ok()
        {
            let debug_path =
                std::env::var("TRIDENT_FUZZ_DEBUG_PATH").unwrap_or("trident_debug.log".to_string());
            svm_builder.with_debug_file_logs(&debug_path);
        } else if std::env::var("TRIDENT_LOG").is_ok() {
            svm_builder.with_cli_logs();
        }

        svm_builder.build()
    }
}
