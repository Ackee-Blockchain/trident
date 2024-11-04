use trident_client::fuzzing::*;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    InitializeIxDummy2(InitializeIxDummy2),
    InitializeIxDummyExample(InitializeIxDummyExample),
}
#[derive(Arbitrary, Debug)]
pub struct InitializeIxDummy2 {
    pub accounts: InitializeIxDummy2Accounts,
    pub data: InitializeIxDummy2Data,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeIxDummy2Accounts {
    pub signer: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct InitializeIxDummy2Data {
    pub _var1: bool,
    pub _var2: u8,
    pub _var3: i8,
    pub _var4: u16,
    pub _var5: i16,
    pub _var6: u32,
    pub _var7: i32,
    pub _var8: u64,
    pub _var9: i32,
    pub _var10: f64,
    pub _var11: u128,
    pub _var12: i128,
    pub _ver13: Vec<u8>,
    pub _var14: String,
    pub _var15: AccountId,
    pub _var16: Option<i16>,
    pub _var17: Vec<u32>,
    pub _var18: [i128; 5usize],
    pub _var19: InputParameter,
    pub _var20: Vec<Vec<Vec<Vec<Vec<u8>>>>>,
    pub _var21: Vec<Vec<Vec<Vec<Option<u8>>>>>,
    pub _var22: Vec<Vec<Vec<Vec<Option<Vec<u8>>>>>>,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeIxDummyExample {
    pub accounts: InitializeIxDummyExampleAccounts,
    pub data: InitializeIxDummyExampleData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeIxDummyExampleAccounts {
    pub account: AccountId,
    pub account_info: AccountId,
    pub account_loader: AccountId,
    pub boxed: AccountId,
    pub interace: AccountId,
    pub interface_account: AccountId,
    pub option: AccountId,
    pub program: AccountId,
    pub signer: AccountId,
    pub system_account: AccountId,
    pub sysvar: AccountId,
    pub unchecked_account: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct InitializeIxDummyExampleData {
    pub _var1: bool,
    pub _var2: u8,
    pub _var3: i8,
    pub _var4: u16,
    pub _var5: i16,
    pub _var6: u32,
    pub _var7: i32,
    pub _var8: u64,
    pub _var9: i32,
    pub _var10: f64,
    pub _var11: u128,
    pub _var12: i128,
    pub _ver13: Vec<u8>,
    pub _var14: String,
    pub _var15: AccountId,
    pub _var16: Option<i16>,
    pub _var17: Vec<u32>,
    pub _var18: [i128; 5usize],
    pub _var19: InputParameter,
    pub _var20: Vec<Vec<Vec<Vec<Vec<u8>>>>>,
    pub _var21: Vec<Vec<Vec<Vec<Option<u8>>>>>,
    pub _var22: Vec<Vec<Vec<Vec<Option<Vec<u8>>>>>>,
}
///IxOps implementation for `InitializeIxDummy2` with all required functions.
impl IxOps for InitializeIxDummy2 {
    type IxData = dummy_2::instruction::InitializeIx;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        dummy_2::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = dummy_2::instruction::InitializeIx {
            _var1: self.data._var1,
            _var2: self.data._var2,
            _var3: self.data._var3,
            _var4: self.data._var4,
            _var5: self.data._var5,
            _var6: self.data._var6,
            _var7: self.data._var7,
            _var8: self.data._var8,
            _var9: self.data._var9,
            _var10: self.data._var10,
            _var11: self.data._var11,
            _var12: self.data._var12,
            _ver13: self.data._ver13.clone(),
            _var14: self.data._var14.clone(),
            _var15: todo!(),
            _var16: self.data._var16,
            _var17: self.data._var17.clone(),
            _var18: self.data._var18,
            _var19: todo!(),
            _var20: self.data._var20.clone(),
            _var21: self.data._var21.clone(),
            _var22: self.data._var22.clone(),
        };
        Ok(data)
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
        let signers = vec![todo!()];
        let acc_meta = todo!();
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `InitializeIxDummyExample` with all required
/// functions.
impl IxOps for InitializeIxDummyExample {
    type IxData = dummy_example::instruction::InitializeIx;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        dummy_example::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = dummy_example::instruction::InitializeIx {
            _var1: self.data._var1,
            _var2: self.data._var2,
            _var3: self.data._var3,
            _var4: self.data._var4,
            _var5: self.data._var5,
            _var6: self.data._var6,
            _var7: self.data._var7,
            _var8: self.data._var8,
            _var9: self.data._var9,
            _var10: self.data._var10,
            _var11: self.data._var11,
            _var12: self.data._var12,
            _ver13: self.data._ver13.clone(),
            _var14: self.data._var14.clone(),
            _var15: todo!(),
            _var16: self.data._var16,
            _var17: self.data._var17.clone(),
            _var18: self.data._var18,
            _var19: todo!(),
            _var20: self.data._var20.clone(),
            _var21: self.data._var21.clone(),
            _var22: self.data._var22.clone(),
        };
        Ok(data)
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
        let signers = vec![todo!()];
        let acc_meta = todo!();
        Ok((signers, acc_meta))
    }
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    signer_dummy_2: AccountsStorage<todo!()>,
    account: AccountsStorage<PdaStore>,
    account_info: AccountsStorage<todo!()>,
    account_loader: AccountsStorage<todo!()>,
    boxed: AccountsStorage<todo!()>,
    interace: AccountsStorage<todo!()>,
    interface_account: AccountsStorage<todo!()>,
    option: AccountsStorage<todo!()>,
    program: AccountsStorage<todo!()>,
    signer_dummy_example: AccountsStorage<todo!()>,
    system_account: AccountsStorage<todo!()>,
    sysvar: AccountsStorage<todo!()>,
    unchecked_account: AccountsStorage<todo!()>,
}
