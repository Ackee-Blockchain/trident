use solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_client::fuzzing::*;
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
    pub _mint: AccountId,
    pub _token_program: AccountId,
    pub _system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct InitVestingData {
    pub recipient: AccountId,
    #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(1..=1_000_000))]
    pub amount: u64,
    // we want start_at smaller than end_at
    // and for testing purposes we can run tests with times from the past
    #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1_000_000))]
    pub start_at: u64,
    #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(1_001_001..=1_050_000))]
    pub end_at: u64,
    #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(1..=1000))]
    pub interval: u64,
}
#[derive(Arbitrary, Debug)]
pub struct WithdrawUnlocked {
    pub accounts: WithdrawUnlockedAccounts,
    pub _data: WithdrawUnlockedData,
}
#[derive(Arbitrary, Debug)]
pub struct WithdrawUnlockedAccounts {
    pub recipient: AccountId,
    pub recipient_token_account: AccountId,
    pub escrow: AccountId,
    pub escrow_token_account: AccountId,
    pub escrow_pda_authority: AccountId,
    pub _mint: AccountId,
    pub _token_program: AccountId,
    pub _system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct WithdrawUnlockedData {}
///IxOps implementation for `InitVesting` with all required functions.
impl IxOps for InitVesting {
    type IxData = incorrect_integer_arithmetic_3::instruction::InitVesting;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        incorrect_integer_arithmetic_3::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let recipient = fuzz_accounts.recipient.get_or_create_account(
            self.data.recipient,
            client,
            10 * LAMPORTS_PER_SOL,
        );
        let data = incorrect_integer_arithmetic_3::instruction::InitVesting {
            recipient: recipient.pubkey(),
            amount: self.data.amount,
            start_at: self.data.start_at,
            end_at: self.data.end_at,
            interval: self.data.interval,
        };
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let sender = fuzz_accounts.sender.get_or_create_account(
            self.accounts.sender,
            client,
            5 * LAMPORTS_PER_SOL,
        );
        // INFO use constant Account ID, so we will not generate multiple mints,
        // and also we can easily link to Withdraw
        let mint = fuzz_accounts
            .mint
            .get_or_create_account(0, client, 6, &sender.pubkey(), None);

        let sender_token_account = fuzz_accounts.sender_token_account.get_or_create_account(
            self.accounts.sender_token_account,
            client,
            mint,
            sender.pubkey(),
            u64::MAX,
            None,
            None,
            0,
            None,
        );

        let recipient = fuzz_accounts.recipient.get_or_create_account(
            self.data.recipient,
            client,
            10 * LAMPORTS_PER_SOL,
        );
        let escrow = fuzz_accounts.escrow.get_or_create_account(
            self.accounts.escrow,
            client,
            &[recipient.pubkey().as_ref(), b"ESCROW_SEED"],
            &incorrect_integer_arithmetic_3::ID,
        );

        let escrow_token_account = fuzz_accounts.escrow_token_account.get_or_create_account(
            self.accounts.escrow_token_account,
            client,
            mint,
            sender.pubkey(),
            0,
            None,
            None,
            0,
            None,
        );

        let acc_meta = incorrect_integer_arithmetic_3::accounts::InitVesting {
            sender: sender.pubkey(),
            sender_token_account,
            escrow,
            escrow_token_account,
            mint,
            token_program: anchor_spl::token::ID,
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);

        Ok((vec![sender], acc_meta))
    }
}
///IxOps implementation for `WithdrawUnlocked` with all required functions.
impl IxOps for WithdrawUnlocked {
    type IxData = incorrect_integer_arithmetic_3::instruction::WithdrawUnlocked;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        incorrect_integer_arithmetic_3::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = incorrect_integer_arithmetic_3::instruction::WithdrawUnlocked {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let recipient = fuzz_accounts.recipient.get_or_create_account(
            self.accounts.recipient,
            client,
            5 * LAMPORTS_PER_SOL,
        );

        // INFO use constant Account ID, so we will not generate multiple mints,
        // and also we can easily link to Initialize
        let mint =
            fuzz_accounts
                .mint
                .get_or_create_account(0, client, 6, &recipient.pubkey(), None);

        let recipient_token_account = fuzz_accounts.recipient_token_account.get_or_create_account(
            self.accounts.recipient_token_account,
            client,
            mint,
            recipient.pubkey(),
            0,
            None,
            None,
            0,
            None,
        );

        let escrow = fuzz_accounts.escrow.get_or_create_account(
            self.accounts.escrow,
            client,
            &[recipient.pubkey().as_ref(), b"ESCROW_SEED"],
            &incorrect_integer_arithmetic_3::ID,
        );

        let escrow_pda_authority = fuzz_accounts.escrow_pda_authority.get_or_create_account(
            self.accounts.escrow_pda_authority,
            client,
            &[b"ESCROW_PDA_AUTHORITY"],
            &incorrect_integer_arithmetic_3::ID,
        );

        let escrow_token_account = fuzz_accounts.escrow_token_account.get_or_create_account(
            self.accounts.escrow_token_account,
            client,
            mint,
            escrow_pda_authority,
            u64::MAX,
            None,
            None,
            0,
            None,
        );

        let acc_meta = incorrect_integer_arithmetic_3::accounts::WithdrawUnlocked {
            recipient: recipient.pubkey(),
            recipient_token_account,
            escrow,
            escrow_token_account,
            escrow_pda_authority,
            mint,
            token_program: anchor_spl::token::ID,
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);
        Ok((vec![recipient], acc_meta))
    }
    fn check(
        &self,
        pre_ix: &[SnapshotAccount],
        post_ix: &[SnapshotAccount],
        _ix_data: Self::IxData,
    ) -> Result<(), FuzzingError> {
        if let Ok(escrow) =
            incorrect_integer_arithmetic_3::Escrow::try_deserialize(&mut pre_ix[2].data())
        {
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

            if escrow.recipient == recipient.key() {
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
/// Use AccountsStorage<T> where T can be one of:
/// Keypair, PdaStore, TokenStore, MintStore, ProgramStore
#[derive(Default)]
pub struct FuzzAccounts {
    // No need to fuzz Token Program
    // token_program: AccountsStorage<ProgramStore>,
    sender_token_account: AccountsStorage<TokenStore>,
    escrow_token_account: AccountsStorage<TokenStore>,
    escrow_pda_authority: AccountsStorage<PdaStore>,
    sender: AccountsStorage<KeypairStore>,
    // No need to fuzz System Program
    // _system_program: AccountsStorage<ProgramStore>,
    recipient_token_account: AccountsStorage<TokenStore>,
    recipient: AccountsStorage<KeypairStore>,
    mint: AccountsStorage<MintStore>,
    escrow: AccountsStorage<PdaStore>,
}
