use super::transaction_private::TransactionPrivateMethods;
use super::TransactionGetters;
use super::TransactionHooks;
use super::TransactionSetters;
use crate::error::*;
use crate::fuzzing::FuzzingStatistics;
use crate::fuzzing::TridentRng;
use crate::traits::FuzzClient;

use solana_sdk::transaction::TransactionError;
use trident_svm::prelude::solana_svm::transaction_results::TransactionExecutionResult;
use trident_svm::processor::InstructionError;

#[allow(private_bounds)]
/// Trait providing methods to prepare data and accounts for transaction execution
pub trait TransactionMethods:
    TransactionHooks
    + TransactionGetters
    + TransactionSetters
    + TransactionPrivateMethods
    + std::fmt::Debug
{
    /// Builds a transaction instance from fuzzer data
    ///
    /// Creates a new transaction with the necessary data from the fuzzer
    /// and prepares any accounts needed for execution.
    fn build(
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
        rng: &mut TridentRng,
    ) -> Self;

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
    fn execute(
        &mut self,
        client: &mut impl FuzzClient,
        stats_logger: &mut FuzzingStatistics,
        rng: &TridentRng,
    ) -> Result<(), FuzzingError> {
        let instructions = self.create_transaction(client);

        let fuzzing_metrics = std::env::var("FUZZING_METRICS");

        // If stats are enabled, use the stats logger
        // Run pre-transaction hook
        self.pre_transaction(client);

        // Take snapshot of accounts before execution
        self.set_snapshot_before(client);

        // Execute the transaction
        if fuzzing_metrics.is_ok() {
            stats_logger.increase_invoked(self.get_transaction_name());
        }

        let processing_data = client._process_instructions(&instructions);

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
                        stats_logger.increase_successful(self.get_transaction_name());
                    }
                    // Take snapshot of accounts after execution
                    self.set_snapshot_after(client);

                    // Run invariant checks
                    if let Err(invariant_error) = self.transaction_invariant_check() {
                        // Record check failure
                        if fuzzing_metrics.is_ok() {
                            stats_logger.increase_failed_invariant(
                                self.get_transaction_name(),
                                rng.get_seed(),
                                invariant_error.to_string(),
                            );
                        }
                        return Err(invariant_error);
                    }

                    // Run post-transaction hook
                    self.post_transaction(client);
                    Ok(())
                }
                Err(transaction_error) => {
                    if let TransactionError::InstructionError(_error_code, instruction_error) =
                        &transaction_error
                    {
                        match instruction_error {
                            InstructionError::ProgramFailedToComplete => {
                                if fuzzing_metrics.is_ok() {
                                    stats_logger.increase_transaction_panicked(
                                        self.get_transaction_name(),
                                        rng.get_seed(),
                                        instruction_error.to_string(),
                                    );
                                }
                            }
                            _ => {
                                if fuzzing_metrics.is_ok() {
                                    stats_logger.increase_failed(
                                        self.get_transaction_name(),
                                        instruction_error.to_string(),
                                        details.log_messages.clone(),
                                    );
                                }
                            }
                        }
                    } else if fuzzing_metrics.is_ok() {
                        stats_logger.increase_failed(
                            self.get_transaction_name(),
                            transaction_error.to_string(),
                            details.log_messages.clone(),
                        );
                    }
                    // Handle transaction error
                    self.transaction_error_handler(transaction_error.clone())?;
                    Ok(())
                }
            },
            TransactionExecutionResult::NotExecuted(transaction_error) => {
                // Transaction was not executed, just do nothing and return
                Err(transaction_error.clone().into())
            }
        }
    }

    /// Executes the transaction without lifecycle hooks
    ///
    /// This is a simplified version that only:
    /// - Creates transaction instructions
    /// - Takes account snapshots before and after execution
    /// - Processes the transaction
    /// - Records statistics if enabled
    ///
    /// It does NOT run pre/post hooks or invariant checks.
    fn execute_no_hooks(
        &mut self,
        client: &mut impl FuzzClient,
        stats_logger: &mut FuzzingStatistics,
        rng: &TridentRng,
    ) -> Result<(), FuzzingError> {
        let instructions = self.create_transaction(client);

        let fuzzing_metrics = std::env::var("FUZZING_METRICS");

        // Take snapshot of accounts before execution
        self.set_snapshot_before(client);

        // Execute the transaction
        if fuzzing_metrics.is_ok() {
            stats_logger.increase_invoked(self.get_transaction_name());
        }

        let processing_data = client._process_instructions(&instructions);

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
                        stats_logger.increase_successful(self.get_transaction_name());
                    }
                    // Take snapshot of accounts after execution
                    self.set_snapshot_after(client);

                    // Run post-transaction hook
                    self.post_transaction(client);
                    Ok(())
                }
                Err(transaction_error) => {
                    if let TransactionError::InstructionError(_error_code, instruction_error) =
                        &transaction_error
                    {
                        match instruction_error {
                            InstructionError::ProgramFailedToComplete => {
                                if fuzzing_metrics.is_ok() {
                                    stats_logger.increase_transaction_panicked(
                                        self.get_transaction_name(),
                                        rng.get_seed(),
                                        instruction_error.to_string(),
                                    );
                                }
                            }
                            _ => {
                                if fuzzing_metrics.is_ok() {
                                    stats_logger.increase_failed(
                                        self.get_transaction_name(),
                                        instruction_error.to_string(),
                                        details.log_messages.clone(),
                                    );
                                }
                            }
                        }
                    } else if fuzzing_metrics.is_ok() {
                        stats_logger.increase_failed(
                            self.get_transaction_name(),
                            transaction_error.to_string(),
                            details.log_messages.clone(),
                        );
                    }
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
