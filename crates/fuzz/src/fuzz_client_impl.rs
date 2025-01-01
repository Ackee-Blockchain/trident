use solana_sdk::transaction::TransactionError;
use solana_sdk::{
    account::AccountSharedData, clock::Clock, hash::Hash, instruction::Instruction, pubkey::Pubkey,
    sysvar::Sysvar,
};

use crate::fuzz_client::FuzzClient;
use solana_sdk::signer::Signer;
use trident_config::Config;
use trident_svm::trident_svm::TridentSVM;
use trident_svm::utils::{ProgramEntrypoint, SBFTargets, TridentAccountSharedData};

impl FuzzClient for TridentSVM<'_> {
    fn new_client(programs: &[ProgramEntrypoint], config: &Config) -> Self {
        let sbf_programs =
            config
                .programs()
                .iter()
                .fold(Vec::new(), |mut sbf_programs, config_program| {
                    let target = SBFTargets::new(
                        config_program.address,
                        None, // TODO add authority to the config fuzzing program
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

        TridentSVM::new(programs, &sbf_programs, &permanent_accounts)
    }
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

    fn process_instructions(
        &mut self,
        instructions: &[Instruction],
    ) -> Result<(), crate::error::FuzzClientError> {
        // there should be at least 1 RW fee-payer account.
        // But we do not pay for TX currently so has to be manually updated
        // tx.message.header.num_required_signatures = 1;
        // tx.message.header.num_readonly_signed_accounts = 0;
        let tx = solana_sdk::transaction::Transaction::new_with_payer(
            instructions,
            Some(&self.payer().pubkey()),
        );

        let result = self.process_transaction(tx);

        // eprintln!("result: {:#?}", result.execution_results);
        // eprintln!("result: {:#?}", result.loaded_transactions);

        self.settle(&result);

        match &result.execution_results[0] {
            solana_svm::transaction_results::TransactionExecutionResult::Executed {
                details,
                programs_modified_by_tx: _,
            } => match &details.status {
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
            solana_svm::transaction_results::TransactionExecutionResult::NotExecuted(
                transaction_error,
            ) => match transaction_error {
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
