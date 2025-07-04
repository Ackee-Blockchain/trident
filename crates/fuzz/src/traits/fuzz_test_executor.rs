use super::transaction_private::TransactionPrivateMethods;
use super::FuzzTestGetters;
use super::TransactionGetters;
use super::TransactionHooks;
use super::TransactionSetters;
use crate::error::*;
use crate::traits::FuzzClient;

use solana_sdk::transaction::TransactionError;
use trident_svm::prelude::solana_svm::transaction_results::TransactionExecutionResult;
use trident_svm::processor::InstructionError;

#[allow(private_bounds)]
/// Trait providing methods to prepare data and accounts for transaction execution
pub trait FuzzTestExecutor: FuzzTestGetters {
    /// Executes the transaction with full lifecycle hooks
    ///
    /// This method handles the complete transaction lifecycle:
    /// - Creates transaction instructions
    /// - Takes account snapshots before execution
    /// - Runs pre-transaction hooks
    /// - Processes the transaction
    /// - Takes account snapshots after execution
    /// - Performs invariant checks
    /// - Runs post-transaction hooks
    /// - Handles any errors
    fn execute_transaction<T>(
        &mut self,
        transaction: &mut T,
        transaction_name_override: Option<&str>,
    ) -> Result<(), FuzzingError>
    where
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

        // Take snapshot of accounts before execution
        transaction.set_snapshot_before(self.get_client());

        // If stats are enabled, use the stats logger
        // Run pre-transaction hook
        transaction.pre_transaction(self.get_client());

        // Execute the transaction
        if fuzzing_metrics.is_ok() {
            self.get_metrics().increase_invoked(&transaction_name);
        }
        if fuzzing_debug.is_ok() {
            let tx = format!("{:#?}", transaction);
            trident_svm::prelude::trident_svm_log::log_message(
                &tx,
                trident_svm::prelude::Level::Debug,
            );
        }

        let processing_data = self.get_client()._process_instructions(&instructions);

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
                        self.get_metrics().increase_successful(&transaction_name);
                    }
                    // Take snapshot of accounts after execution
                    transaction.set_snapshot_after(self.get_client());

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
                            let rng = self.get_rng().get_seed();

                            self.get_metrics().increase_failed_invariant(
                                &transaction_name,
                                rng,
                                invariant_error.to_string(),
                            );
                        }
                        return Err(invariant_error);
                    }

                    // Run post-transaction hook
                    transaction.post_transaction(self.get_client());
                    Ok(())
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
                                    let rng = self.get_rng().get_seed();
                                    self.get_metrics().increase_transaction_panicked(
                                        &transaction_name,
                                        rng,
                                        instruction_error.to_string(),
                                        details.log_messages.clone(),
                                    );
                                }
                            }
                            InstructionError::Custom(error_code) => {
                                if fuzzing_metrics.is_ok() {
                                    self.get_metrics().increase_custom_instruction_error(
                                        &transaction_name,
                                        error_code,
                                        details.log_messages.clone(),
                                    );
                                }
                            }
                            _ => {
                                if fuzzing_metrics.is_ok() {
                                    self.get_metrics().increase_failed(
                                        &transaction_name,
                                        instruction_error.to_string(),
                                        details.log_messages.clone(),
                                    );
                                }
                            }
                        }
                    } else if fuzzing_metrics.is_ok() {
                        self.get_metrics().increase_failed(
                            &transaction_name,
                            transaction_error.to_string(),
                            details.log_messages.clone(),
                        );
                    }
                    // Handle transaction error
                    transaction.transaction_error_handler(transaction_error.clone())?;
                    Ok(())
                }
            },
            TransactionExecutionResult::NotExecuted(transaction_error) => {
                // Transaction was not executed, just do nothing and return
                Err(transaction_error.clone().into())
            }
        }
    }
}
