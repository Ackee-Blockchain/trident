use borsh::BorshDeserialize;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::TransactionError;
use trident_svm::prelude::solana_svm::transaction_processing_result::TransactionProcessingResult;
use trident_svm::processor::InstructionError;

use crate::trident::Trident;

use solana_sdk::account::AccountSharedData;
use solana_sdk::account::ReadableAccount;
use solana_sdk::account::WritableAccount;
use solana_sdk::clock::Clock;
use solana_sdk::hash::Hash;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar::Sysvar;

#[cfg(feature = "syscall-v2")]
use trident_svm::types::trident_entrypoint::TridentEntrypoint;
use trident_svm::types::trident_program::TridentProgram;

pub struct TransactionResult {
    transaction_result: solana_sdk::transaction::Result<()>,
    transaction_logs: Vec<String>,
}

impl TransactionResult {
    fn new(
        transaction_result: solana_sdk::transaction::Result<()>,
        transaction_logs: Vec<String>,
    ) -> Self {
        Self {
            transaction_result,
            transaction_logs,
        }
    }

    pub fn is_success(&self) -> bool {
        self.transaction_result.is_ok()
    }

    pub fn is_failure(&self) -> bool {
        self.transaction_result.is_err()
    }

    pub fn logs(&self) -> &[String] {
        &self.transaction_logs
    }
    pub fn get_result(&self) -> &solana_sdk::transaction::Result<()> {
        &self.transaction_result
    }
}

impl Trident {
    pub fn process_transaction(
        &mut self,
        instructions: &[Instruction],
        transaction_name: &str,
    ) -> TransactionResult {
        let fuzzing_metrics = std::env::var("FUZZING_METRICS");
        let fuzzing_debug = std::env::var("TRIDENT_FUZZ_DEBUG");

        if fuzzing_metrics.is_ok() {
            self.fuzzing_data.add_executed_transaction(transaction_name);
        }
        if fuzzing_debug.is_ok() {
            let tx = format!("{:#?}", instructions);
            trident_svm::prelude::trident_svm_log::log_message(
                &tx,
                trident_svm::prelude::Level::Debug,
            );
        }
        let processing_data = self.process_instructions(instructions);

        // NOTE: for now we just expect that one transaction was executed
        let tx_result = &processing_data.processing_results[0];

        self.handle_tx_result(tx_result, transaction_name, instructions)
    }

    #[cfg(feature = "syscall-v2")]
    pub fn deploy_entrypoint(&mut self, _program: TridentEntrypoint) {
        self.client.deploy_entrypoint_program(&_program);
    }

    pub fn deploy_program(&mut self, program: TridentProgram) {
        self.client.deploy_binary_program(&program);
    }

    pub fn warp_to_epoch(&mut self, warp_epoch: u64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.epoch = warp_epoch;
        self.client.set_sysvar(&clock);
    }

    pub fn warp_to_slot(&mut self, warp_slot: u64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.slot = warp_slot;
        self.client.set_sysvar(&clock);
    }
    pub fn warp_to_timestamp(&mut self, warp_timestamp: i64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.unix_timestamp = warp_timestamp;
        self.client.set_sysvar(&clock);
    }

    pub fn forward_in_time(&mut self, seconds: i64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.unix_timestamp = clock.unix_timestamp.saturating_add(seconds);
        self.client.set_sysvar(&clock);
    }

