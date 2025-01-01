use std::cell::RefCell;

use solana_sdk::instruction::Instruction;

use crate::{
    error::{FuzzClientErrorWithOrigin, Origin},
    fuzz_client::FuzzClient,
    fuzz_stats::FuzzingStatistics,
    ix_ops::IxOps,
    snapshot::Snapshot,
};

use trident_config::Config;

pub struct TransactionExecutor;

impl TransactionExecutor {
    pub fn process_transaction<I>(
        instruction_name: &str,
        client: &mut impl FuzzClient,
        ix: &I,
        config: &Config,
        accounts: &RefCell<I::IxAccounts>,
    ) -> core::result::Result<(), FuzzClientErrorWithOrigin>
    where
        I: IxOps,
    {
        let program_id = ix.get_program_id();

        let (_signers, account_metas) = ix
            .get_accounts(client, &mut accounts.borrow_mut())
            .map_err(|e| e.with_origin(Origin::Instruction(instruction_name.to_owned())))
            .expect("Accounts calculation expect");

        let data = ix
            .get_data(client, &mut accounts.borrow_mut())
            .map_err(|e| e.with_origin(Origin::Instruction(instruction_name.to_owned())))
            .expect("Data calculation expect");

        let mut snapshot = Snapshot::new(&account_metas);

        snapshot.capture_before(client).unwrap();

        let ixx = Instruction {
            program_id,
            accounts: account_metas,
            data: data.clone(),
        };

        if config.get_fuzzing_with_stats() {
            let mut stats_logger = FuzzingStatistics::new();

            stats_logger.increase_invoked(instruction_name.to_owned());

            let tx_result = client
                .process_instructions(&[ixx])
                .map_err(|e| e.with_origin(Origin::Instruction(instruction_name.to_owned())));
            match tx_result {
                Ok(_) => {
                    stats_logger.increase_successful(instruction_name.to_owned());

                    snapshot.capture_after(client).unwrap();
                    let (acc_before, acc_after) = snapshot.get_snapshot();
                    if let Err(e) = ix.check(acc_before, acc_after, data).map_err(|e| {
                        e.with_origin(Origin::Instruction(instruction_name.to_owned()))
                    }) {
                        stats_logger.increase_failed_check(instruction_name.to_owned());
                        stats_logger.output_serialized();

                        eprintln!("\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} instruction did not pass!",instruction_name.to_owned());
                        panic!("{}", e)
                    }
                    stats_logger.output_serialized();
                }
                Err(e) => {
                    stats_logger.increase_failed(instruction_name.to_owned());
                    stats_logger.output_serialized();

                    let raw_accounts = snapshot.get_before();
                    ix.tx_error_handler(e, data, raw_accounts)?
                }
            }
        } else {
            let tx_result = client
                .process_instructions(&[ixx])
                .map_err(|e| e.with_origin(Origin::Instruction(instruction_name.to_owned())));
            match tx_result {
                Ok(_) => {
                    snapshot.capture_after(client).unwrap();
                    let (acc_before, acc_after) = snapshot.get_snapshot();

                    if let Err(e) = ix.check(acc_before, acc_after, data).map_err(|e| {
                        e.with_origin(Origin::Instruction(instruction_name.to_owned()))
                    }) {
                        eprintln!("\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} instruction did not pass!",instruction_name.to_owned());
                        panic!("{}", e)
                    }
                }
                Err(e) => {
                    let raw_accounts = snapshot.get_before();
                    ix.tx_error_handler(e, data, raw_accounts)?
                }
            }
        }
        Ok(())
    }
}
