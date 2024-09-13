use solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_client::fuzzing::*;

use cpi_metaplex_7::trident_fuzz_initialize_snapshot::InitializeAlias;

type InitializeSnapshot<'info> = InitializeAlias<'info>;
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
    pub signer: AccountId,
    pub mint: AccountId,
    pub _metadata_account: AccountId,
    pub _mpl_token_metadata: AccountId,
    pub _system_program: AccountId,
    pub _token_program: AccountId,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeData {
    pub input: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl<'info> IxOps<'info> for Initialize {
    type IxData = cpi_metaplex_7::instruction::Initialize;
    type IxAccounts = FuzzAccounts;
    type IxSnapshot = InitializeSnapshot<'info>;
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        cpi_metaplex_7::ID
    }
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = cpi_metaplex_7::instruction::Initialize {
            input: self.data.input,
            name: self.data.name.clone(),
            symbol: self.data.symbol.clone(),
            uri: self.data.uri.clone(),
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
            10 * LAMPORTS_PER_SOL,
        );

        let mint = fuzz_accounts.mint.get_or_create_account(
            self.accounts.mint,
            client,
            10 * LAMPORTS_PER_SOL,
        );

        let metadata_account = mpl_token_metadata::accounts::Metadata::find_pda(&mint.pubkey());

        let signers = vec![signer.clone(), mint.clone()];
        let acc_meta = cpi_metaplex_7::accounts::Initialize {
            signer: signer.pubkey(),
            mint: mint.pubkey(),
            metadata_account: metadata_account.0,
            mpl_token_metadata: mpl_token_metadata::ID,
            system_program: solana_sdk::system_program::ID,
            token_program: anchor_spl::token::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
#[doc = r" Use AccountsStorage<T> where T can be one of:"]
#[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
#[derive(Default)]
pub struct FuzzAccounts {
    _metadata_account: AccountsStorage<PdaStore>,
    mint: AccountsStorage<Keypair>,
    _mpl_token_metadata: AccountsStorage<ProgramStore>,
    signer: AccountsStorage<Keypair>,
    _system_program: AccountsStorage<ProgramStore>,
    _token_program: AccountsStorage<ProgramStore>,
}
