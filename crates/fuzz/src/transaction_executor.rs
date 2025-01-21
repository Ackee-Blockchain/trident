use std::cell::RefCell;

use solana_sdk::instruction::Instruction;

use crate::error::FuzzClientError;
use crate::error::FuzzClientErrorWithOrigin;
use crate::error::Origin;
use crate::fuzz_client::FuzzClient;
use crate::fuzz_stats::FuzzingStatistics;
use crate::ix_ops::IxOps;
use crate::snapshot::Snapshot;

use trident_config::TridentConfig;

pub struct TransactionExecutor;

impl TransactionExecutor {
    pub fn process_transaction<I>(
        instruction_name: &str,
        client: &mut impl FuzzClient,
        ix: &I,
        config: &TridentConfig,
        accounts: &RefCell<I::IxAccounts>,
    ) -> core::result::Result<(), FuzzClientErrorWithOrigin>
    where
        I: IxOps,
    {
        // Obtain the program id
        let program_id = ix.get_program_id();

        // Obtain the instruction data
        let data = ix
            .get_data(client, &mut accounts.borrow_mut())
            .map_err(|e| e.with_origin(Origin::Instruction(instruction_name.to_owned())))
            .expect("Data calculation expect");

        // Obtain the account metas and signers
        let (_signers, account_metas) = ix
            .get_accounts(client, &mut accounts.borrow_mut())
            .map_err(|e| e.with_origin(Origin::Instruction(instruction_name.to_owned())))
            .expect("Accounts calculation expect");

        // Initializes the snapshot from the account metas
        let mut snapshot = Snapshot::new(&account_metas);

        // Capture the accounts before the instruction is executed
        snapshot.capture_before(client).unwrap();

        // Create the instruction to be executed
        let ixx = Instruction {
            program_id,
            accounts: account_metas,
            data: data.clone(),
        };

        // If stats are enabled, log the invocation of the instruction
        if config.get_fuzzing_with_stats() {
            let mut stats_logger = FuzzingStatistics::new();

            stats_logger.increase_invoked(instruction_name.to_owned());

            // Execute the instruction
            let tx_result = client.process_instructions(&[ixx]);

            // Check the result of the instruction execution
            match tx_result {
                Ok(_) => {
                    // Log the successful execution of the instruction
                    stats_logger.increase_successful(instruction_name.to_owned());

                    // Capture the accounts after the instruction is executed
                    snapshot.capture_after(client).unwrap();

                    // Get the snapshot of the accounts before and after the instruction execution
                    let (acc_before, acc_after) = snapshot.get_snapshot();

                    // Let the user perform custom checks on the accounts
                    if let Err(e) = ix.check(acc_before, acc_after, data).map_err(|e| {
                        e.with_origin(Origin::Instruction(instruction_name.to_owned()))
                    }) {
                        // Log the failure of the custom check
                        stats_logger.increase_failed_check(instruction_name.to_owned());
                        stats_logger.output_serialized();

                        eprintln!("\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} instruction did not pass!",instruction_name.to_owned());
                        panic!("{}", e)
                    }

                    stats_logger.output_serialized();

                    // Let the user perform custom post-instruction behavior
                    ix.post_instruction(client, acc_after);
                }
                Err(e) => {
                    // Log the failure of the instruction execution
                    stats_logger.increase_failed(instruction_name.to_owned());
                    stats_logger.output_serialized();

                    // Let use use transaction error handler to handle the error
                    let raw_accounts = snapshot.get_before();
                    ix.tx_error_handler(e, data, raw_accounts).map_err(|e| {
                        FuzzClientError::from(e)
                            .with_origin(Origin::Instruction(instruction_name.to_owned()))
                    })?
                }
            }
        } else {
            // If stats are not enabled, execute the instruction directly
            let tx_result = client.process_instructions(&[ixx]);
            match tx_result {
                Ok(_) => {
                    // Capture the accounts after the instruction is executed
                    snapshot.capture_after(client).unwrap();

                    // Get the snapshot of the accounts before and after the instruction execution
                    let (acc_before, acc_after) = snapshot.get_snapshot();

                    // Let the user perform custom checks on the accounts
                    if let Err(e) = ix.check(acc_before, acc_after, data).map_err(|e| {
                        e.with_origin(Origin::Instruction(instruction_name.to_owned()))
                    }) {
                        eprintln!("\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} instruction did not pass!",instruction_name.to_owned());
                        panic!("{}", e)
                    }

                    // Let the user perform custom post-instruction behavior
                    ix.post_instruction(client, acc_after);
                }
                Err(e) => {
                    // Let use use transaction error handler to handle the error
                    let raw_accounts = snapshot.get_before();
                    ix.tx_error_handler(e, data, raw_accounts).map_err(|e| {
                        FuzzClientError::from(e)
                            .with_origin(Origin::Instruction(instruction_name.to_owned()))
                    })?
                }
            }
        }
        Ok(())
    }
}