    pub fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.client.set_account(address, account, false);
    }

    pub fn payer(&self) -> solana_sdk::signature::Keypair {
        self.client.get_payer()
    }

    pub fn get_account(&mut self, key: &Pubkey) -> AccountSharedData {
        trident_svm::trident_svm::TridentSVM::get_account(&self.client, key).unwrap_or_default()
    }
    pub fn get_account_with_type<T: BorshDeserialize>(
        &mut self,
        key: &Pubkey,
        discriminator_size: usize,
    ) -> Option<T> {
        let account = self.get_account(key);
        let data = account.data();

        if data.len() > discriminator_size {
            T::deserialize(&mut &data[discriminator_size..]).ok()
        } else {
            None
        }
    }

    pub fn get_current_timestamp(&self) -> i64 {
        self.get_sysvar::<Clock>().unix_timestamp
    }

    pub fn get_last_blockhash(&self) -> Hash {
        panic!("Not yet implemented for TridentSVM");
    }

    fn process_instructions(
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

        self.client.process_transaction_with_settle(tx)
    }

    pub fn get_sysvar<T: Sysvar>(&self) -> T {
        trident_svm::trident_svm::TridentSVM::get_sysvar::<T>(&self.client)
    }

    pub fn airdrop(&mut self, address: &Pubkey, amount: u64) {
        let mut account = self.get_account(address);

        account.set_lamports(account.lamports() + amount);
        self.set_account_custom(address, &account);
    }

    fn handle_tx_result(
        &mut self,
        tx_result: &TransactionProcessingResult,
        transaction_name: &str,
        instructions: &[Instruction],
    ) -> TransactionResult {
        let fuzzing_metrics = std::env::var("FUZZING_METRICS");
        let fuzzing_debug = std::env::var("TRIDENT_FUZZ_DEBUG");

        match tx_result {
            Ok(result) => match result {
                trident_svm::prelude::solana_svm::transaction_processing_result::ProcessedTransaction::Executed(executed_transaction) => match &executed_transaction.execution_details.status {
                    Ok(_) => {
                        // Record successful execution
                        if fuzzing_metrics.is_ok() {
                            self.fuzzing_data
                                .add_successful_transaction(transaction_name);
                        }
                        TransactionResult::new(Ok(()), executed_transaction.execution_details.log_messages.clone().unwrap_or_default())
                    },
                    Err(transaction_error) => {
                        if let TransactionError::InstructionError(_error_code, instruction_error) =
                            &transaction_error
                        {
                            match instruction_error {
                                InstructionError::ProgramFailedToComplete => {
                                    if fuzzing_metrics.is_ok() {
                                        if fuzzing_debug.is_ok() {
                                            trident_svm::prelude::trident_svm_log::log_message(
                                                "TRANSACTION PANICKED",
                                                trident_svm::prelude::Level::Error,
                                            );
                                        }
                                        let rng = self.rng.get_seed();
                                        // TODO format instructions
                                        let tx = format!("{:#?}", instructions);
                                        self.fuzzing_data.add_transaction_panicked(
                                            transaction_name,
                                            rng,
                                            instruction_error.to_string(),
                                            executed_transaction.execution_details.log_messages.clone(),
                                            tx,
                                        );
                                    }
                                }
                                InstructionError::Custom(error_code) => {
                                    if fuzzing_metrics.is_ok() {
                                        self.fuzzing_data.add_custom_instruction_error(
                                            transaction_name,
                                            error_code,
                                            executed_transaction.execution_details.log_messages.clone(),
                                        );
                                    }
                                }
                                _ => {
                                    if fuzzing_metrics.is_ok() {
                                        self.fuzzing_data.add_failed_transaction(
                                            transaction_name,
                                            instruction_error.to_string(),
                                            executed_transaction.execution_details.log_messages.clone(),
                                        );
                                    }
                                }
                            }
                        } else if fuzzing_metrics.is_ok() {
                            self.fuzzing_data.add_failed_transaction(
                                transaction_name,
                                transaction_error.to_string(),
                                executed_transaction.execution_details.log_messages.clone(),
                            );
                        }
                        TransactionResult::new(Err(transaction_error.clone()), executed_transaction.execution_details.log_messages.clone().unwrap_or_default())
                    },
                },
                trident_svm::prelude::solana_svm::transaction_processing_result::ProcessedTransaction::FeesOnly(_) => todo!(),
            },
            Err(transaction_error) => TransactionResult::new(Err(transaction_error.clone()), vec![]),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn create_account(
        &mut self,
        address: &Pubkey,
        from: &Pubkey,
        space: usize,
        owner: &Pubkey,
    ) -> Vec<Instruction> {
        let account = self.client.get_account(address).unwrap_or_default();
        let rent = solana_sdk::rent::Rent::default();
        if account.lamports() > 0 {
            let mut instructions = vec![];
            let lamports_required = rent.minimum_balance(space);

            let remaining_lamports = lamports_required.saturating_sub(account.lamports());

            if remaining_lamports > 0 {
                let transfer =
                    solana_sdk::system_instruction::transfer(from, address, remaining_lamports);
                instructions.push(transfer);
            }

            let allocate = solana_sdk::system_instruction::allocate(address, space as u64);
            instructions.push(allocate);

            let assign = solana_sdk::system_instruction::assign(address, owner);
            instructions.push(assign);

            instructions
        } else {
            let ix = solana_sdk::system_instruction::create_account(
                from,
                address,
                rent.minimum_balance(space),
                space as u64,
                owner,
            );
            vec![ix]
        }
    }
    pub fn transfer(&mut self, from: &Pubkey, to: &Pubkey, amount: u64) -> Instruction {
        solana_sdk::system_instruction::transfer(from, to, amount)
    }
}
