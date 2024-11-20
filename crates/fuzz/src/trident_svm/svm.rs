use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use solana_bpf_loader_program::syscalls::create_program_runtime_environment_v1;
use solana_bpf_loader_program::syscalls::create_program_runtime_environment_v2;

use solana_compute_budget::compute_budget::ComputeBudget;
use solana_program_runtime::loaded_programs::{BlockRelation, ForkGraph, ProgramCacheEntry};
use solana_sdk::{
    account::WritableAccount,
    account::{AccountSharedData, ReadableAccount},
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
    clock::Clock,
    clock::Slot,
    feature_set::FeatureSet,
    fee::FeeStructure,
    hash::Hash,
    instruction::Instruction,
    native_loader,
    native_token::LAMPORTS_PER_SOL,
    pubkey,
    pubkey::Pubkey,
    rent::Rent,
    rent_collector::RentCollector,
    signature::Keypair,
    signer::Signer,
    sysvar::{Sysvar, SysvarId},
    transaction::SanitizedTransaction,
    transaction::TransactionError,
    transaction::{self},
};

use crate::{config::Config, fuzz_client::FuzzingProgram};

use crate::trident_svm::{accounts_db::AccountsDB, native::BUILTINS};

use solana_svm::{
    account_loader::CheckedTransactionDetails,
    transaction_processing_callback::TransactionProcessingCallback,
    transaction_processing_result::TransactionProcessingResultExtensions,
    transaction_processor::{
        ExecutionRecordingConfig, TransactionProcessingConfig, TransactionProcessingEnvironment,
    },
    transaction_processor::{LoadAndExecuteSanitizedTransactionsOutput, TransactionBatchProcessor},
};

use std::collections::HashSet;

use crate::trident_svm::log::setup_solana_logging;

use crate::fuzz_client::FuzzClient;

pub(crate) struct TridentForkGraph {}

impl ForkGraph for TridentForkGraph {
    fn relationship(&self, _a: Slot, _b: Slot) -> BlockRelation {
        BlockRelation::Unknown
    }
}

pub struct TridentSVM {
    accounts: AccountsDB,
    payer: Keypair,
    feature_set: Arc<FeatureSet>,
    processor: TransactionBatchProcessor<TridentForkGraph>,
    fork_graph: Arc<RwLock<TridentForkGraph>>,
}

impl FuzzClient for TridentSVM {
    fn warp_to_epoch(&mut self, warp_epoch: u64) {
        let mut clock = self.accounts.get_sysvar::<Clock>();

        clock.epoch = warp_epoch;
        self.set_sysvar(&clock);
    }

    fn warp_to_slot(&mut self, warp_slot: u64) {
        let mut clock = self.accounts.get_sysvar::<Clock>();

        clock.slot = warp_slot;
        self.set_sysvar(&clock);
    }

