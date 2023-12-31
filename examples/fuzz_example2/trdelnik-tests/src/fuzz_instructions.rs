pub mod fuzz_example2_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trdelnik_client::fuzzing::*;
    #[derive(Arbitrary, Clone, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
        Withdraw(Withdraw),
    }
    #[derive(Arbitrary, Clone)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitializeAccounts {
        pub author: AccountId,
        pub escrow: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitializeData {
        pub receiver: Pubkey,
        pub amount: u64,
    }
    #[derive(Arbitrary, Clone)]
    pub struct Withdraw {
        pub accounts: WithdrawAccounts,
        pub data: WithdrawData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct WithdrawAccounts {
        pub receiver: AccountId,
        pub escrow: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct WithdrawData {}
    impl<'info> IxOps<'info> for Initialize {
        type IxData = fuzz_example2::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example2::instruction::Initialize {
                receiver: todo!(),
                amount: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = fuzz_example2::accounts::Initialize {
                author: todo!(),
                escrow: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Withdraw {
        type IxData = fuzz_example2::instruction::Withdraw;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example2::instruction::Withdraw {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = fuzz_example2::accounts::Withdraw {
                receiver: todo!(),
                escrow: todo!(),
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
        receiver: AccountsStorage<Keypair>,
        system_program: AccountsStorage<ProgramStore>,
        author: AccountsStorage<Keypair>,
        escrow: AccountsStorage<PdaStore>,
    }
    impl FuzzAccounts {
        pub fn new() -> Self {
            Default::default()
        }
    }
}
