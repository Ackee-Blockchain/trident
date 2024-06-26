pub mod hello_world_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::system_program::ID as SYSTEM_PROGRAM_ID;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        pub author: AccountId,
        pub hello_world_account: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {
        pub input: u8,
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = hello_world::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = hello_world::instruction::Initialize {
                input: self.data.input,
            };
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
                5 * LAMPORTS_PER_SOL,
            );

            let hello_world_account = fuzz_accounts
                .hello_world_account
                .get_or_create_account(
                    self.accounts.hello_world_account,
                    &[b"hello_world_seed"],
                    &hello_world::ID,
                )
                .unwrap();
            let signers = vec![author.clone()];
            let acc_meta = hello_world::accounts::Initialize {
                author: author.pubkey(),
                hello_world_account: hello_world_account.pubkey(),
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
        fn check(
            &self,
            _pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            _ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            if let Some(hello_world_account) = post_ix.hello_world_account {
                if hello_world_account.input == 253 {
                    return Err(FuzzingError::Custom(1));
                }
            }
            Ok(())
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        author: AccountsStorage<Keypair>,
        hello_world_account: AccountsStorage<PdaStore>,
        // No need to fuzz system_program
        // system_program: AccountsStorage<todo!()>,
    }
}
