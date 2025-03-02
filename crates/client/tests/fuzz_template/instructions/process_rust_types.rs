use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN")]
# [discriminator ([74u8 , 102u8 , 18u8 , 245u8 , 253u8 , 10u8 , 252u8 , 246u8 ,])]
pub struct ProcessRustTypesInstruction {
    pub accounts: ProcessRustTypesInstructionAccounts,
    pub data: ProcessRustTypesInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(ProcessRustTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct ProcessRustTypesInstructionAccounts {
    composite_account_nested: CompositeAccountNestedAccounts,
    #[account(signer)]
    signer: TridentAccount,
    data_account_1: TridentAccount,
    data_account_2: TridentAccount,
    data_account_3: TridentAccount,
    data_account_4: TridentAccount,
    data_account_5: TridentAccount,
    data_account_6: TridentAccount,
    composite_account: CompositeAccountAccounts,
}
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(ProcessRustTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct NestedInnerAccounts {
    some_account: TridentAccount,
    #[account(address = "11111111111111111111111111111111")]
    system_program: TridentAccount,
}
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(ProcessRustTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct CompositeAccountNestedAccounts {
    some_account: TridentAccount,
    nested_inner: NestedInnerAccounts,
}
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(ProcessRustTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct CompositeAccountAccounts {
    some_account: TridentAccount,
    #[account(signer)]
    signer: TridentAccount,
    data_account_1: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct ProcessRustTypesInstructionData {
    _input_u8: u8,
    _input_u16: u16,
    _input_u32: u32,
    _input_u64: u64,
    _input_i8: i8,
    _input_i16: i16,
    _input_i32: i32,
    _input_i64: i64,
    _input_i128: i128,
    _input_f32: f32,
    _input_f64: f64,
    _input_string: String,
    _input_vec: Vec<u8>,
    _input_vec_string: Vec<String>,
    _input_bool: bool,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for ProcessRustTypesInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
}
