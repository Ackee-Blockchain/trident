use solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_client::fuzzing::*;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    InitializeFn(InitializeFn),
}
#[derive(Arbitrary, Debug)]
pub struct InitializeFn {
    pub accounts: InitializeFnAccounts,
    pub data: InitializeFnData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeFnAccounts {
    pub author: AccountId,
    pub hello_world_account: AccountId,
    pub _system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct InitializeFnData {
    pub input: u8,
}
///IxOps implementation for `InitializeFn` with all required functions.
impl IxOps for InitializeFn {
    type IxData = hello_world::instruction::InitializeFn;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        hello_world::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = hello_world::instruction::InitializeFn {
            input: self.data.input,
        };
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
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

        let hello_world_account = fuzz_accounts.hello_world_account.get_or_create_account(
            self.accounts.hello_world_account,
            client,
            &[b"hello_world_seed"],
            &hello_world::ID,
        );
        let signers = vec![author.clone()];
        let acc_meta = hello_world::accounts::InitializeContext {
            author: author.pubkey(),
            hello_world_account,
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
    fn check(
        &self,
        _pre_ix: &[SnapshotAccount],
        post_ix: &[SnapshotAccount],
        _ix_data: Self::IxData,
    ) -> Result<(), FuzzingError> {
        if let Ok(hello_world_account) =
            hello_world::StoreHelloWorld::try_deserialize(&mut post_ix[1].data())
        {
            if hello_world_account.input == 253 {
                return Err(FuzzingError::Custom(1));
            }
        }
        Ok(())
    }
}
/// Use AccountsStorage<T> where T can be one of:
/// Keypair, PdaStore, TokenStore, MintStore, ProgramStore
#[derive(Default)]
pub struct FuzzAccounts {
    author: AccountsStorage<Keypair>,
    hello_world_account: AccountsStorage<PdaStore>,
    // No need to fuzz system_program
    // system_program: AccountsStorage<todo!()>,
}
