use std::{cell::RefCell, collections::HashMap};

use anchor_lang::InstructionData;
use solana_banks_client::BanksClientError;
use solana_sdk::{
    instruction::Instruction, signature::Keypair, signer::Signer, transaction::Transaction,
};

use crate::{
    config::Config,
    error::{FuzzClientError, FuzzClientErrorWithOrigin, Origin},
    fuzz_client::FuzzClient,
    fuzz_stats::FuzzingStatistics,
    ix_ops::IxOps,
    snapshot::Snapshot,
};

pub struct TransactionExecutor;

impl TransactionExecutor {
    #[allow(clippy::too_many_arguments)]
    pub fn process_transaction<'info, I>(
        instruction_name: &str,
        client: &mut impl FuzzClient,
        ix: &'info I,
        snapshot: &'info mut Snapshot<'info, I>,
        sent_txs: &mut HashMap<anchor_lang::solana_program::hash::Hash, ()>,
        config: &Config,
        accounts: &RefCell<I::IxAccounts>,
    ) -> core::result::Result<(), FuzzClientErrorWithOrigin>
    where
        I: IxOps<'info>,
    {
        let program_id = ix.get_program_id();

        let (mut signers, account_metas) = ix
            .get_accounts(client, &mut accounts.borrow_mut())
            .map_err(|e| e.with_origin(Origin::Instruction(instruction_name.to_owned())))
            .expect("Accounts calculation expect");

        let data = ix
            .get_data(client, &mut accounts.borrow_mut())
            .map_err(|e| e.with_origin(Origin::Instruction(instruction_name.to_owned())))
            .expect("Data calculation expect");

        snapshot.add_metas(&account_metas);

        snapshot.capture_before(client).unwrap();

        let ixx = Instruction {
            program_id,
            accounts: account_metas,
            data: data.data(),
        };

        let mut transaction = Transaction::new_with_payer(&[ixx], Some(&client.payer().pubkey()));

        signers.push(client.payer().insecure_clone());
        let sig: Vec<&Keypair> = signers.iter().collect();
        transaction.sign(&sig, client.get_last_blockhash());

        let duplicate_tx = if config.get_allow_duplicate_txs() {
            None
        } else {
            let message_hash = transaction.message().hash();
            sent_txs.insert(message_hash, ())
        };

        match duplicate_tx {
            Some(_) => eprintln!(
                "\x1b[1;93mWarning\x1b[0m: Skipping duplicate instruction `{}`",
                instruction_name.to_owned()
            ),
            None => {
                if config.get_fuzzing_with_stats() {
                    let mut stats_logger = FuzzingStatistics::new();

                    stats_logger.increase_invoked(instruction_name.to_owned());

                    let tx_result = client
                        .process_transaction_with_metadata(transaction)
                        .map_err(|e| {
                            e.with_origin(Origin::Instruction(instruction_name.to_owned()))
                        });
                    match tx_result {
                        Ok(result_with_metadata) => {
                            // We got metadata => transaction was either successful or returned with TransactionError
                            match result_with_metadata.result {
                                Ok(()) => {
                                    if let Some(metadata) = result_with_metadata.metadata {
                                        stats_logger.update_cu_stats(
                                            instruction_name.to_owned(),
                                            metadata.compute_units_consumed,
                                        );
                                    }
                                    stats_logger.increase_successful(instruction_name.to_owned());

                                    // handle snapshot stuff
                                    snapshot.capture_after(client).unwrap();
                                    let (acc_before, acc_after) = snapshot
                                        .get_snapshot()
                                        .map_err(|e| {
                                            e.with_origin(Origin::Instruction(
                                                instruction_name.to_owned(),
                                            ))
                                        })
                                        .expect("Snapshot deserialization expect"); // we want to panic if we cannot unwrap to cause a crash

                                    if let Err(e) =
                                        ix.check(acc_before, acc_after, data).map_err(|e| {
                                            e.with_origin(Origin::Instruction(
                                                instruction_name.to_owned(),
                                            ))
                                        })
                                    {
                                        stats_logger
                                            .increase_failed_check(instruction_name.to_owned());
                                        stats_logger.output_serialized();

                                        eprintln!("\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} instruction did not pass!",instruction_name.to_owned());
                                        panic!("{}", e)
                                    }
                                    stats_logger.output_serialized();
                                }
                                Err(e) => {
                                    if let Some(metadata) = result_with_metadata.metadata {
                                        stats_logger.update_failed_cu_stats(
                                            instruction_name.to_owned(),
                                            metadata.compute_units_consumed,
                                        );
                                    }
                                    stats_logger.increase_failed(instruction_name.to_owned());
                                    stats_logger.output_serialized();

                                    let fuzz_err = FuzzClientError::from(
                                        BanksClientError::TransactionError(e),
                                    );
                                    let mut raw_accounts = snapshot.get_raw_pre_ix_accounts();
                                    ix.tx_error_handler(
                                        fuzz_err.with_origin(Origin::Instruction(
                                            instruction_name.to_owned(),
                                        )),
                                        data,
                                        &mut raw_accounts,
                                    )?
                                }
                            }
                        }
                        // No metadata => one of ClientError, Io, RpcError or SimulationError errors occured
                        Err(e) => {
                            stats_logger.increase_failed(instruction_name.to_owned());
                            stats_logger.output_serialized();

                            let mut raw_accounts = snapshot.get_raw_pre_ix_accounts();
                            ix.tx_error_handler(e, data, &mut raw_accounts)?
                        }
                    }
                } else {
                    let tx_result = client.process_transaction(transaction).map_err(|e| {
                        e.with_origin(Origin::Instruction(instruction_name.to_owned()))
                    });
                    match tx_result {
                        Ok(_) => {
                            snapshot.capture_after(client).unwrap();
                            let (acc_before, acc_after) = snapshot
                                .get_snapshot()
                                .map_err(|e| {
                                    e.with_origin(Origin::Instruction(instruction_name.to_owned()))
                                })
                                .expect("Snapshot deserialization expect"); // we want to panic if we cannot unwrap to cause a crash

                            if let Err(e) = ix.check(acc_before, acc_after, data).map_err(|e| {
                                e.with_origin(Origin::Instruction(instruction_name.to_owned()))
                            }) {
                                eprintln!("\x1b[31mCRASH DETECTED!\x1b[0m Custom check after the {} instruction did not pass!",instruction_name.to_owned());
                                panic!("{}", e)
                            }
                        }
                        Err(e) => {
                            let mut raw_accounts = snapshot.get_raw_pre_ix_accounts();
                            ix.tx_error_handler(e, data, &mut raw_accounts)?
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
