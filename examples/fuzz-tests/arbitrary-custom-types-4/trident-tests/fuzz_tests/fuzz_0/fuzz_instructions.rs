pub mod arbitrary_custom_types_4_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::system_program::ID as SYSTEM_PROGRAM_ID;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
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
        pub input: InputUpdatePrameters,
        pub variant: InputUpdateVariant,
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = arbitrary_custom_types_4::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = arbitrary_custom_types_4::instruction::Initialize {};
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

            let acc_meta = arbitrary_custom_types_4::accounts::Initialize {
                counter: counter.pubkey(),
                user: user.pubkey(),
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((vec![user, counter], acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Update {
        type IxData = arbitrary_custom_types_4::instruction::Update;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdateSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let input = self.data.input.into();
            let variant = self.data.variant.into();

            let data = arbitrary_custom_types_4::instruction::Update { input, variant };
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

            let acc_meta = arbitrary_custom_types_4::accounts::Update {
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
        user: AccountsStorage<Keypair>,
        counter: AccountsStorage<Keypair>,
    }
    // -------------------------------------------------------------------
    // -------------------------------------------------------------------
    // -------------------------------------------------------------------
    // Use arbitrary section
    #[derive(Arbitrary, Debug, Clone, Copy)]
    pub struct InputUpdatePrameters {
        pub input1: u8,
        pub input2: u8,
    }

    #[derive(Arbitrary, Debug, Clone, Copy)]
    pub enum InputUpdateVariant {
        UpdateVariant1,
        UpdateVariant2,
    }

    impl std::convert::From<InputUpdatePrameters> for arbitrary_custom_types_4::InputUpdatePrameters {
        fn from(val: InputUpdatePrameters) -> Self {
            arbitrary_custom_types_4::InputUpdatePrameters {
                input1: val.input1,
                input2: val.input2,
            }
        }
    }
    impl std::convert::From<InputUpdateVariant> for arbitrary_custom_types_4::InputUpdateVariant {
        fn from(val: InputUpdateVariant) -> Self {
            match val {
                InputUpdateVariant::UpdateVariant1 => {
                    arbitrary_custom_types_4::InputUpdateVariant::UpdateVariant1
                }
                InputUpdateVariant::UpdateVariant2 => {
                    arbitrary_custom_types_4::InputUpdateVariant::UpdateVariant2
                }
            }
        }
    }
}
