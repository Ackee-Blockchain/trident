use anchor_lang::AccountDeserialize;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    InitVesting(InitVesting),
    WithdrawUnlocked(WithdrawUnlocked),
}
#[derive(Arbitrary, Debug)]
pub struct InitVesting {
    pub accounts: InitVestingAccounts,
    pub data: InitVestingData,
}
#[derive(Arbitrary, Debug)]
pub struct InitVestingAccounts {
    pub sender: AccountId,
    pub sender_token_account: AccountId,
    pub escrow: AccountId,
    pub escrow_token_account: AccountId,
    pub mint: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct InitVestingData {
    pub recipient: AccountId,
    pub amount: u64,
    pub start_at: u64,
    pub end_at: u64,
    pub interval: u64,
}
// -------------------------------------------------------------------
// -------------------------------------------------------------------
// Implement Arbitrary
impl<'a> Arbitrary<'a> for InitVestingData {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        // obtain AccountId
        let recipient = AccountId::arbitrary(u)?;

        // limit the generated amount to the 1_000_000
        let amount = u.int_in_range(1..=1_000_000)?;

        // now we want to obtain
        // - start_at
        // - end_at
        // - interval
        // however we want to limit the data such that:
        // - start_at < end_at
        // - end_at - start_at > interval
        // - interval has lower limit of 500 and upper limit of 1000.

        let start_at: u64 = u.int_in_range(1_000_000..=5_000_000)?;
        let end_at: u64 = u.int_in_range(1_000_000..=5_000_000)?;
        let interval: u64 = u.int_in_range(500..=1000)?;

        // ensure that start_at < end_at
        if start_at >= end_at {
            return Err(arbitrary::Error::IncorrectFormat);
        }

        // ensure that end_at - start_at > interval
        match end_at.checked_sub(start_at) {
            Some(diff) => {
                if diff <= interval {
                    return Err(arbitrary::Error::IncorrectFormat);
                }
            }
            None => return Err(arbitrary::Error::IncorrectFormat),
        }

