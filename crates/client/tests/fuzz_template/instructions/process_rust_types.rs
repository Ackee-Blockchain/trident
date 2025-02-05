use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// Instruction Struct
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN")]
# [discriminator ([74u8 , 102u8 , 18u8 , 245u8 , 253u8 , 10u8 , 252u8 , 246u8 ,])]
pub struct ProcessRustTypesInstruction {
    pub accounts: ProcessRustTypesInstructionAccounts,
    pub data: ProcessRustTypesInstructionData,
}
/// Instruction Accounts
/// Instruction accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct ProcessRustTypesInstructionAccounts {
    pub composite_account_nested: CompositeAccountNestedAccounts,
    pub signer: TridentAccount,
    pub data_account_1: TridentAccount,
    pub data_account_2: TridentAccount,
    pub data_account_3: TridentAccount,
    pub data_account_4: TridentAccount,
    pub data_account_5: TridentAccount,
    pub data_account_6: TridentAccount,
    pub composite_account: CompositeAccountAccounts,
}
/// Composite Account Structs
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct NestedInnerAccounts {
    pub some_account: TridentAccount,
    pub system_program: TridentAccount,
}
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct CompositeAccountNestedAccounts {
    pub some_account: TridentAccount,
    pub nested_inner: NestedInnerAccounts,
}
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct CompositeAccountAccounts {
    pub some_account: TridentAccount,
    pub signer: TridentAccount,
    pub data_account_1: TridentAccount,
}
/// Instruction Data
/// Instruction data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct ProcessRustTypesInstructionData {
    pub _input_u8: u8,
    pub _input_u16: u16,
    pub _input_u32: u32,
    pub _input_u64: u64,
    pub _input_i8: i8,
    pub _input_i16: i16,
    pub _input_i32: i32,
    pub _input_i64: i64,
    pub _input_i128: i128,
    pub _input_f32: f32,
    pub _input_f64: f64,
    pub _input_string: String,
    pub _input_vec: Vec<u8>,
    pub _input_vec_string: Vec<String>,
    pub _input_bool: bool,
}
/// Instruction Setters
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionSetters for ProcessRustTypesInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
}
