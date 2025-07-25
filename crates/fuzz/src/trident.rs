use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;
use solana_sdk::account::ReadableAccount;
use solana_sdk::account::WritableAccount;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::TransactionError;
use trident_fuzz_metrics::FuzzingStatistics;
use trident_svm::prelude::solana_svm::transaction_results::TransactionExecutionResult;
use trident_svm::processor::InstructionError;
use trident_svm::trident_svm::TridentSVM;

use crate::fuzzing::TridentRng;
use crate::traits::FuzzClient;
use crate::traits::TransactionGetters;
use crate::traits::TransactionHooks;
use crate::traits::TransactionPrivateMethods;
use crate::traits::TransactionSetters;

use trident_fuzz_metrics::types::Seed;

pub struct Trident {
    client: TridentSVM,
    metrics: FuzzingStatistics,
    rng: TridentRng,
}

impl Trident {
    pub fn new_with_random_seed() -> Self {
        Self {
            client: TridentSVM::new_client(),
            metrics: FuzzingStatistics::default(),
            rng: TridentRng::random(),
        }
    }

    pub fn new_with_seed(seed: Seed) -> Self {
        Self {
            client: TridentSVM::new_client(),
            metrics: FuzzingStatistics::default(),
            rng: TridentRng::new(seed),
        }
    }

    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.rng.gen_range(range)
    }

    pub fn airdrop(&mut self, address: &Pubkey, amount: u64) {
        let mut account = self.client.get_account(address).unwrap_or_default();

        account.set_lamports(account.lamports() + amount);
        self.client.set_account_custom(address, &account);
    }

    pub fn gen_string(&mut self, length: usize) -> String {
        self.rng.gen_string(length)
    }

    pub fn get_client(&mut self) -> &mut impl FuzzClient {
        &mut self.client
    }

    pub fn execute_transaction<T>(
        &mut self,
        transaction: &mut T,
        transaction_name_override: Option<&str>,
    ) where
        T: TransactionHooks
            + TransactionGetters
            + TransactionSetters
            + TransactionPrivateMethods
            + std::fmt::Debug,
    {
        let transaction_name = if let Some(transaction_name_override) = transaction_name_override {
            transaction_name_override.to_string()
        } else {
            transaction.get_transaction_name()
        };

        let instructions = transaction.create_transaction(self.get_client());

        let fuzzing_metrics = std::env::var("FUZZING_METRICS");
        let fuzzing_debug = std::env::var("TRIDENT_FUZZ_DEBUG");

        // If stats are enabled, use the stats logger

        // Execute the transaction
        if fuzzing_metrics.is_ok() {
            self.metrics.add_executed_transaction(&transaction_name);
        }
        if fuzzing_debug.is_ok() {
            let tx = format!("{:#?}", transaction);
            trident_svm::prelude::trident_svm_log::log_message(
                &tx,
                trident_svm::prelude::Level::Debug,
            );
        }
        // 1. Take snapshot of accounts before execution
        transaction.set_snapshot_before(self.get_client());

        // 2. Run pre-transaction hook
        transaction.pre_transaction(self.get_client());

        // 3. Process instructions
        let processing_data = self.get_client()._process_instructions(&instructions);

        // 4. Take snapshot of accounts after execution
        transaction.set_snapshot_after(self.get_client());

        // 5. Run post-transaction hook
        transaction.post_transaction(self.get_client());

        // NOTE: for now we just expect that one transaction was executed
        let tx_result = &processing_data.execution_results[0];

        match tx_result {
            TransactionExecutionResult::Executed {
                details,
                programs_modified_by_tx: _,
            } => match &details.status {
                Ok(_) => {
                    // Record successful execution
                    if fuzzing_metrics.is_ok() {
                        self.metrics.add_successful_transaction(&transaction_name);
                    }

                    // Run invariant checks
                    if let Err(invariant_error) = transaction.transaction_invariant_check() {
                        if fuzzing_debug.is_ok() {
                            trident_svm::prelude::trident_svm_log::log_message(
                                &invariant_error.to_string(),
                                trident_svm::prelude::Level::Error,
                            );
                        }

                        // Record check failure
                        if fuzzing_metrics.is_ok() {
                            let rng = self.rng.get_seed();

                            self.metrics.add_failed_invariant(
                                &transaction_name,
                                &rng,
                                invariant_error.to_string(),
                            );
                        }
                    }
                }
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
                                    self.metrics.add_transaction_panicked(
                                        &transaction_name,
                                        rng,
                                        instruction_error.to_string(),
                                        details.log_messages.clone(),
                                    );
                                }
                            }
                            InstructionError::Custom(error_code) => {
                                if fuzzing_metrics.is_ok() {
                                    self.metrics.add_custom_instruction_error(
                                        &transaction_name,
                                        error_code,
                                        details.log_messages.clone(),
                                    );
                                }
                            }
                            _ => {
                                if fuzzing_metrics.is_ok() {
                                    self.metrics.add_failed_transaction(
                                        &transaction_name,
                                        instruction_error.to_string(),
                                        details.log_messages.clone(),
                                    );
                                }
                            }
                        }
                    } else if fuzzing_metrics.is_ok() {
                        self.metrics.add_failed_transaction(
                            &transaction_name,
                            transaction_error.to_string(),
                            details.log_messages.clone(),
                        );
                    }
                    // Handle transaction error
                    transaction.transaction_error_handler(transaction_error.clone());
                }
            },
            TransactionExecutionResult::NotExecuted(_transaction_error) => {
                // Transaction was not executed, just do nothing and return
            }
        }
    }

    pub fn override_seed(&mut self, seed: Seed) {
        self.rng.override_seed(seed);
    }

    #[doc(hidden)]
    pub fn _next_iteration(&mut self) {
        self.client._clear_accounts();
        self.rng.rotate_seed();
    }

    #[doc(hidden)]
    pub fn _set_thread_id(&mut self, thread_id: usize) {
        self.rng.set_thread_id(thread_id);
    }

    #[doc(hidden)]
    pub fn _get_metrics(self) -> FuzzingStatistics {
        self.metrics
    }
}
