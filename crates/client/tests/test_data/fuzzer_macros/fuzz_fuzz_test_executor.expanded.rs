use trident_client::FuzzTestExecutor;
pub enum FuzzInstruction {
    InitVesting(InitVesting),
    WithdrawUnlocked(WithdrawUnlocked),
}
impl FuzzTestExecutor<FuzzAccounts> for FuzzInstruction {
    fn run_fuzzer(
        &self,
        program_id: Pubkey,
        accounts: &RefCell<FuzzAccounts>,
        client: &mut impl FuzzClient,
        sent_txs: &mut HashMap<Hash, ()>,
    ) -> core::result::Result<(), FuzzClientErrorWithOrigin> {
        match self {
            FuzzInstruction::InitVesting(ix) => {
                let (mut signers, metas) = ix
                    .get_accounts(client, &mut accounts.borrow_mut())
                    .map_err(|e| {
                        e.with_origin(Origin::Instruction(self.to_context_string()))
                    })
                    .expect("Accounts calculation expect");
                let mut snaphot = Snapshot::new(&metas, ix);
                snaphot.capture_before(client).unwrap();
                let data = ix
                    .get_data(client, &mut accounts.borrow_mut())
                    .map_err(|e| {
                        e.with_origin(Origin::Instruction(self.to_context_string()))
                    })
                    .expect("Data calculation expect");
                let ixx = Instruction {
                    program_id,
                    accounts: metas.clone(),
                    data: data.data(),
                };
                let mut transaction = Transaction::new_with_payer(
                    &[ixx],
                    Some(&client.payer().pubkey()),
                );
                signers.push(client.payer().clone());
                let sig: Vec<&Keypair> = signers.iter().collect();
                transaction.sign(&sig, client.get_last_blockhash());
                let duplicate_tx = if false {
                    None
                } else {
                    let message_hash = transaction.message().hash();
                    sent_txs.insert(message_hash, ())
                };
                match duplicate_tx {
                    Some(_) => {
                        ::std::io::_eprint(
                            format_args!(
                                "\u{1b}[1;93mWarning\u{1b}[0m: Skipping duplicate instruction `{0}`\n",
                                self.to_context_string(),
                            ),
                        );
                    }
                    None => {
                        let tx_result = client
                            .process_transaction(transaction)
                            .map_err(|e| {
                                e.with_origin(Origin::Instruction(self.to_context_string()))
                            });
                        match tx_result {
                            Ok(_) => {
                                snaphot.capture_after(client).unwrap();
                                let (acc_before, acc_after) = snaphot
                                    .get_snapshot()
                                    .map_err(|e| {
                                        e.with_origin(Origin::Instruction(self.to_context_string()))
                                    })
                                    .expect("Snapshot deserialization expect");
                                if let Err(e)
                                    = ix
                                        .check(acc_before, acc_after, data)
                                        .map_err(|e| {
                                            e.with_origin(Origin::Instruction(self.to_context_string()))
                                        })
                                {
                                    {
                                        ::std::io::_eprint(
                                            format_args!(
                                                "\u{1b}[31mCRASH DETECTED!\u{1b}[0m Custom check after the {0} instruction did not pass!\n",
                                                self.to_context_string(),
                                            ),
                                        );
                                    };
                                    {
                                        #[cold]
                                        #[track_caller]
                                        #[inline(never)]
                                        #[rustc_const_panic_str]
                                        #[rustc_do_not_const_check]
                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                            arg: &T,
                                        ) -> ! {
                                            ::core::panicking::panic_display(arg)
                                        }
                                        panic_cold_display(&e);
                                    }
                                }
                            }
                            Err(e) => {
                                let mut raw_accounts = snaphot.get_raw_pre_ix_accounts();
                                ix.tx_error_handler(e, data, &mut raw_accounts)?
                            }
                        }
                    }
                }
            }
            FuzzInstruction::WithdrawUnlocked(ix) => {
                let (mut signers, metas) = ix
                    .get_accounts(client, &mut accounts.borrow_mut())
                    .map_err(|e| {
                        e.with_origin(Origin::Instruction(self.to_context_string()))
                    })
                    .expect("Accounts calculation expect");
                let mut snaphot = Snapshot::new(&metas, ix);
                snaphot.capture_before(client).unwrap();
                let data = ix
                    .get_data(client, &mut accounts.borrow_mut())
                    .map_err(|e| {
                        e.with_origin(Origin::Instruction(self.to_context_string()))
                    })
                    .expect("Data calculation expect");
                let ixx = Instruction {
                    program_id,
                    accounts: metas.clone(),
                    data: data.data(),
                };
                let mut transaction = Transaction::new_with_payer(
                    &[ixx],
                    Some(&client.payer().pubkey()),
                );
                signers.push(client.payer().clone());
                let sig: Vec<&Keypair> = signers.iter().collect();
                transaction.sign(&sig, client.get_last_blockhash());
                let duplicate_tx = if false {
                    None
                } else {
                    let message_hash = transaction.message().hash();
                    sent_txs.insert(message_hash, ())
                };
                match duplicate_tx {
                    Some(_) => {
                        ::std::io::_eprint(
                            format_args!(
                                "\u{1b}[1;93mWarning\u{1b}[0m: Skipping duplicate instruction `{0}`\n",
                                self.to_context_string(),
                            ),
                        );
                    }
                    None => {
                        let tx_result = client
                            .process_transaction(transaction)
                            .map_err(|e| {
                                e.with_origin(Origin::Instruction(self.to_context_string()))
                            });
                        match tx_result {
                            Ok(_) => {
                                snaphot.capture_after(client).unwrap();
                                let (acc_before, acc_after) = snaphot
                                    .get_snapshot()
                                    .map_err(|e| {
                                        e.with_origin(Origin::Instruction(self.to_context_string()))
                                    })
                                    .expect("Snapshot deserialization expect");
                                if let Err(e)
                                    = ix
                                        .check(acc_before, acc_after, data)
                                        .map_err(|e| {
                                            e.with_origin(Origin::Instruction(self.to_context_string()))
                                        })
                                {
                                    {
                                        ::std::io::_eprint(
                                            format_args!(
                                                "\u{1b}[31mCRASH DETECTED!\u{1b}[0m Custom check after the {0} instruction did not pass!\n",
                                                self.to_context_string(),
                                            ),
                                        );
                                    };
                                    {
                                        #[cold]
                                        #[track_caller]
                                        #[inline(never)]
                                        #[rustc_const_panic_str]
                                        #[rustc_do_not_const_check]
                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                            arg: &T,
                                        ) -> ! {
                                            ::core::panicking::panic_display(arg)
                                        }
                                        panic_cold_display(&e);
                                    }
                                }
                            }
                            Err(e) => {
                                let mut raw_accounts = snaphot.get_raw_pre_ix_accounts();
                                ix.tx_error_handler(e, data, &mut raw_accounts)?
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
pub struct InitVesting {
    pub accounts: InitVestingAccounts,
    pub data: InitVestingData,
}
pub struct InitVestingAccounts {
    pub sender: AccountId,
    pub sender_token_account: AccountId,
    pub escrow: AccountId,
    pub escrow_token_account: AccountId,
    pub mint: AccountId,
    pub token_program: AccountId,
    pub system_program: AccountId,
}
pub struct InitVestingData {
    pub recipient: AccountId,
    pub amount: u64,
    pub start_at: u64,
    pub end_at: u64,
    pub interval: u64,
}
pub struct WithdrawUnlocked {
    pub accounts: WithdrawUnlockedAccounts,
    pub data: WithdrawUnlockedData,
}
pub struct WithdrawUnlockedAccounts {
    pub recipient: AccountId,
    pub recipient_token_account: AccountId,
    pub escrow: AccountId,
    pub escrow_token_account: AccountId,
    pub escrow_pda_authority: AccountId,
    pub mint: AccountId,
    pub token_program: AccountId,
    pub system_program: AccountId,
}
pub struct WithdrawUnlockedData {}
