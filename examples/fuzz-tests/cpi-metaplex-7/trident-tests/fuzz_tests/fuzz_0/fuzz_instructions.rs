use solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_client::fuzzing::*;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
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
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct InitializeData {
    pub input: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
///IxOps implementation for `Initialize` with all required functions.
impl IxOps for Initialize {
    type IxData = cpi_metaplex_7::instruction::Initialize;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        cpi_metaplex_7::ID
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
        let data = cpi_metaplex_7::instruction::Initialize {
            input: self.data.input,
            name: self.data.name.clone(),
            symbol: self.data.symbol.clone(),
            uri: self.data.uri.clone(),
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
/// Use AccountsStorage<T> where T can be one of:
/// Keypair, PdaStore, TokenStore, MintStore, ProgramStore
#[derive(Default)]
pub struct FuzzAccounts {
    _metadata_account: AccountsStorage<PdaStore>,
    mint: AccountsStorage<KeypairStore>,
    _mpl_token_metadata: AccountsStorage<ProgramStore>,
    signer: AccountsStorage<KeypairStore>,
    _system_program: AccountsStorage<ProgramStore>,
    _token_program: AccountsStorage<ProgramStore>,
}
