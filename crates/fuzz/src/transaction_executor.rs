use std::cell::RefCell;

use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;

use crate::error::FuzzClientError;
use crate::error::FuzzClientErrorWithOrigin;
use crate::error::Origin;
use crate::fuzz_client::FuzzClient;
use crate::fuzz_stats::FuzzingStatistics;
use crate::fuzzing::FuzzTestExecutor;
use crate::fuzzing::TransactionInstructions;
use crate::snapshot::Snapshot;

use trident_config::TridentConfig;

pub struct TransactionExecutor;

impl TransactionExecutor {
    /// Execution instruction for fuzzing one or multiple instructions within one transaction
    pub fn process_instructions_batch<T: FuzzTestExecutor<U>, U>(
        client: &mut impl FuzzClient,
        instructions_batch: &TransactionInstructions<T>,
        config: &TridentConfig,
        accounts: &RefCell<U>,
    ) -> core::result::Result<(), FuzzClientErrorWithOrigin> {
        // check, but should never happen
        if instructions_batch.instructions.is_empty() {
            return Ok(());
        }

        let mut instructions = vec![];
        let mut instruction_names = vec![];
        let mut transaction_account_metas: Vec<Vec<AccountMeta>> = vec![];
        let mut transaction_input_data: Vec<Vec<u8>> = vec![];

        // obtain all instructions and all account metas
        for instruction in instructions_batch.instructions.iter() {
            let program_id = instruction.get_program_id();

            let data = instruction
                .get_data(client, accounts)
                .expect("Data calculation expect");

            let (_signers, account_metas) = instruction
                .get_accounts(client, accounts)
                .expect("Accounts calculation expect");

            let ix = Instruction {
                program_id,
                accounts: account_metas.clone(),
                data: data.clone(),
            };

            instructions.push(ix);
            instruction_names.push(instruction.instruction_name());
            transaction_account_metas.push(account_metas);
            transaction_input_data.push(data);
        }

        // Obtain last instruction from the batch
        let last_instruction = instructions_batch.instructions.last().unwrap();

        // Initializes the snapshot from the account metas
        let mut snapshot = Snapshot::new(&transaction_account_metas, transaction_input_data);

        // Capture the accounts before the transaction is executed
        snapshot.capture_before(client).unwrap();

        // If stats are enabled, log the invocation of the transaction
        if config.get_fuzzing_with_stats() {
            let mut stats_logger = FuzzingStatistics::new();

            stats_logger.increase_invoked(instruction_names.join(" + "));

            // Execute the instruction
            let tx_result = client.process_instructions(&instructions);

            // Check the result of the instruction execution
            match tx_result {
                Ok(_) => {
                    // Log the successful execution of the instruction
                    stats_logger.increase_successful(instruction_names.join(" + "));

                    // Capture the accounts after the instruction is executed
                    snapshot.capture_after(client).unwrap();

                    // Get the snapshot of the accounts before and after the instruction execution
                    let (acc_before, acc_after) = snapshot.get_snapshot();

                    // Let the user perform custom checks on the accounts
                    if let Err(e) = last_instruction
                        .transaction_invariant_check(acc_before, acc_after)
                        .map_err(|e| {
                            e.with_origin(Origin::Instruction(
                                instruction_names.join(" + ").to_owned(),
                            ))
                        })
                    {
                        // Log the failure of the custom check
                        stats_logger.increase_failed_check(instruction_names.join(" + "));
                        stats_logger.output_serialized();

                        eprintln!(
                            "\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} instruction did not pass!",
                            instruction_names.join(" + ")
                        );
                        panic!("{}", e)
                    }

                    // output the stats
                    stats_logger.output_serialized();

                    // Let the user perform custom post-instruction behavior
                    last_instruction.post_transaction(client, acc_after);
                }
                Err(e) => {
                    // Log the failure of the instruction execution
                    stats_logger.increase_failed(instruction_names.join(" + "));
                    stats_logger.output_serialized();

                    // Let use use transaction error handler to handle the error
                    let acc_before = snapshot.get_before();
                    last_instruction
                        .transaction_error_handler(e, acc_before)
                        .map_err(|e| {
                            FuzzClientError::from(e)
                                .with_origin(Origin::Instruction(instruction_names.join(" + ")))
                        })?
                }
            }
        } else {
            // If stats are not enabled, execute the instruction directly
            let tx_result = client.process_instructions(&instructions);
            match tx_result {
                Ok(_) => {
                    // Capture the accounts after the instruction is executed
                    snapshot.capture_after(client).unwrap();

                    // Get the snapshot of the accounts before and after the instruction execution
                    let (acc_before, acc_after) = snapshot.get_snapshot();

                    // Let the user perform custom checks on the accounts
                    if let Err(e) = last_instruction
                        .transaction_invariant_check(acc_before, acc_after)
                        .map_err(|e| {
                            e.with_origin(Origin::Instruction(instruction_names.join(" + ")))
                        })
                    {
                        eprintln!("\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} instruction did not pass!",instruction_names.join(" + "));
                        panic!("{}", e)
                    }

                    // Let the user perform custom post-instruction behavior
                    last_instruction.post_transaction(client, acc_after);
                }
                Err(e) => {
                    // Let use use transaction error handler to handle the error
                    let acc_before = snapshot.get_before();
                    last_instruction
                        .transaction_error_handler(e, acc_before)
                        .map_err(|e| {
                            FuzzClientError::from(e)
                                .with_origin(Origin::Instruction(instruction_names.join(" + ")))
                        })?
                }
            }
        }

        Ok(())
    }
}
