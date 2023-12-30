pub mod fuzzer_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trdelnik_client::fuzzing::*;
    #[derive(Arbitrary, Clone, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
        Update(Update),
    }
    #[derive(Arbitrary, Clone)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitializeAccounts {
        pub counter: AccountId,
        pub user: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitializeData {}
    #[derive(Arbitrary, Clone)]
    pub struct Update {
        pub accounts: UpdateAccounts,
        pub data: UpdateData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct UpdateAccounts {
        pub counter: AccountId,
        pub authority: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct UpdateData {
        pub input1: u8,
        pub input2: u8,
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = fuzzer::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzzer::instruction::Initialize {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = fuzzer::accounts::Initialize {
                counter: todo!(),
                user: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Update {
        type IxData = fuzzer::instruction::Update;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdateSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzzer::instruction::Update {
                input1: todo!(),
                input2: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = fuzzer::accounts::Update {
                counter: todo!(),
                authority: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        user: AccountsStorage<Keypair>,
        counter: AccountsStorage<Keypair>,
        authority: AccountsStorage<Keypair>,
        system_program: AccountsStorage<ProgramStore>,
    }
    impl FuzzAccounts {
        pub fn new() -> Self {
            Default::default()
        }
    }
}
