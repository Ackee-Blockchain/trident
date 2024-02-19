pub mod fuzz_example0_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trdelnik_client::{fuzzing::*, solana_sdk::native_token::LAMPORTS_PER_SOL};
    #[derive(Arbitrary, Debug, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
        Update(Update),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        pub counter: AccountId,
        pub user: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {}
    #[derive(Arbitrary, Debug)]
    pub struct Update {
        pub accounts: UpdateAccounts,
        pub data: UpdateData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAccounts {
        pub counter: AccountId,
        pub authority: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateData {
        pub input1: u8,
        pub input2: u8,
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = fuzz_example0::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example0::instruction::Initialize {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let user = fuzz_accounts.user.get_or_create_account(
                self.accounts.user,
                client,
                5 * LAMPORTS_PER_SOL,
            );
            let counter = fuzz_accounts.counter.get_or_create_account(
                self.accounts.counter,
                client,
                5 * LAMPORTS_PER_SOL,
            );

            let acc_meta = fuzz_example0::accounts::Initialize {
                counter: counter.pubkey(),
                user: user.pubkey(),
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((vec![user, counter], acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Update {
        type IxData = fuzz_example0::instruction::Update;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdateSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example0::instruction::Update {
                input1: self.data.input1,
                input2: self.data.input2,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let user = fuzz_accounts.user.get_or_create_account(
                self.accounts.authority,
                client,
                15 * LAMPORTS_PER_SOL,
            );
            let counter = fuzz_accounts.counter.get_or_create_account(
                self.accounts.counter,
                client,
                5 * LAMPORTS_PER_SOL,
            );

            let acc_meta = fuzz_example0::accounts::Update {
                counter: counter.pubkey(),
                authority: user.pubkey(),
            }
            .to_account_metas(None);
            Ok((vec![user], acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        // INFO
        // The 'authority' and 'system_program' were automatically
        // generated in the FuzzAccounts struct, as they are both
        // used in the program. However, the 'authority' is in fact
        // the user account, just named differently. Therefore, we will use only
        // the generated user accounts for both 'user' and 'authority account' fields
        // in this fuzz test. Additionally, there is no need to fuzz the 'system_program' account.
        user: AccountsStorage<Keypair>,
        counter: AccountsStorage<Keypair>,
        // authority: AccountsStorage<Keypair>,
        // system_program: AccountsStorage<ProgramStore>,
    }
    impl FuzzAccounts {
        pub fn new() -> Self {
            Default::default()
        }
    }
}
