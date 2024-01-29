pub mod fuzz_example3_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trdelnik_client::{fuzzing::*, solana_sdk::native_token::LAMPORTS_PER_SOL};
    pub enum FuzzInstruction {
        InitVesting(InitVesting),
        WithdrawUnlocked(WithdrawUnlocked),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FuzzInstruction {
        #[inline]
        fn clone(&self) -> FuzzInstruction {
            match self {
                FuzzInstruction::InitVesting(__self_0) => {
                    FuzzInstruction::InitVesting(::core::clone::Clone::clone(__self_0))
                }
                FuzzInstruction::WithdrawUnlocked(__self_0) => {
                    FuzzInstruction::WithdrawUnlocked(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
            }
        }
    }
    pub struct InitVesting {
        pub accounts: InitVestingAccounts,
        pub data: InitVestingData,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InitVesting {
        #[inline]
        fn clone(&self) -> InitVesting {
            InitVesting {
                accounts: ::core::clone::Clone::clone(&self.accounts),
                data: ::core::clone::Clone::clone(&self.data),
            }
        }
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
    #[automatically_derived]
    impl ::core::clone::Clone for InitVestingAccounts {
        #[inline]
        fn clone(&self) -> InitVestingAccounts {
            InitVestingAccounts {
                sender: ::core::clone::Clone::clone(&self.sender),
                sender_token_account: ::core::clone::Clone::clone(
                    &self.sender_token_account,
                ),
                escrow: ::core::clone::Clone::clone(&self.escrow),
                escrow_token_account: ::core::clone::Clone::clone(
                    &self.escrow_token_account,
                ),
                mint: ::core::clone::Clone::clone(&self.mint),
                token_program: ::core::clone::Clone::clone(&self.token_program),
                system_program: ::core::clone::Clone::clone(&self.system_program),
            }
        }
    }
    pub struct InitVestingData {
        pub recipient: AccountId,
        pub amount: u64,
        #[arbitrary(
            with = |u:&mut
            arbitrary::Unstructured|u.int_in_range(0..= 1_000_000)
        )]
        pub start_at: u64,
        #[arbitrary(
            with = |u:&mut
            arbitrary::Unstructured|u.int_in_range(1_001_001..= 1_050_000)
        )]
        pub end_at: u64,
        #[arbitrary(with = |u:&mut arbitrary::Unstructured|u.int_in_range(1..= 1000))]
        pub interval: u64,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InitVestingData {
        #[inline]
        fn clone(&self) -> InitVestingData {
            InitVestingData {
                recipient: ::core::clone::Clone::clone(&self.recipient),
                amount: ::core::clone::Clone::clone(&self.amount),
                start_at: ::core::clone::Clone::clone(&self.start_at),
                end_at: ::core::clone::Clone::clone(&self.end_at),
                interval: ::core::clone::Clone::clone(&self.interval),
            }
        }
    }
    pub struct WithdrawUnlocked {
        pub accounts: WithdrawUnlockedAccounts,
        pub data: WithdrawUnlockedData,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for WithdrawUnlocked {
        #[inline]
        fn clone(&self) -> WithdrawUnlocked {
            WithdrawUnlocked {
                accounts: ::core::clone::Clone::clone(&self.accounts),
                data: ::core::clone::Clone::clone(&self.data),
            }
        }
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
    #[automatically_derived]
    impl ::core::clone::Clone for WithdrawUnlockedAccounts {
        #[inline]
        fn clone(&self) -> WithdrawUnlockedAccounts {
            WithdrawUnlockedAccounts {
                recipient: ::core::clone::Clone::clone(&self.recipient),
                recipient_token_account: ::core::clone::Clone::clone(
                    &self.recipient_token_account,
                ),
                escrow: ::core::clone::Clone::clone(&self.escrow),
                escrow_token_account: ::core::clone::Clone::clone(
                    &self.escrow_token_account,
                ),
                escrow_pda_authority: ::core::clone::Clone::clone(
                    &self.escrow_pda_authority,
                ),
                mint: ::core::clone::Clone::clone(&self.mint),
                token_program: ::core::clone::Clone::clone(&self.token_program),
                system_program: ::core::clone::Clone::clone(&self.system_program),
            }
        }
    }
    pub struct WithdrawUnlockedData {}
    #[automatically_derived]
    impl ::core::clone::Clone for WithdrawUnlockedData {
        #[inline]
        fn clone(&self) -> WithdrawUnlockedData {
            WithdrawUnlockedData {}
        }
    }
    impl<'info> IxOps<'info> for InitVesting {
        type IxData = fuzz_example3::instruction::InitVesting;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitVestingSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let recipient = fuzz_accounts
                .recipient
                .get_or_create_account(
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
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let sender = fuzz_accounts
                .sender
                .get_or_create_account(
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
            let recipient = fuzz_accounts
                .recipient
                .get_or_create_account(
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
            Ok((
                <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([sender])),
                acc_meta,
            ))
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
            let data = fuzz_example3::instruction::WithdrawUnlocked {
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let recipient = fuzz_accounts
                .recipient
                .get_or_create_account(
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
            Ok((
                <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([recipient])),
                acc_meta,
            ))
        }
        fn check(
            &self,
            pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            _ix_data: Self::IxData,
        ) -> Result<(), &'static str> {
            if let Some(escrow) = pre_ix.escrow {
                let recipient = pre_ix.recipient.unwrap();
                if let Some(recipient_token_account_pre) = pre_ix.recipient_token_account
                {
                    if let Some(recipient_token_account_post)
                        = post_ix.recipient_token_account
                    {
                        if escrow.recipient == *recipient.key {
                            if recipient_token_account_pre.amount
                                == recipient_token_account_post.amount
                            {
                                return Err("Recipient was not able to withdraw any funds");
                            } else if recipient_token_account_pre.amount + escrow.amount
                                != recipient_token_account_post.amount
                            {
                                if recipient_token_account_pre.amount + escrow.amount
                                    > recipient_token_account_post.amount
                                {
                                    return Err("Recipient withdrew LESS");
                                } else {
                                    return Err("Recipient withdrew MORE");
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }
    }
    /// Use AccountsStorage<T> where T can be one of:
    /// Keypair, PdaStore, TokenStore, MintStore, ProgramStore
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
    #[automatically_derived]
    impl ::core::default::Default for FuzzAccounts {
        #[inline]
        fn default() -> FuzzAccounts {
            FuzzAccounts {
                _token_program: ::core::default::Default::default(),
                sender_token_account: ::core::default::Default::default(),
                escrow_token_account: ::core::default::Default::default(),
                escrow_pda_authority: ::core::default::Default::default(),
                sender: ::core::default::Default::default(),
                _system_program: ::core::default::Default::default(),
                recipient_token_account: ::core::default::Default::default(),
                recipient: ::core::default::Default::default(),
                mint: ::core::default::Default::default(),
                escrow: ::core::default::Default::default(),
            }
        }
    }
    impl FuzzAccounts {
        pub fn new() -> Self {
            Default::default()
        }
    }
}