    fn forward_in_time(&mut self, seconds: i64) -> Result<(), crate::error::FuzzClientError> {
        let mut clock = self.accounts.get_sysvar::<Clock>();

        clock.unix_timestamp = clock.unix_timestamp.saturating_add(seconds);
        self.set_sysvar(&clock);
        Ok(())
    }

    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.accounts.add_account(address, account);
    }

    fn payer(&self) -> solana_sdk::signature::Keypair {
        self.payer.insecure_clone()
    }

    fn get_account(&mut self, key: &Pubkey) -> AccountSharedData {
        self.accounts.get_account(key).unwrap_or_default()
    }

    fn get_last_blockhash(&self) -> Hash {
        panic!("Not yet implemented for TridentSVM");
    }

    fn process_instruction(
        &mut self,
        instruction: Instruction,
    ) -> Result<(), crate::error::FuzzClientError> {
        setup_solana_logging();
        let tx = solana_sdk::transaction::Transaction::new_with_payer(
            &[instruction],
            Some(&self.payer().pubkey()),
        );

        // there should be at least 1 RW fee-payer account.
        // But we do not pay for TX currently so has to be manually updated
        // tx.message.header.num_required_signatures = 1;
        // tx.message.header.num_readonly_signed_accounts = 0;

        let sanitezed_tx =
            SanitizedTransaction::try_from_legacy_transaction(tx, &HashSet::new()).unwrap();
        // let result = self.process_transaction_svm(&[sanitezed_tx]);

        let compute_budget = ComputeBudget::default();
        let fee_structure = FeeStructure::default();
        let lamports_per_signature = fee_structure.lamports_per_signature;
        let rent_collector = RentCollector::default();

        let processing_environment = TransactionProcessingEnvironment {
            blockhash: Hash::default(),
            epoch_total_stake: None,
            epoch_vote_accounts: None,
            feature_set: self.feature_set.clone(),
            fee_structure: Some(&fee_structure),
            lamports_per_signature,
            rent_collector: Some(&rent_collector),
        };

        let processing_config = TransactionProcessingConfig {
            compute_budget: Some(compute_budget),
            log_messages_bytes_limit: Some(10 * 1000),
            recording_config: ExecutionRecordingConfig {
                enable_cpi_recording: true,
                enable_log_recording: true,
                enable_return_data_recording: true,
            },
            ..Default::default()
        };

        let result = self.processor.load_and_execute_sanitized_transactions(
            self,
            &[sanitezed_tx],
            get_transaction_check_results(1, lamports_per_signature),
            &processing_environment,
            &processing_config,
        );

        // settle results
        self.settle(&result);

        match &result.processing_results[0] {
            Ok(res) => match res {
                solana_svm::transaction_processing_result::ProcessedTransaction::Executed(
                    executed_transaction,
                ) => match &executed_transaction.execution_details.status {
                    Ok(_) => Ok(()),
                    Err(e) => match e {
                        TransactionError::InstructionError(_index, ix_error) => match ix_error {
                            solana_sdk::instruction::InstructionError::ProgramFailedToComplete => {
                                panic!("Program Failed to complete")
                            }
                            _ => Err(crate::error::FuzzClientError::Custom(3)),
                        },
                        _ => Err(crate::error::FuzzClientError::Custom(4)),
                    },
                },
                solana_svm::transaction_processing_result::ProcessedTransaction::FeesOnly(
                    _fees_only_transaction,
                ) => Err(crate::error::FuzzClientError::Custom(2)),
            },
            Err(e) => match e {
                TransactionError::InstructionError(_index, ix_error) => match ix_error {
                    solana_sdk::instruction::InstructionError::ProgramFailedToComplete => {
                        panic!("Program Failed to complete")
                    }
                    _ => Err(crate::error::FuzzClientError::Custom(3)),
                },
                _ => Err(crate::error::FuzzClientError::Custom(4)),
            },
        }
    }

    fn get_sysvar<T: Sysvar>(&mut self) -> T {
        self.accounts.get_sysvar::<T>()
    }

    fn clear_accounts(&mut self) {
        self.clear_accounts();
    }
}

impl TransactionProcessingCallback for TridentSVM {
    fn account_matches_owners(&self, account: &Pubkey, owners: &[Pubkey]) -> Option<usize> {
        self.get_account_shared_data(account)
            .and_then(|account| owners.iter().position(|key| account.owner().eq(key)))
    }

    fn get_account_shared_data(
        &self,
        pubkey: &Pubkey,
    ) -> Option<solana_sdk::account::AccountSharedData> {
        self.accounts.get_account(pubkey)
    }
}

impl Default for TridentSVM {
    fn default() -> Self {
        let payer = Keypair::new();

        let mut client = Self {
            accounts: Default::default(),
            payer: payer.insecure_clone(),
            feature_set: Arc::new(FeatureSet::all_enabled()),
            processor: TransactionBatchProcessor::<TridentForkGraph>::new_uninitialized(1, 1),
            fork_graph: Arc::new(RwLock::new(TridentForkGraph {})),
        };

        let payer_account = AccountSharedData::new(
            500_000_000 * LAMPORTS_PER_SOL,
            0,
            &solana_sdk::system_program::ID,
        );
        client.accounts.add_account(&payer.pubkey(), &payer_account);

        client
    }
}

impl TridentSVM {
    pub fn new(_program: &[FuzzingProgram], config: &Config) -> Self {
        TridentSVM::default()
            .with_processor()
            .with_sysvars()
            // .with_native_programs(program)
            .with_sbf_programs(config)
            .with_permanent_accounts(config)
            .with_builtins()
            .with_solana_program_library()
    }
    fn with_sysvars(mut self) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let clock = Clock {
            unix_timestamp: time as i64,
            ..Default::default()
        };
        self.set_sysvar(&clock);
        self.set_sysvar(&Rent::default());

        self.processor.fill_missing_sysvar_cache_entries(&self);

