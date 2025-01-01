use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
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
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct InitializeFnData {
    pub input: u8,
}
///IxOps implementation for `InitializeFn` with all required functions.
impl IxOps for InitializeFn {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![18u8, 187u8, 169u8, 213u8, 94u8, 180u8, 86u8, 152u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data.input).unwrap());
        }
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let mut signers = vec![];
        {
            let author = fuzz_accounts.author_hello_world.get_or_create_account(
                self.accounts.author,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new(author.pubkey(), true));
            signers.push(author.insecure_clone());
        }
        {
            let hello_world_account = fuzz_accounts
                .hello_world_account_hello_world
                .get_or_create_account(
                    self.accounts.hello_world_account,
                    client,
                    &[b"hello_world_seed"],
                    &self.get_program_id(),
                );
            account_metas.push(AccountMeta::new(hello_world_account, false));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        Ok((signers, account_metas))
    }
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    author_hello_world: AccountsStorage<KeypairStore>,
    hello_world_account_hello_world: AccountsStorage<PdaStore>,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct StoreHelloWorld {
    input: u8,
}
