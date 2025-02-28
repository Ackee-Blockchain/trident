use super::transaction_private::TransactionPrivateMethods;
use super::TransactionCustomMethods;
use super::TransactionGetters;
use super::TransactionSetters;
use crate::error::*;
use crate::fuzzing::FuzzingStatistics;
use crate::traits::FuzzClient;

use trident_config::TridentConfig;

#[allow(private_bounds)]
/// Trait providing methods to prepare data and accounts for transaction
pub trait TransactionMethods:
    TransactionCustomMethods
    + TransactionGetters
    + TransactionSetters
    + TransactionPrivateMethods
    + std::fmt::Debug
{
    /// DO NOT MODIFY THIS METHOD
    fn process_transaction(
        &mut self,
        client: &mut impl FuzzClient,
        config: &TridentConfig,
        fuzz_accounts: &mut Self::IxAccounts,
    ) -> Result<(), FuzzingError> {
        // // get discriminators
        // let discriminators = self.get_instruction_discriminators();

        // // get program ids
        // let program_ids = self.get_instruction_program_ids();

        // // get data
        // let data = self.get_instruction_data(client, fuzz_accounts);

        // // get accounts
        // let accounts = self.get_instruction_accounts(client, fuzz_accounts);

        // #[allow(unexpected_cfgs)]
        // {
        //     if cfg!(fuzzing_debug) {
        //         println!(
        //             "\x1b[96mCurrently processing transaction with instructions\x1b[0m: {:#?}",
        //             self
        //         );
        //     }
        // }

        // // create instructions
        // let instructions: Vec<Instruction> =
        //     itertools::multizip((discriminators, program_ids, data, accounts))
        //         .map(|(discriminator, program_id, data, accounts)| {
        //             let mut ix_data = vec![];
        //             ix_data.extend(discriminator);
        //             ix_data.extend(data);

        //             Instruction {
        //                 program_id,
        //                 data: ix_data,
        //                 accounts,
        //             }
        //         })
        //         .collect();

        let instructions = self.create_transaction(client, fuzz_accounts);

        // If stats are enabled, log the invocation of the transaction
        if config.get_fuzzing_with_stats() {
            let mut stats_logger = FuzzingStatistics::new();

            stats_logger.increase_invoked(self.get_transaction_name());

            // Let the user perform custom pre-transaction behavior
            self.pre_transaction(client);

            // obtain snapshot of the accounts before the transaction is executed
            self.set_snapshot_before(client);

            // Execute the transaction
            let tx_result = client.process_instructions(&instructions);

            // Check the result of the instruction execution
            match tx_result {
                Ok(_) => {
                    // Log the successful execution of the transaction
                    stats_logger.increase_successful(self.get_transaction_name());

                    // Capture the accounts after the transaction is executed
                    self.set_snapshot_after(client);

                    // Let the user perform custom checks on the accounts
                    if let Err(e) = self.transaction_invariant_check() {
                        // Log the failure of the custom check
                        stats_logger.increase_failed_check(self.get_transaction_name());
                        stats_logger.output_serialized();

                        eprintln!(
                            "\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} instruction did not pass!",
                            self.get_transaction_name()
                        );
                        panic!("{}", e)
                    }

                    // output the stats
                    stats_logger.output_serialized();

                    // Let the user perform custom post-instruction behavior
                    self.post_transaction(client);
                }
                Err(e) => {
                    // Log the failure of the instruction execution
                    stats_logger.increase_failed(self.get_transaction_name());
                    stats_logger.output_serialized();

                    // Let use use transaction error handler to handle the error
                    self.transaction_error_handler(e)?
                }
            }
        } else {
            let mut stats_logger = FuzzingStatistics::new();

            stats_logger.increase_invoked(self.get_transaction_name());

            // obtain snapshot of the accounts before the transaction is executed
            self.set_snapshot_before(client);

            // Let the user perform custom pre-transaction behavior
            self.pre_transaction(client);

            // Execute the instruction
            let tx_result = client.process_instructions(&instructions);

            // Check the result of the instruction execution
            match tx_result {
                Ok(_) => {
                    // Log the successful execution of the instruction
                    stats_logger.increase_successful(self.get_transaction_name());

                    // Capture the accounts after the instruction is executed
                    self.set_snapshot_after(client);

                    // Get the snapshot of the accounts before and after the instruction execution
                    // let (acc_before, acc_after) = snapshot.get_snapshot();

                    // Let the user perform custom checks on the accounts
                    if let Err(e) = self.transaction_invariant_check() {
                        // Log the failure of the custom check
                        stats_logger.increase_failed_check(self.get_transaction_name());
                        stats_logger.output_serialized();

                        eprintln!(
                            "\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} transaction did not pass!",
                            self.get_transaction_name()
                        );
                        panic!("{}", e)
                    }

                    // output the stats
                    stats_logger.output_serialized();

                    // Let the user perform custom post-instruction behavior
                    self.post_transaction(client);
                }
                Err(e) => {
                    // Log the failure of the instruction execution
                    stats_logger.increase_failed(self.get_transaction_name());
                    stats_logger.output_serialized();

                    // Let use use transaction error handler to handle the error
                    self.transaction_error_handler(e)?
                }
            }
        }

        Ok(())
    }
}
