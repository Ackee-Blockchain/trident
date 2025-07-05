use solana_sdk::account::AccountSharedData;
use solana_sdk::clock::Clock;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar::Sysvar;

use trident_config::TridentConfig;
use trident_fork_testing::{
    get_forked_programs, ClusterType as ForkClusterType, ForkProgramConfig,
};

use trident_svm::trident_svm::TridentSVM;
use trident_svm::types::trident_account::TridentAccountSharedData;
use trident_svm::types::trident_entrypoint::TridentEntrypoint;
use trident_svm::types::trident_program::TridentProgram;

use crate::traits::FuzzClient;

impl FuzzClient for TridentSVM {
    fn deploy_entrypoint(&mut self, program: TridentEntrypoint) {
        self.deploy_entrypoint_program(&program);
    }

    fn deploy_program(&mut self, program: TridentProgram) {
        self.deploy_binary_program(&program);
    }

    #[doc(hidden)]
    fn new_client(config: &TridentConfig) -> Self {
        let forked_or_cached_programs = config.fork_or_cache_programs();

        // Fork programs from clusters if specified
        let mut program_binaries = Vec::new();

        // Add forked programs
        if !forked_or_cached_programs.is_empty() {
            // Convert trident_config types to fork crate types
            let fork_configs: Vec<ForkProgramConfig> = forked_or_cached_programs
                .iter()
                .map(|config| {
                    let cluster = match config.cluster.to_short_string().as_str() {
                        "m" => ForkClusterType::Mainnet,
                        "d" => ForkClusterType::Devnet,
                        "t" => ForkClusterType::Testnet,
                        "l" => ForkClusterType::Localnet,
                        _ => ForkClusterType::Mainnet, // default
                    };

                    ForkProgramConfig {
                        cluster,
                        address: config.address.clone(),
                        overwrite: config.overwrite,
                    }
                })
                .collect();

            match get_forked_programs(&fork_configs) {
                Ok(forked_programs) => {
                    for forked_program in forked_programs {
                        let target = TridentProgram::new(
                            forked_program.address,
                            None, // TODO add authority to the forked program
                            forked_program.data,
                        );
                        program_binaries.push(target);
                        println!("Added forked program: {}", forked_program.address);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to fork programs: {}", e);
                }
            }
        }

        // Add regular configured programs
        let configured_programs =
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

        program_binaries.extend(configured_programs);

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

        if std::env::var("TRIDENT_FUZZ_DEBUG").is_ok() {
            svm_builder.with_debug_file_logs();
        } else if std::env::var("TRIDENT_LOG").is_ok() {
            svm_builder.with_cli_logs();
        }

        svm_builder.build()
    }
    fn warp_to_epoch(&mut self, warp_epoch: u64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.epoch = warp_epoch;
        self.set_sysvar(&clock);
    }

    fn warp_to_slot(&mut self, warp_slot: u64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.slot = warp_slot;
        self.set_sysvar(&clock);
    }
    fn warp_to_timestamp(&mut self, warp_timestamp: i64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.unix_timestamp = warp_timestamp;
        self.set_sysvar(&clock);
    }

    fn forward_in_time(&mut self, seconds: i64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.unix_timestamp = clock.unix_timestamp.saturating_add(seconds);
        self.set_sysvar(&clock);
    }

    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.set_account(address, account, false);
    }

    fn payer(&self) -> solana_sdk::signature::Keypair {
        self.get_payer()
    }

    fn get_account(&mut self, key: &Pubkey) -> AccountSharedData {
        trident_svm::trident_svm::TridentSVM::get_account(self, key).unwrap_or_default()
    }

    fn get_last_blockhash(&self) -> Hash {
        panic!("Not yet implemented for TridentSVM");
    }

    #[doc(hidden)]
    fn _process_instructions(
        &mut self,
        instructions: &[Instruction],
    ) -> trident_svm::prelude::solana_svm::transaction_processor::LoadAndExecuteSanitizedTransactionsOutput{
        // there should be at least 1 RW fee-payer account.
        // But we do not pay for TX currently so has to be manually updated
        // tx.message.header.num_required_signatures = 1;
        // tx.message.header.num_readonly_signed_accounts = 0;
        let tx = solana_sdk::transaction::Transaction::new_with_payer(
            instructions,
            Some(&self.payer().pubkey()),
        );

        self.process_transaction_with_settle(tx)
    }

    fn get_sysvar<T: Sysvar>(&self) -> T {
        trident_svm::trident_svm::TridentSVM::get_sysvar::<T>(self)
    }

    #[doc(hidden)]
    fn _clear_accounts(&mut self) {
        self.clear_accounts();
    }
}