        Ok(InitVestingData {
            recipient,
            amount,
            start_at,
            end_at,
            interval,
        })
    }
    // -------------------------------------------------------------------
    // -------------------------------------------------------------------
}
#[derive(Arbitrary, Debug)]
pub struct WithdrawUnlocked {
    pub accounts: WithdrawUnlockedAccounts,
    pub data: WithdrawUnlockedData,
}
#[derive(Arbitrary, Debug)]
pub struct WithdrawUnlockedAccounts {
    pub recipient: AccountId,
    pub recipient_token_account: AccountId,
    pub escrow: AccountId,
    pub escrow_token_account: AccountId,
    pub escrow_pda_authority: AccountId,
    pub mint: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct WithdrawUnlockedData {}
///IxOps implementation for `InitVesting` with all required functions.
impl IxOps for InitVesting {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![119u8, 192u8, 67u8, 41u8, 47u8, 82u8, 152u8, 27u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("AGpdCBtXUyLWKutvMCVDeTywkxgvQVjJk54btLQNLMiZ")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            let recipient = fuzz_accounts
                .recipient_arbitrary_limit_inputs_5
                .get_or_create_account(self.data.recipient, client, 10 * LAMPORTS_PER_SOL);
            args.extend(borsh::to_vec(&recipient.pubkey()).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.amount).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.start_at).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.end_at).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.interval).unwrap());
        }
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let mut signers = vec![];
        let recipient = fuzz_accounts
            .recipient_arbitrary_limit_inputs_5
            .get_or_create_account(self.data.recipient, client, 10 * LAMPORTS_PER_SOL);

        let sender = {
            let sender = fuzz_accounts
                .sender_arbitrary_limit_inputs_5
                .get_or_create_account(self.accounts.sender, client, 500 * LAMPORTS_PER_SOL);
            account_metas.push(AccountMeta::new(sender.pubkey(), true));
            signers.push(sender.insecure_clone());
            sender.pubkey()
        };
        let mint = fuzz_accounts
            .mint_arbitrary_limit_inputs_5
            .get_or_create_mint_account(self.accounts.mint, client, 6, &sender, None);

        {
            let sender_token_account = fuzz_accounts
                .sender_token_account_arbitrary_limit_inputs_5
                .get_or_create_token_account(
                    self.accounts.sender_token_account,
                    client,
                    mint.pubkey(),
                    sender,
                    u64::MAX,
                    None,
                    None,
                    0,
                    None,
                );
            account_metas.push(AccountMeta::new(sender_token_account.pubkey(), false));
        }
        {
            let escrow = fuzz_accounts
                .escrow_arbitrary_limit_inputs_5
                .get_or_create_account(
                    self.accounts.escrow,
                    client,
                    &[recipient.pubkey().as_ref(), b"ESCROW_SEED"],
                    &self.get_program_id(),
                );
            account_metas.push(AccountMeta::new(escrow, false));
        }
        {
            let escrow_token_account = fuzz_accounts
                .escrow_token_account_arbitrary_limit_inputs_5
                .get_or_create_token_account(
                    self.accounts.escrow_token_account,
                    client,
                    mint.pubkey(),
                    sender,
                    0,
                    None,
                    None,
                    0,
                    None,
                );
            account_metas.push(AccountMeta::new(escrow_token_account.pubkey(), false));
        }
        {
            account_metas.push(AccountMeta::new_readonly(mint.pubkey(), false));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        Ok((signers, account_metas))
    }
}
///IxOps implementation for `WithdrawUnlocked` with all required functions.
impl IxOps for WithdrawUnlocked {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![213u8, 161u8, 76u8, 199u8, 38u8, 28u8, 209u8, 80u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("AGpdCBtXUyLWKutvMCVDeTywkxgvQVjJk54btLQNLMiZ")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let mut signers = vec![];

        let mint = fuzz_accounts
            .mint_arbitrary_limit_inputs_5
            .get(self.accounts.mint);

        let recipient = {
            let recipient = fuzz_accounts
                .recipient_arbitrary_limit_inputs_5
                .get_or_create_account(self.accounts.recipient, client, 500 * LAMPORTS_PER_SOL);
            account_metas.push(AccountMeta::new(recipient.pubkey(), true));
            signers.push(recipient.insecure_clone());
            recipient.pubkey()
        };

        let escrow_pda_authority = fuzz_accounts
            .escrow_pda_authority_arbitrary_limit_inputs_5
            .get_or_create_account(
                self.accounts.escrow_pda_authority,
                client,
                &[b"ESCROW_PDA_AUTHORITY"],
                &self.get_program_id(),
            );
        {
            let recipient_token_account = fuzz_accounts
                .recipient_token_account_arbitrary_limit_inputs_5
                .get_or_create_token_account(
                    self.accounts.recipient_token_account,
                    client,
                    mint.pubkey(),
                    recipient,
                    0,
                    None,
                    None,
                    0,
                    None,
                );
            account_metas.push(AccountMeta::new(recipient_token_account.pubkey(), false));
        }
        {
            let escrow = fuzz_accounts
                .escrow_arbitrary_limit_inputs_5
                .get_or_create_account(
                    self.accounts.escrow,
                    client,
                    &[recipient.as_ref(), b"ESCROW_SEED"],
                    &self.get_program_id(),
                );
            account_metas.push(AccountMeta::new(escrow, false));
        }
        {
            let escrow_token_account = fuzz_accounts
                .escrow_token_account_arbitrary_limit_inputs_5
                .get_or_create_token_account(
                    self.accounts.escrow_token_account,
                    client,
                    mint.pubkey(),
                    escrow_pda_authority,
                    0,
                    None,
                    None,
                    0,
                    None,
                );
            account_metas.push(AccountMeta::new(escrow_token_account.pubkey(), false));
        }
        {
            account_metas.push(AccountMeta::new_readonly(escrow_pda_authority, false));
        }
        {
            account_metas.push(AccountMeta::new(mint.pubkey(), false));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        Ok((signers, account_metas))
    }
    fn check(
        &self,
        pre_ix: &[SnapshotAccount],
        post_ix: &[SnapshotAccount],
        _ix_data: Vec<u8>,
    ) -> Result<(), FuzzingError> {
        if let Ok(escrow) = Escrow::deserialize(&mut pre_ix[2].data_no_discriminator()) {
            let recipient = pre_ix[0].pubkey();

            let recipient_token_account_pre =
                match anchor_spl::token::TokenAccount::try_deserialize(&mut pre_ix[1].data()) {
                    Ok(recipient_token_account_pre) => recipient_token_account_pre,
                    Err(_) => return Ok(()),
                };

            let recipient_token_account_post =
                match anchor_spl::token::TokenAccount::try_deserialize(&mut post_ix[1].data()) {
                    Ok(recipient_token_account_post) => recipient_token_account_post,
                    Err(_) => return Ok(()),
                };

            if escrow.recipient == recipient {
                if recipient_token_account_pre.amount == recipient_token_account_post.amount {
                    // Recipient was not able to withdraw
                    return Err(FuzzingError::BalanceMismatch);
                } else if recipient_token_account_pre.amount + escrow.amount
                    != recipient_token_account_post.amount
                {
                    if recipient_token_account_pre.amount + escrow.amount
                        > recipient_token_account_post.amount
                    {
                        // Recipient withdraw less
                        return Err(FuzzingError::Custom(15));
                    } else {
                        // Recipient withdraw more
                        return Err(FuzzingError::Custom(2));
                    }
                }
            }
        }
        Ok(())
    }
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    escrow_arbitrary_limit_inputs_5: AccountsStorage<PdaStore>,
    escrow_pda_authority_arbitrary_limit_inputs_5: AccountsStorage<PdaStore>,
    escrow_token_account_arbitrary_limit_inputs_5: AccountsStorage<KeypairStore>,
    mint_arbitrary_limit_inputs_5: AccountsStorage<KeypairStore>,
    recipient_arbitrary_limit_inputs_5: AccountsStorage<KeypairStore>,
    recipient_token_account_arbitrary_limit_inputs_5: AccountsStorage<KeypairStore>,
    sender_arbitrary_limit_inputs_5: AccountsStorage<KeypairStore>,
    sender_token_account_arbitrary_limit_inputs_5: AccountsStorage<KeypairStore>,
}
/// Custom Types defined within the Solana Program
/// These are important in order to be able to Serialize/Deserialize
/// all possible instruction inputs.
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Escrow {
    recipient: Pubkey,
    amount: u64,
    withdrawal: u64,
    start_time: u64,
    end_time: u64,
    interval: u64,
    bump: u8,
}