        self
    }

    fn with_processor(self) -> Self {
        {
            let compute_budget = ComputeBudget::default();

            let mut cache: std::sync::RwLockWriteGuard<
                '_,
                solana_program_runtime::loaded_programs::ProgramCache<TridentForkGraph>,
            > = self.processor.program_cache.write().unwrap();

            cache.fork_graph = Some(Arc::downgrade(&self.fork_graph));

            cache.environments.program_runtime_v1 = Arc::new(
                create_program_runtime_environment_v1(
                    &self.feature_set,
                    &compute_budget,
                    true,
                    true,
                )
                .unwrap(),
            );
            cache.environments.program_runtime_v2 =
                Arc::new(create_program_runtime_environment_v2(&compute_budget, true));
        }

        self
    }
    fn with_sbf_programs(mut self, config: &Config) -> Self {
        config.fuzz.programs.iter().for_each(|sbf_target| {
            self.add_program(&sbf_target.address, &sbf_target.data);
        });

        self
    }
    fn with_permanent_accounts(mut self, config: &Config) -> Self {
        config.fuzz.accounts.iter().for_each(|account| {
            self.accounts
                .add_permanent_account(&account.pubkey, &account.account);
        });

        self
    }
    fn add_program(&mut self, address: &Pubkey, data: &[u8]) {
        let rent = Rent::default();

        let program_account = address;

        let program_data_account =
            bpf_loader_upgradeable::get_program_data_address(program_account);

        let state = UpgradeableLoaderState::Program {
            programdata_address: program_data_account,
        };

        let buffer = bincode::serialize(&state).unwrap();
        let account_data = AccountSharedData::create(
            rent.minimum_balance(buffer.len()),
            buffer,
            bpf_loader_upgradeable::id(),
            true,
            Default::default(),
        );

        self.accounts.add_program(program_account, &account_data);

        let state = UpgradeableLoaderState::ProgramData {
            slot: 0,
            upgrade_authority_address: None,
        };
        let mut header = bincode::serialize(&state).unwrap();

        let mut complement = vec![
            0;
            std::cmp::max(
                0,
                UpgradeableLoaderState::size_of_programdata_metadata().saturating_sub(header.len())
            )
        ];

        let mut buffer: Vec<u8> = data.to_vec();
        header.append(&mut complement);
        header.append(&mut buffer);

        let account_data = AccountSharedData::create(
            rent.minimum_balance(header.len()),
            header,
            bpf_loader_upgradeable::id(),
            true,
            Default::default(),
        );

        self.accounts
            .add_program(&program_data_account, &account_data);
    }
    fn with_solana_program_library(mut self) -> Self {
        self.add_program(
            &pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            include_bytes!("solana-program-library/spl-token-mainet.so"),
        );
        self
    }
    fn set_sysvar<T>(&mut self, sysvar: &T)
    where
        T: Sysvar + SysvarId,
    {
        let account = AccountSharedData::new_data(1, &sysvar, &solana_sdk::sysvar::id()).unwrap();
        self.accounts.add_sysvar(&T::id(), &account);
    }

    #[allow(dead_code)]
    #[doc = "Executing programs as native is currently not supported"]
    #[doc = "thus programs can be included only as SBF binaries"]
    fn with_native_programs(mut self, native_programs: &[FuzzingProgram]) -> Self {
        native_programs.iter().for_each(|native| {
            let entry = match native.entry {
                Some(entry) => entry,
                None => panic!("Native programs have to have entry specified"),
            };

            self.accounts.add_program(
                &native.program_id,
                &native_loader::create_loadable_account_for_test(&native.program_name),
            );

            self.processor.add_builtin(
                &self,
                native.program_id,
                &native.program_name,
                ProgramCacheEntry::new_builtin(0, native.program_name.len(), entry),
            );
        });

        self
    }
    fn with_builtins(mut self) -> Self {
        BUILTINS.iter().for_each(|builtint| {
            self.accounts.add_program(
                &builtint.program_id,
                &native_loader::create_loadable_account_for_test(builtint.name),
            );

            self.processor.add_builtin(
                &self,
                builtint.program_id,
                builtint.name,
                ProgramCacheEntry::new_builtin(0, builtint.name.len(), builtint.entrypoint),
            );
        });

        self
    }
    fn settle(&mut self, output: &LoadAndExecuteSanitizedTransactionsOutput) {
        for x in &output.processing_results {
            if x.was_processed_with_successful_result() {
                let result = x.as_ref().unwrap();
                let executed_tx = result.executed_transaction().unwrap();

                for account in &executed_tx.loaded_transaction.accounts {
                    if !account.1.executable() && account.1.owner() != &solana_sdk::sysvar::id() {
                        self.accounts.add_account(&account.0, &account.1);
                    }
                }
            }
        }
    }
    pub fn clear_accounts(&mut self) {
        self.accounts.reset_temp();
        let payer_account = AccountSharedData::new(
            500_000_000 * LAMPORTS_PER_SOL,
            0,
            &solana_sdk::system_program::ID,
        );
        self.accounts
            .add_account(&self.payer.pubkey(), &payer_account);
    }
}

/// This function is also a mock. In the Agave validator, the bank pre-checks
/// transactions before providing them to the SVM API. We mock this step in
/// PayTube, since we don't need to perform such pre-checks.
pub(crate) fn get_transaction_check_results(
    len: usize,
    lamports_per_signature: u64,
) -> Vec<transaction::Result<CheckedTransactionDetails>> {
    vec![
        transaction::Result::Ok(CheckedTransactionDetails {
            nonce: None,
            lamports_per_signature,
        });
        len
    ]
}
