pub mod fuzz_example3_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trdelnik_client::{fuzzing::*, solana_sdk::native_token::LAMPORTS_PER_SOL};
    #[derive(Arbitrary, Clone, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        InitVesting(InitVesting),
        WithdrawUnlocked(WithdrawUnlocked),
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitVesting {
        pub accounts: InitVestingAccounts,
        pub data: InitVestingData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitVestingAccounts {
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub sender: AccountId,
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub sender_token_account: AccountId,
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub escrow: AccountId,
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub escrow_token_account: AccountId,
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub mint: AccountId,
        pub token_program: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitVestingData {
        pub recipient: AccountId,
        // the range is selected randomly
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(13581..=580743))]
        pub amount: u64,
        // we want start_at smaller than end_at
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1_000_000))]
        pub start_at: u64,
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(1_001_001..=1_050_000))]
        pub end_at: u64,
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(1..=1000))]
        pub interval: u64,
    }
    #[derive(Arbitrary, Clone)]
    pub struct WithdrawUnlocked {
        pub accounts: WithdrawUnlockedAccounts,
        pub data: WithdrawUnlockedData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct WithdrawUnlockedAccounts {
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub recipient: AccountId,
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub recipient_token_account: AccountId,
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub escrow: AccountId,
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub escrow_token_account: AccountId,
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub escrow_pda_authority: AccountId,
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1))]
        pub mint: AccountId,
        pub token_program: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct WithdrawUnlockedData {}
    impl<'info> IxOps<'info> for InitVesting {
        type IxData = fuzz_example3::instruction::InitVesting;
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
            let data = fuzz_example3::instruction::InitVesting {
                recipient: recipient.pubkey(),
                amount: self.data.amount,
                start_at: self.data.start_at,
                end_at: self.data.end_at,
                interval: self.data.interval,
            };
            // let data = fuzz_example3::instruction::InitVesting {
            //     recipient: recipient.pubkey(),
            //     amount: 11_111_111,
            //     start_at: 1_000_000 - 200_000,
            //     end_at: 1_000_000,
            //     interval: 10,
            // };
            // whole amount cannot be withdrawn
            // const amount = new BN(11111111);
            // const start = now.subn(200000);
            // const end = now;
            // const interval = new BN(10);

            // // Bug to be found
            // const amount = new BN(200);
            // const start = now.subn(10);
            // const end = now;
            // const interval = new BN(5);

            // works
            // const amount = new BN(2001000); // amount to vest
            // const start = now.subn(10000); // start vesting in the past so that we do not need to wait
            // const end = now; // end now so that we do not need to wait to withdraw whole vested amount
            // const interval = new BN(5); // unlock new amount every X seconds
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
                1000 * LAMPORTS_PER_SOL,
            );

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
                    &fuzz_example3::ID,
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

            let acc_meta = fuzz_example3::accounts::InitVesting {
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
        type IxData = fuzz_example3::instruction::WithdrawUnlocked;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawUnlockedSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example3::instruction::WithdrawUnlocked {};
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
                1000 * LAMPORTS_PER_SOL,
            );

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
                    &fuzz_example3::ID,
                )
                .unwrap();

            let escrow_pda_authority = fuzz_accounts
                .escrow_pda_authority
                .get_or_create_account(
                    self.accounts.escrow_pda_authority,
                    &[b"ESCROW_PDA_AUTHORITY"],
                    &fuzz_example3::ID,
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

            let acc_meta = fuzz_example3::accounts::WithdrawUnlocked {
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
        ) -> Result<(), &'static str> {
            if let Some(escrow) = pre_ix.escrow {
                let recipient = pre_ix.recipient.unwrap();
                if escrow.recipient != *recipient.key {
                    return Ok(());
                } else if let Some(recepient_token_account_pre) = pre_ix.recipient_token_account {
                    if let Some(recepient_token_account_post) = post_ix.recipient_token_account {
                        if recepient_token_account_pre.amount + escrow.amount
                            != recepient_token_account_post.amount
                        {
                            if recepient_token_account_pre.amount + escrow.amount
                                >= recepient_token_account_post.amount
                            {
                                eprintln!(
                                    "Amount Mismatch: {}",
                                    (recepient_token_account_pre.amount + escrow.amount)
                                        - recepient_token_account_post.amount
                                );
                                eprintln!("Before: {}", recepient_token_account_pre.amount);
                                eprintln!("After: {}", recepient_token_account_post.amount);
                                eprintln!(
                                    "Expected: {}",
                                    recepient_token_account_pre.amount + escrow.amount
                                );
                            } else {
                                eprintln!(
                                    "Amount Mismatch: {}",
                                    recepient_token_account_post.amount
                                        - (recepient_token_account_pre.amount + escrow.amount)
                                );
                                eprintln!("Before: {}", recepient_token_account_pre.amount);
                                eprintln!("After: {}", recepient_token_account_post.amount);
                                eprintln!(
                                    "Expected: {}",
                                    recepient_token_account_pre.amount + escrow.amount
                                );
                            }

                            return Err("Transfered amount mismatch");
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
        _token_program: AccountsStorage<ProgramStore>,
        sender_token_account: AccountsStorage<TokenStore>,
        escrow_token_account: AccountsStorage<TokenStore>,
        escrow_pda_authority: AccountsStorage<PdaStore>,
        sender: AccountsStorage<Keypair>,
        _system_program: AccountsStorage<ProgramStore>,
        recipient_token_account: AccountsStorage<TokenStore>,
        recipient: AccountsStorage<Keypair>,
        mint: AccountsStorage<MintStore>,
        escrow: AccountsStorage<PdaStore>,
    }
    impl FuzzAccounts {
        pub fn new() -> Self {
            Default::default()
        }
    }
}
