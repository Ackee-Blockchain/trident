use borsh::BorshDeserialize;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::TransactionError;
use trident_svm::prelude::TridentTransactionProcessingResult;
use trident_svm::processor::InstructionError;

use crate::trident::transaction_result::TransactionResult;
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

impl Trident {
    /// Processes a transaction containing one or more instructions
    ///
    /// This method executes the provided instructions as a single transaction and returns
    /// the result including success/failure status and transaction logs. It also handles
    /// fuzzing metrics collection when enabled via environment variables.
    ///
    /// # Arguments
    /// * `instructions` - A slice of instructions to execute in the transaction
    /// * `transaction_name` - A descriptive name for the transaction (used in metrics)
    ///
    /// # Returns
    /// A `TransactionResult` containing the execution result and logs
    ///
    /// # Example
    /// ```rust,ignore
    /// let instructions = vec![system_instruction::transfer(&from, &to, 1000)];
    /// let result = trident.process_transaction(&instructions, Some("Transfer SOL"));
    /// assert!(result.is_success());
    /// ```
    pub fn process_transaction(
        &mut self,
        instructions: &[Instruction],
        log_as: Option<&str>,
    ) -> TransactionResult {
        let fuzzing_metrics = std::env::var("FUZZING_METRICS");
        let fuzzing_debug = std::env::var("TRIDENT_FUZZ_DEBUG");

        if fuzzing_metrics.is_ok() && log_as.is_some() {
            if let Some(log_as) = log_as {
                self.fuzzing_data.add_executed_transaction(log_as);
            }
        }
        if fuzzing_debug.is_ok() {
            let tx = format!("{:#?}", instructions);
            trident_svm::prelude::trident_svm_log::log_message(
                &tx,
                trident_svm::prelude::Level::Debug,
            );
        }
        let processing_data = self.process_instructions(instructions);

        self.handle_tx_result(&processing_data, log_as, instructions)
    }

    /// Deploys an entrypoint program to the SVM runtime
    ///
    /// This method is only available when the "syscall-v2" feature is enabled.
    /// It deploys a program that serves as an entrypoint for other programs.
    ///
    /// # Arguments
    /// * `_program` - The entrypoint program to deploy
    #[cfg(feature = "syscall-v2")]
    pub fn deploy_entrypoint(&mut self, _program: TridentEntrypoint) {
        self.client.deploy_entrypoint_program(&_program);
    }

    /// Deploys a binary program to the SVM runtime
    ///
    /// This method deploys a compiled Solana program (BPF/SBF) to the runtime,
    /// making it available for instruction execution.
    ///
    /// # Arguments
    /// * `program` - The compiled program to deploy
    pub fn deploy_program(&mut self, program: TridentProgram) {
        self.client.deploy_binary_program(&program);
    }

    /// Warps the blockchain clock to a specific epoch
    ///
    /// This method updates the system clock sysvar to simulate time progression
    /// to the specified epoch, useful for testing time-dependent program logic.
    ///
    /// # Arguments
    /// * `warp_epoch` - The target epoch to warp to
    pub fn warp_to_epoch(&mut self, warp_epoch: u64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.epoch = warp_epoch;
        self.client.set_sysvar(&clock);
    }

    /// Warps the blockchain clock to a specific slot
    ///
    /// This method updates the system clock sysvar to simulate progression
    /// to the specified slot number.
    ///
    /// # Arguments
    /// * `warp_slot` - The target slot number to warp to
    pub fn warp_to_slot(&mut self, warp_slot: u64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.slot = warp_slot;
        self.client.set_sysvar(&clock);
    }
    /// Warps the blockchain clock to a specific Unix timestamp
    ///
    /// This method updates the system clock sysvar to simulate time progression
    /// to the specified Unix timestamp.
    ///
    /// # Arguments
    /// * `warp_timestamp` - The target Unix timestamp to warp to
    pub fn warp_to_timestamp(&mut self, warp_timestamp: i64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.unix_timestamp = warp_timestamp;
        self.client.set_sysvar(&clock);
    }

    /// Advances the blockchain clock by a specified number of seconds
    ///
    /// This method increments the current Unix timestamp by the given number
    /// of seconds, useful for testing time-based program behavior.
    ///
    /// # Arguments
    /// * `seconds` - The number of seconds to advance the clock
    pub fn forward_in_time(&mut self, seconds: i64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.unix_timestamp = clock.unix_timestamp.saturating_add(seconds);
        self.client.set_sysvar(&clock);
    }

