use solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_client::fuzzing::*;

use hello_world::trident_fuzz_initialize_context_snapshot::InitializeContextAlias;

type InitializeFnSnapshot<'info> = InitializeContextAlias<'info>;
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
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
#[derive(Arbitrary, Debug)]
pub struct InitializeFnData {
    pub input: u8,
}
impl<'info> IxOps<'info> for InitializeFn {
    type IxData = hello_world::instruction::InitializeFn;
    type IxAccounts = FuzzAccounts;
    type IxSnapshot = InitializeFnSnapshot<'info>;
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        hello_world::ID
    }
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
        let acc_meta = hello_world::accounts::InitializeContext {
            author: author.pubkey(),
            hello_world_account: hello_world_account.pubkey(),
            system_program: solana_sdk::system_program::ID,
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
