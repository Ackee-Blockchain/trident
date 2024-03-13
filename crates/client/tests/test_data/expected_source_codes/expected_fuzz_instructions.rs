pub mod fuzz_example3_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        InitVesting(InitVesting),
        WithdrawUnlocked(WithdrawUnlocked),
        WithdrawDummy(WithdrawDummy),
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
        pub _recipient: AccountId,
        pub amount: u64,
        pub start_at: u64,
        pub end_at: u64,
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
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawDummy {
        pub accounts: WithdrawDummyAccounts,
        pub data: WithdrawDummyData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawDummyAccounts {
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
    pub struct WithdrawDummyData {}
    impl<'info> IxOps<'info> for InitVesting {
        type IxData = fuzz_example3::instruction::InitVesting;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitVestingSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example3::instruction::InitVesting {
                recipient: todo!(),
                _recipient: todo!(),
                amount: todo!(),
                start_at: todo!(),
                end_at: todo!(),
                interval: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = fuzz_example3::accounts::InitVesting {
                sender: todo!(),
                sender_token_account: todo!(),
                escrow: todo!(),
                escrow_token_account: todo!(),
                mint: todo!(),
                token_program: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
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
            let signers = vec![todo!()];
            let acc_meta = fuzz_example3::accounts::Withdraw {
                recipient: todo!(),
                recipient_token_account: todo!(),
                escrow: todo!(),
                escrow_token_account: todo!(),
                escrow_pda_authority: todo!(),
                mint: todo!(),
                token_program: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for WithdrawDummy {
        type IxData = fuzz_example3::instruction::WithdrawDummy;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawDummySnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example3::instruction::WithdrawDummy {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = fuzz_example3::accounts::Withdraw {
                recipient: todo!(),
                recipient_token_account: todo!(),
                escrow: todo!(),
                escrow_token_account: todo!(),
                escrow_pda_authority: todo!(),
                mint: todo!(),
                token_program: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        escrow: AccountsStorage<todo!()>,
        escrow_pda_authority: AccountsStorage<todo!()>,
        escrow_token_account: AccountsStorage<todo!()>,
        mint: AccountsStorage<todo!()>,
        recipient: AccountsStorage<todo!()>,
        recipient_token_account: AccountsStorage<todo!()>,
        sender: AccountsStorage<todo!()>,
        sender_token_account: AccountsStorage<todo!()>,
        system_program: AccountsStorage<todo!()>,
        token_program: AccountsStorage<todo!()>,
    }
}