    /// Sets a custom account state at the specified address
    ///
    /// This method allows you to manually set account data, lamports, and owner
    /// for any public key, useful for setting up test scenarios.
    ///
    /// # Arguments
    /// * `address` - The public key where the account should be stored
    /// * `account` - The account data to set
    pub fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.client.set_account(address, account, false);
    }

    /// Returns the default payer keypair for transactions
    ///
    /// This keypair is used to pay transaction fees and sign transactions
    /// when no other payer is specified.
    ///
    /// # Returns
    /// The default payer keypair
    pub fn payer(&self) -> solana_sdk::signature::Keypair {
        self.client.get_payer()
    }

    /// Retrieves account data for the specified public key
    ///
    /// Returns the account data including lamports, owner, and data bytes.
    /// If the account doesn't exist, returns a default empty account.
    ///
    /// # Arguments
    /// * `key` - The public key of the account to retrieve
    ///
    /// # Returns
    /// The account data or a default account if not found
    pub fn get_account(&mut self, key: &Pubkey) -> AccountSharedData {
        trident_svm::trident_svm::TridentSVM::get_account(&self.client, key).unwrap_or_default()
    }
    /// Retrieves and deserializes account data as a specific type
    ///
    /// This method fetches account data and attempts to deserialize it using Borsh,
    /// skipping the specified discriminator bytes at the beginning.
    ///
    /// # Arguments
    /// * `key` - The public key of the account to retrieve
    /// * `discriminator_size` - Number of bytes to skip before deserializing
    ///
    /// # Returns
    /// Some(T) if deserialization succeeds, None otherwise
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

    /// Gets the current Unix timestamp from the blockchain clock
    ///
    /// Returns the current timestamp as stored in the Clock sysvar.
    ///
    /// # Returns
    /// The current Unix timestamp in seconds
    pub fn get_current_timestamp(&self) -> i64 {
        self.get_sysvar::<Clock>().unix_timestamp
    }

    /// Gets the last blockhash (not implemented for TridentSVM)
    ///
    /// # Panics
    /// This method always panics as it's not yet implemented for TridentSVM
    pub fn get_last_blockhash(&self) -> Hash {
        panic!("Not yet implemented for TridentSVM");
    }

    fn process_instructions(
        &mut self,
        instructions: &[Instruction],
    ) -> TridentTransactionProcessingResult {
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

    /// Retrieves a system variable (sysvar) of the specified type
    ///
    /// System variables contain blockchain state information like clock,
    /// rent, and epoch schedule data.
    ///
    /// # Returns
    /// The requested sysvar data
    pub fn get_sysvar<T: Sysvar>(&self) -> T {
        trident_svm::trident_svm::TridentSVM::get_sysvar::<T>(&self.client)
    }

    /// Airdrops SOL to the specified address
    ///
    /// This method adds the specified amount of lamports to the target account.
    /// If the account doesn't exist, it will be created with the airdropped amount.
    ///
    /// # Arguments
    /// * `address` - The public key to receive the airdrop
    /// * `amount` - The number of lamports to airdrop
    pub fn airdrop(&mut self, address: &Pubkey, amount: u64) {
        let mut account = self.get_account(address);

        account.set_lamports(account.lamports() + amount);
        self.set_account_custom(address, &account);
    }

    /// Derives the program data address for an upgradeable program
    ///
    /// This method finds the program data account address for an upgradeable BPF loader program
    /// by deriving a Program Derived Address (PDA) using the program's address as a seed.
    ///
    /// # Arguments
    /// * `program_address` - The public key of the upgradeable program
    ///
    /// # Returns
    /// The derived program data address (PDA)
    pub fn get_program_data_address_v3(&self, program_address: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[program_address.as_ref()],
            &solana_sdk::bpf_loader_upgradeable::ID,
        )
        .0
    }

    /// Creates a program address (PDA) from seeds and a program ID
    ///
    /// This method attempts to create a valid program-derived address using the provided
    /// seeds and program ID. Unlike `find_program_address`, this does not search for a
    /// valid bump seed and will return None if the provided seeds don't produce a valid PDA.
    ///
    /// # Arguments
    /// * `seeds` - Array of seed byte slices used to derive the address
    /// * `program_id` - The program ID to use for derivation
    ///
    /// # Returns
    /// Some(Pubkey) if the seeds produce a valid PDA, None otherwise
    ///
    /// # Example
    /// ```rust,ignore
    /// let seeds = &[b"my-seed", &[bump_seed]];
    /// if let Some(pda) = trident.create_program_address(seeds, &program_id) {
    ///     println!("Created PDA: {}", pda);
    /// }
    /// ```
    pub fn create_program_address(&self, seeds: &[&[u8]], program_id: &Pubkey) -> Option<Pubkey> {
        Pubkey::create_program_address(seeds, program_id).ok()
    }

    /// Finds a valid program address (PDA) and its bump seed
    ///
    /// This method searches for a valid program-derived address by trying different bump
    /// seeds (starting from 255 and counting down) until a valid PDA is found. This is the
    /// canonical way to derive PDAs in Solana programs.
    ///
    /// # Arguments
    /// * `seeds` - Array of seed byte slices used to derive the address
    /// * `program_id` - The program ID to use for derivation
    ///
    /// # Returns
    /// A tuple containing the derived PDA and the bump seed used to generate it
    ///
    /// # Example
    /// ```rust,ignore
    /// let seeds = &[b"my-seed", user_pubkey.as_ref()];
    /// let (pda, bump) = trident.find_program_address(seeds, &program_id);
    /// println!("Found PDA: {} with bump: {}", pda, bump);
    /// ```
    pub fn find_program_address(&self, seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(seeds, program_id)
    }

    fn handle_tx_result(
        &mut self,
        tx_processing_result: &TridentTransactionProcessingResult,
        log_as: Option<&str>,
        instructions: &[Instruction],
    ) -> TransactionResult {
        let fuzzing_metrics = std::env::var("FUZZING_METRICS");
        let fuzzing_debug = std::env::var("TRIDENT_FUZZ_DEBUG");

        // NOTE: for now we just expect that one transaction was executed
        let tx_result = &tx_processing_result.get_result().processing_results[0];

        let transaction_timestamp = tx_processing_result.get_transaction_timestamp();

        match tx_result {
            Ok(result) => match result {
                trident_svm::prelude::solana_svm::transaction_processing_result::ProcessedTransaction::Executed(executed_transaction) => match &executed_transaction.execution_details.status {
                    Ok(_) => {
                        // Record successful execution
                        if fuzzing_metrics.is_ok() && log_as.is_some() {
                            if let Some(log_as) = log_as {
                                self.fuzzing_data
                                    .add_successful_transaction(log_as);
                            }
                        }
                        TransactionResult::new(Ok(()), executed_transaction.execution_details.log_messages.clone().unwrap_or_default(), transaction_timestamp)
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
                                        if log_as.is_some() {
                                            let rng = self.rng.get_seed();
                                            // TODO format instructions
                                            let tx = format!("{:#?}", instructions);
                                            self.fuzzing_data.add_transaction_panicked(
                                                log_as.unwrap(),
                                                rng,
                                                instruction_error.to_string(),
                                                executed_transaction.execution_details.log_messages.clone(),
                                                tx,
                                            );
                                        }
                                    }
                                }
                                InstructionError::Custom(error_code) => {
                                    if fuzzing_metrics.is_ok() && log_as.is_some() {
                                        if let Some(log_as) = log_as {
                                            self.fuzzing_data.add_custom_instruction_error(
                                                log_as,
                                                error_code,
                                                executed_transaction.execution_details.log_messages.clone(),
                                            );
                                        }
                                    }
                                }
                                _ => {
                                    if fuzzing_metrics.is_ok() && log_as.is_some() {
                                        if let Some(log_as) = log_as {
                                            self.fuzzing_data.add_failed_transaction(
                                                log_as,
                                                    instruction_error.to_string(),
                                                    executed_transaction.execution_details.log_messages.clone(),
                                                );
                                        }
                                    }
                                }
                            }
                        } else if fuzzing_metrics.is_ok() && log_as.is_some() {
                            if let Some(log_as) = log_as {
                            self.fuzzing_data.add_failed_transaction(
                                log_as,
                                    transaction_error.to_string(),
                                    executed_transaction.execution_details.log_messages.clone(),
                                );
                            }
                        }
                        TransactionResult::new(Err(transaction_error.clone()), executed_transaction.execution_details.log_messages.clone().unwrap_or_default(), transaction_timestamp)
                    },
                },
                trident_svm::prelude::solana_svm::transaction_processing_result::ProcessedTransaction::FeesOnly(_) => todo!(),
            },
            Err(transaction_error) => TransactionResult::new(Err(transaction_error.clone()), vec![], transaction_timestamp),
        }
    }
}
