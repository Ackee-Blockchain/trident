pub mod incorrect_integer_arithmetic_3_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::system_program::ID as SYSTEM_PROGRAM_ID;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
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
        pub token_program: AccountId,
        pub system_program: AccountId,
    }
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
        pub token_program: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawUnlockedData {}
    impl<'info> IxOps<'info> for InitVesting {
        type IxData = incorrect_integer_arithmetic_3::instruction::InitVesting;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitVestingSnapshot<'info>;
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
                .get_or_create_account(0, client, 6, &sender.pubkey(), None)
                .unwrap();

            let sender_token_account = fuzz_accounts
                .sender_token_account
                .get_or_create_account(
                    self.accounts.sender_token_account,
                    client,
                    mint,
                    sender.pubkey(),
                    u64::MAX,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let recipient = fuzz_accounts.recipient.get_or_create_account(
                self.data.recipient,
                client,
                10 * LAMPORTS_PER_SOL,
            );
            let escrow = fuzz_accounts
                .escrow
                .get_or_create_account(
                    self.accounts.escrow,
                    &[recipient.pubkey().as_ref(), b"ESCROW_SEED"],
                    &incorrect_integer_arithmetic_3::ID,
                )
                .unwrap();

            let escrow_token_account = fuzz_accounts
                .escrow_token_account
                .get_or_create_account(
                    self.accounts.escrow_token_account,
                    client,
                    mint,
                    sender.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let acc_meta = incorrect_integer_arithmetic_3::accounts::InitVesting {
                sender: sender.pubkey(),
                sender_token_account,
                escrow: escrow.pubkey(),
                escrow_token_account,
                mint,
                token_program: anchor_spl::token::ID,
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);

            Ok((vec![sender], acc_meta))
        }
    }
    impl<'info> IxOps<'info> for WithdrawUnlocked {
        type IxData = incorrect_integer_arithmetic_3::instruction::WithdrawUnlocked;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawUnlockedSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = incorrect_integer_arithmetic_3::instruction::WithdrawUnlocked {};
            Ok(data)
        }
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
            let mint = fuzz_accounts
                .mint
                .get_or_create_account(0, client, 6, &recipient.pubkey(), None)
                .unwrap();

            let recipient_token_account = fuzz_accounts
                .recipient_token_account
                .get_or_create_account(
                    self.accounts.recipient_token_account,
                    client,
                    mint,
                    recipient.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let escrow = fuzz_accounts
                .escrow
                .get_or_create_account(
                    self.accounts.escrow,
                    &[recipient.pubkey().as_ref(), b"ESCROW_SEED"],
                    &incorrect_integer_arithmetic_3::ID,
                )
                .unwrap();

            let escrow_pda_authority = fuzz_accounts
                .escrow_pda_authority
                .get_or_create_account(
                    self.accounts.escrow_pda_authority,
                    &[b"ESCROW_PDA_AUTHORITY"],
                    &incorrect_integer_arithmetic_3::ID,
                )
                .unwrap();

            let escrow_token_account = fuzz_accounts
                .escrow_token_account
                .get_or_create_account(
                    self.accounts.escrow_token_account,
                    client,
                    mint,
                    escrow_pda_authority.pubkey(),
                    u64::MAX,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let acc_meta = incorrect_integer_arithmetic_3::accounts::WithdrawUnlocked {
                recipient: recipient.pubkey(),
                recipient_token_account,
                escrow: escrow.pubkey(),
                escrow_token_account,
                escrow_pda_authority: escrow_pda_authority.pubkey(),
                mint,
                token_program: anchor_spl::token::ID,
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((vec![recipient], acc_meta))
        }
        fn check(
            &self,
            pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            _ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            if let Some(escrow) = pre_ix.escrow {
                let recipient = pre_ix.recipient;
                let recipient_token_account_pre = pre_ix.recipient_token_account;
                let recipient_token_account_post = post_ix.recipient_token_account;
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
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        // No need to fuzz Token Program
        // token_program: AccountsStorage<ProgramStore>,
        sender_token_account: AccountsStorage<TokenStore>,
        escrow_token_account: AccountsStorage<TokenStore>,
        escrow_pda_authority: AccountsStorage<PdaStore>,
        sender: AccountsStorage<Keypair>,
        // No need to fuzz System Program
        // _system_program: AccountsStorage<ProgramStore>,
        recipient_token_account: AccountsStorage<TokenStore>,
        recipient: AccountsStorage<Keypair>,
        mint: AccountsStorage<MintStore>,
        escrow: AccountsStorage<PdaStore>,
    }
}
