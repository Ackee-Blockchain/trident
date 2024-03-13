pub mod fuzz_example1_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use fuzz_example1::state::{PROJECT_SEED, STATE_SEED};
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
        Register(Register),
        EndRegistrations(EndRegistrations),
        Invest(Invest),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=2))]
        pub author: AccountId,
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=2))]
        pub state: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {}
    #[derive(Arbitrary, Debug)]
    pub struct Register {
        pub accounts: RegisterAccounts,
        pub data: RegisterData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RegisterAccounts {
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=2))]
        pub project_author: AccountId,
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=2))]
        pub project: AccountId,
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=2))]
        pub state: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RegisterData {}
    #[derive(Arbitrary, Debug)]
    pub struct EndRegistrations {
        pub accounts: EndRegistrationsAccounts,
        pub data: EndRegistrationsData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct EndRegistrationsAccounts {
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=2))]
        pub author: AccountId,
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=2))]
        pub state: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct EndRegistrationsData {}
    #[derive(Arbitrary, Debug)]
    pub struct Invest {
        pub accounts: InvestAccounts,
        pub data: InvestData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InvestAccounts {
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=2))]
        pub investor: AccountId,
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=2))]
        pub project: AccountId,
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=2))]
        pub state: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
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
                5000000000000,
            );
            let signers = vec![author.clone()];
            let state = fuzz_accounts
                .state
                .get_or_create_account(
                    self.accounts.state,
                    &[author.pubkey().as_ref(), STATE_SEED.as_ref()],
                    &fuzz_example1::ID,
                )
                .ok_or(FuzzingError::Custom(1))?
                .pubkey();

            let acc_meta = fuzz_example1::accounts::Initialize {
                author: author.pubkey(),
                state,
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
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
            let project_author = fuzz_accounts.project_author.get_or_create_account(
                self.accounts.project_author,
                client,
                5000000000000,
            );
            let signers = vec![project_author.clone()];
            let state = fuzz_accounts
                .state
                .get_or_create_account(
                    self.accounts.state,
                    &[project_author.pubkey().as_ref(), STATE_SEED.as_ref()],
                    &fuzz_example1::ID,
                )
                .ok_or(FuzzingError::Custom(2))?
                .pubkey();

            let project = fuzz_accounts
                .project
                .get_or_create_account(
                    self.accounts.project,
                    &[
                        project_author.pubkey().as_ref(),
                        state.as_ref(),
                        PROJECT_SEED.as_ref(),
                    ],
                    &fuzz_example1::ID,
                )
                .ok_or(FuzzingError::Custom(3))?
                .pubkey();

            let acc_meta = fuzz_example1::accounts::Register {
                project_author: project_author.pubkey(),
                project,
                state,
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
        fn check(
            &self,
            pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            _ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            // INFO
            // This fuzz check will reveal that registrations can be performed
            // even though registration windows is not open.
            let state = pre_ix.state;
            if let Some(_project) = post_ix.project {
                let registrations_round = state.registrations_round;
                if !registrations_round {
                    return Err(FuzzingError::Custom(11));
                }
            }
            Ok(())
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
            let author = fuzz_accounts.author.get_or_create_account(
                self.accounts.author,
                client,
                5000000000000,
            );
            let signers = vec![author.clone()];
            let state = fuzz_accounts
                .state
                .get_or_create_account(
                    self.accounts.state,
                    &[author.pubkey().as_ref(), STATE_SEED.as_ref()],
                    &fuzz_example1::ID,
                )
                .ok_or(FuzzingError::Custom(4))?
                .pubkey();
            let acc_meta = fuzz_example1::accounts::EndRegistration {
                author: author.pubkey(),
                state,
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
            let data = fuzz_example1::instruction::Invest {
                amount: self.data.amount,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let investor = fuzz_accounts.investor.get_or_create_account(
                self.accounts.investor,
                client,
                5000000000000,
            );
            let signers = vec![investor.clone()];

            let project_author = fuzz_accounts.project_author.get_or_create_account(
                self.accounts.project,
                client,
                50000000000,
            );
            let state = fuzz_accounts
                .state
                .get_or_create_account(
                    self.accounts.state,
                    &[project_author.pubkey().as_ref(), STATE_SEED.as_ref()],
                    &fuzz_example1::ID,
                )
                .ok_or(FuzzingError::Custom(5))?
                .pubkey();

            let project = fuzz_accounts
                .project
                .get_or_create_account(
                    self.accounts.project,
                    &[
                        project_author.pubkey().as_ref(),
                        state.as_ref(),
                        PROJECT_SEED.as_ref(),
                    ],
                    &fuzz_example1::ID,
                )
                .ok_or(FuzzingError::Custom(6))?
                .pubkey();
            let acc_meta = fuzz_example1::accounts::Invest {
                investor: investor.pubkey(),
                project,
                state,
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        // INFO
        // There is no need to fuzz the 'system_program' account.
        project_author: AccountsStorage<Keypair>,
        author: AccountsStorage<Keypair>,
        project: AccountsStorage<PdaStore>,
        // system_program: AccountsStorage<ProgramStore>,
        investor: AccountsStorage<Keypair>,
        state: AccountsStorage<PdaStore>,
    }
    impl FuzzAccounts {
        pub fn new() -> Self {
            Default::default()
        }
    }
}
