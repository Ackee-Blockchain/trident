// pub mod callee_fuzz_instructions {
//     use crate::accounts_snapshots::*;
//     use trident_client::fuzzing::*;
//     #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
//     pub enum FuzzInstruction {
//         InitializeCallee(InitializeCallee),
//     }
//     #[derive(Arbitrary, Debug)]
//     pub struct InitializeCallee {
//         pub accounts: InitializeCalleeAccounts,
//         pub data: InitializeCalleeData,
//     }
//     #[derive(Arbitrary, Debug)]
//     pub struct InitializeCalleeAccounts {
//         pub signer: AccountId,
//     }
//     #[derive(Arbitrary, Debug)]
//     pub struct InitializeCalleeData {
//         pub input: u8,
//     }
//     impl<'info> IxOps<'info> for InitializeCallee {
//         type IxData = callee::instruction::InitializeCallee;
//         type IxAccounts = FuzzAccounts;
//         type IxSnapshot = InitializeCalleeSnapshot<'info>;
//         fn get_data(
//             &self,
//             _client: &mut impl FuzzClient,
//             _fuzz_accounts: &mut FuzzAccounts,
//         ) -> Result<Self::IxData, FuzzingError> {
//             let data = callee::instruction::InitializeCallee { input: todo!() };
//             Ok(data)
//         }
//         fn get_accounts(
//             &self,
//             client: &mut impl FuzzClient,
//             fuzz_accounts: &mut FuzzAccounts,
//         ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
//             let signers = vec![todo!()];
//             let acc_meta =
//                 callee::accounts::InitializeCallee { signer: todo!() }.to_account_metas(None);
//             Ok((signers, acc_meta))
//         }
//     }
//     #[doc = r" Use AccountsStorage<T> where T can be one of:"]
//     #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
//     #[derive(Default)]
//     pub struct FuzzAccounts {
//         signer: AccountsStorage<todo!()>,
//     }
// }

pub mod caller_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        InitializeCaller(InitializeCaller),
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeCaller {
        pub accounts: InitializeCallerAccounts,
        pub data: InitializeCallerData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeCallerAccounts {
        pub signer: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeCallerData {
        pub input: u8,
    }
    impl<'info> IxOps<'info> for InitializeCaller {
        type IxData = caller::instruction::InitializeCaller;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeCallerSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = caller::instruction::InitializeCaller {
                input: self.data.input,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                5 * solana_sdk::native_token::LAMPORTS_PER_SOL,
            );
            let signers = vec![signer.clone()];
            let acc_meta = caller::accounts::InitializeCaller {
                signer: signer.pubkey(),
                program: callee::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        _program: AccountsStorage<ProgramStore>,
        signer: AccountsStorage<Keypair>,
    }
}
