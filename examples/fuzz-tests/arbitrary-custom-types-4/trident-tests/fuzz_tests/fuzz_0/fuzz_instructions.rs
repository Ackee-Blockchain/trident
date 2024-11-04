use solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_client::fuzzing::*;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    Initialize(Initialize),
    Update(Update),
}
#[derive(Arbitrary, Debug)]
pub struct Initialize {
    pub accounts: InitializeAccounts,
    pub _data: InitializeData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeAccounts {
    pub counter: AccountId,
    pub user: AccountId,
    pub _system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
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
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct UpdateData {
    pub input: InputUpdatePrametersTrident,
    pub variant: InputUpdateVariantTrident,
}
///IxOps implementation for `Initialize` with all required functions.
impl IxOps for Initialize {
    type IxData = arbitrary_custom_types_4::instruction::Initialize;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        arbitrary_custom_types_4::ID
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
        let data = arbitrary_custom_types_4::instruction::Initialize {};
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
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);
        Ok((vec![user, counter], acc_meta))
    }
}
///IxOps implementation for `Update` with all required functions.
impl IxOps for Update {
    type IxData = arbitrary_custom_types_4::instruction::Update;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        arbitrary_custom_types_4::ID
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
        let input = self.data.input.into();
        let variant = self.data.variant.into();

        let data = arbitrary_custom_types_4::instruction::Update { input, variant };
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
/// Use AccountsStorage<T> where T can be one of:
/// Keypair, PdaStore, TokenStore, MintStore, ProgramStore
#[derive(Default)]
pub struct FuzzAccounts {
    user: AccountsStorage<Keypair>,
    counter: AccountsStorage<Keypair>,
    // _authority: AccountsStorage<todo!()>,
    // _system_program: AccountsStorage<todo!()>,
}

// -------------------------------------------------------------------
// -------------------------------------------------------------------
// -------------------------------------------------------------------
// Use arbitrary section
#[derive(Arbitrary, Debug, Clone, Copy)]
pub struct InputUpdatePrametersTrident {
    pub input1: u8,
    pub input2: u8,
}

#[derive(Arbitrary, Debug, Clone, Copy)]
pub enum InputUpdateVariantTrident {
    UpdateVariant1,
    UpdateVariant2,
}

impl std::convert::From<InputUpdatePrametersTrident>
    for arbitrary_custom_types_4::InputUpdatePrameters
{
    fn from(val: InputUpdatePrametersTrident) -> Self {
        arbitrary_custom_types_4::InputUpdatePrameters {
            input1: val.input1,
            input2: val.input2,
        }
    }
}
impl std::convert::From<InputUpdateVariantTrident>
    for arbitrary_custom_types_4::InputUpdateVariant
{
    fn from(val: InputUpdateVariantTrident) -> Self {
        match val {
            InputUpdateVariantTrident::UpdateVariant1 => {
                arbitrary_custom_types_4::InputUpdateVariant::UpdateVariant1
            }
            InputUpdateVariantTrident::UpdateVariant2 => {
                arbitrary_custom_types_4::InputUpdateVariant::UpdateVariant2
            }
        }
    }
}
