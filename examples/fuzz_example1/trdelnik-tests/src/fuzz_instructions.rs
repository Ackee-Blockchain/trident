pub mod fuzz_example1_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trdelnik_client::{fuzzing::*, solana_sdk::native_token::LAMPORTS_PER_SOL};
    #[derive(Arbitrary, Clone, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
        Register(Register),
        EndRegistrations(EndRegistrations),
        Invest(Invest),
    }
    #[derive(Arbitrary, Clone)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitializeAccounts {
        pub author: AccountId,
        pub state: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitializeData {}
    #[derive(Arbitrary, Clone)]
    pub struct Register {
        pub accounts: RegisterAccounts,
        pub data: RegisterData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct RegisterAccounts {
        pub project_author: AccountId,
        pub project: AccountId,
        pub state: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct RegisterData {}
    #[derive(Arbitrary, Clone)]
    pub struct EndRegistrations {
        pub accounts: EndRegistrationsAccounts,
        pub data: EndRegistrationsData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct EndRegistrationsAccounts {
        pub author: AccountId,
        pub state: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct EndRegistrationsData {}
    #[derive(Arbitrary, Clone)]
    pub struct Invest {
        pub accounts: InvestAccounts,
        pub data: InvestData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InvestAccounts {
        pub investor: AccountId,
        pub project: AccountId,
        pub state: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InvestData {
        pub amount: u64,
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = fuzz_example1::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example1::instruction::Initialize {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let author = fuzz_accounts.author.get_or_create_account(
                self.accounts.author,
                client,
                100 * LAMPORTS_PER_SOL,
            );

            let state = fuzz_accounts
                .state
                .get_or_create_account(
                    self.accounts.state,
                    &[author.pubkey().as_ref(), b"state_seed"],
                    &program_client::fuzz_example1_instruction::PROGRAM_ID,
                )
                .ok_or(FuzzingError::CannotGetAccounts)?;

            let acc_meta = fuzz_example1::accounts::Initialize {
                author: author.pubkey(),
                state: state.pubkey(),
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);

            Ok((vec![author], acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Register {
        type IxData = fuzz_example1::instruction::Register;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = RegisterSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example1::instruction::Register {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = fuzz_example1::accounts::Register {
                project_author: todo!(),
                project: todo!(),
                state: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for EndRegistrations {
        type IxData = fuzz_example1::instruction::EndRegistrations;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = EndRegistrationsSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example1::instruction::EndRegistrations {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = fuzz_example1::accounts::EndRegistration {
                author: todo!(),
                state: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Invest {
        type IxData = fuzz_example1::instruction::Invest;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InvestSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example1::instruction::Invest { amount: todo!() };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = fuzz_example1::accounts::Invest {
                investor: todo!(),
                project: todo!(),
                state: todo!(),
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
        investor: AccountsStorage<Keypair>,
        project: AccountsStorage<PdaStore>,
        author: AccountsStorage<Keypair>,
        system_program: AccountsStorage<ProgramStore>,
        project_author: AccountsStorage<Keypair>,
        state: AccountsStorage<PdaStore>,
    }
    impl FuzzAccounts {
        pub fn new() -> Self {
            Default::default()
        }
    }
}
