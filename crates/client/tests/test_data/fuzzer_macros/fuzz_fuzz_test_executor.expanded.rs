use trdelnik_client::FuzzTestExecutor;
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
    ) -> core::result::Result<(), Box<dyn std::error::Error + 'static>> {
        match self {
            FuzzInstruction::InitVesting(ix) => {
                let (mut signers, metas) = if let Ok(acc)
                    = ix
                        .get_accounts(client, &mut accounts.borrow_mut())
                        .map_err(|e| {
                            e.with_origin(Origin::Instruction(self.to_context_string()))
                        })
                {
                    acc
                } else {
                    return Ok(());
                };
                let mut snaphot = Snapshot::new(&metas, ix);
                snaphot.capture_before(client).unwrap();
                let data = if let Ok(data)
                    = ix
                        .get_data(client, &mut accounts.borrow_mut())
                        .map_err(|e| {
                            e.with_origin(Origin::Instruction(self.to_context_string()))
                        })
                {
                    data
                } else {
                    return Ok(());
                };
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
                let res = client
                    .process_transaction(transaction)
                    .map_err(|e| {
                        e.with_origin(Origin::Instruction(self.to_context_string()))
                    });
                snaphot.capture_after(client).unwrap();
                let (acc_before, acc_after) = snaphot
                    .get_snapshot()
                    .map_err(|e| {
                        e.with_origin(Origin::Instruction(self.to_context_string()))
                    })
                    .unwrap();
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
                                "Custom check after the {0} instruction did not pass with the error message: {1}\n",
                                self.to_context_string(), e,
                            ),
                        );
                    };
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Instruction data submitted to the instruction were:\n",
                            ),
                        );
                    };
                    {
                        ::core::panicking::panic_display(&e);
                    }
                }
                if res.is_err() {
                    return Ok(());
                }
            }
            FuzzInstruction::WithdrawUnlocked(ix) => {
                let (mut signers, metas) = if let Ok(acc)
                    = ix
                        .get_accounts(client, &mut accounts.borrow_mut())
                        .map_err(|e| {
                            e.with_origin(Origin::Instruction(self.to_context_string()))
                        })
                {
                    acc
                } else {
                    return Ok(());
                };
                let mut snaphot = Snapshot::new(&metas, ix);
                snaphot.capture_before(client).unwrap();
                let data = if let Ok(data)
                    = ix
                        .get_data(client, &mut accounts.borrow_mut())
                        .map_err(|e| {
                            e.with_origin(Origin::Instruction(self.to_context_string()))
                        })
                {
                    data
                } else {
                    return Ok(());
                };
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
                let res = client
                    .process_transaction(transaction)
                    .map_err(|e| {
                        e.with_origin(Origin::Instruction(self.to_context_string()))
                    });
                snaphot.capture_after(client).unwrap();
                let (acc_before, acc_after) = snaphot
                    .get_snapshot()
                    .map_err(|e| {
                        e.with_origin(Origin::Instruction(self.to_context_string()))
                    })
                    .unwrap();
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
                                "Custom check after the {0} instruction did not pass with the error message: {1}\n",
                                self.to_context_string(), e,
                            ),
                        );
                    };
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Instruction data submitted to the instruction were:\n",
                            ),
                        );
                    };
                    {
                        ::core::panicking::panic_display(&e);
                    }
                }
                if res.is_err() {
                    return Ok(());
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
