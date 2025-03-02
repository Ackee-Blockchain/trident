use super::transaction_private::TransactionPrivateMethods;
use super::TransactionGetters;
use super::TransactionHooks;
use super::TransactionSetters;
use crate::error::*;
use crate::fuzzing::FuzzingStatistics;
use crate::traits::FuzzClient;
use crate::types::FuzzerData;

use solana_sdk::transaction::TransactionError;

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
        fuzzer_data: &mut FuzzerData,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) -> arbitrary::Result<Self>
    where
        Self: std::marker::Sized + for<'a> arbitrary::Arbitrary<'a>;

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
    fn execute(&mut self, client: &mut impl FuzzClient) -> Result<(), FuzzingError> {
        let instructions = self.create_transaction(client);

        let fuzzing_metrics = std::env::var("FUZZING_METRICS");

        // If stats are enabled, use the stats logger
        if fuzzing_metrics.is_ok() {
            let mut stats_logger = FuzzingStatistics::new();

            // Record transaction invocation
            stats_logger.increase_invoked(self.get_transaction_name());

            // Run pre-transaction hook
            self.pre_transaction(client);

            // Take snapshot of accounts before execution
            self.set_snapshot_before(client);

            // Execute the transaction
            let tx_result = client.process_instructions(&instructions);

            match tx_result {
                Ok(_) => {
                    // Record successful execution
                    stats_logger.increase_successful(self.get_transaction_name());

                    // Take snapshot of accounts after execution
                    self.set_snapshot_after(client);

                    // Run invariant checks
                    if let Err(e) = self.transaction_invariant_check() {
                        // Record check failure
                        stats_logger.increase_failed_check(self.get_transaction_name());
                        stats_logger.output_serialized();

                        eprintln!(
                            "\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} transaction did not pass!",
                            self.get_transaction_name()
                        );
                        panic!("{}", e)
                    }

                    // Output statistics
                    stats_logger.output_serialized();

                    // Run post-transaction hook
                    self.post_transaction(client);
                }
                Err(e) => {
                    // Record transaction failure
                    stats_logger.increase_failed(self.get_transaction_name());
                    stats_logger.output_serialized();

                    // Handle transaction error
                    self.transaction_error_handler(e)?
                }
            }
        } else {
            // Take snapshot of accounts before execution
            self.set_snapshot_before(client);

            // Run pre-transaction hook
            self.pre_transaction(client);

            // Execute the transaction
            let tx_result = client.process_instructions(&instructions);

            match tx_result {
                Ok(_) => {
                    // Take snapshot of accounts after execution
                    self.set_snapshot_after(client);

                    // Run invariant checks
                    if let Err(e) = self.transaction_invariant_check() {
                        eprintln!(
                            "\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} transaction did not pass!",
                            self.get_transaction_name()
                        );
                        panic!("{}", e)
                    }

                    // Run post-transaction hook
                    self.post_transaction(client);
                }
                Err(e) => {
                    // Handle transaction error
                    self.transaction_error_handler(e)?
                }
            }
        }

        Ok(())
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
    fn execute_no_hooks(&mut self, client: &mut impl FuzzClient) -> Result<(), TransactionError> {
        let instructions = self.create_transaction(client);

        let fuzzing_metrics = std::env::var("FUZZING_METRICS");

        // If stats are enabled, use the stats logger
        if fuzzing_metrics.is_ok() {
            let mut stats_logger = FuzzingStatistics::new();

            // Record transaction invocation
            stats_logger.increase_invoked(self.get_transaction_name());

            // Take snapshot of accounts before execution
            self.set_snapshot_before(client);

            // Execute the transaction
            let tx_result = client.process_instructions(&instructions);

            match tx_result {
                Ok(_) => {
                    // Record successful execution
                    stats_logger.increase_successful(self.get_transaction_name());

                    // Output statistics
                    stats_logger.output_serialized();

                    // Take snapshot of accounts after execution
                    self.set_snapshot_after(client);
                }
                Err(_e) => {
                    // Record transaction failure
                    stats_logger.increase_failed(self.get_transaction_name());
                    stats_logger.output_serialized();
                }
            }
        } else {
            // Take snapshot of accounts before execution
            self.set_snapshot_before(client);

            // Execute the transaction
            let tx_result = client.process_instructions(&instructions);

            match tx_result {
                Ok(_) => {
                    // Take snapshot of accounts after execution
                    self.set_snapshot_after(client);
                }
                Err(_e) => {
                    // No error handling in no_hooks mode
                }
            }
        }

        Ok(())
    }
}
