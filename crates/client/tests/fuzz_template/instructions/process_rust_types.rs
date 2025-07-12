use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN")]
#[discriminator([74u8, 102u8, 18u8, 245u8, 253u8, 10u8, 252u8, 246u8])]
pub struct ProcessRustTypesInstruction {
    pub accounts: ProcessRustTypesInstructionAccounts,
    pub data: ProcessRustTypesInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ProcessRustTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct ProcessRustTypesInstructionAccounts {
    pub composite_account_nested: CompositeAccountNestedAccounts,

    #[account(mut, signer)]
    pub signer: TridentAccount,

    pub data_account_1: TridentAccount,

    pub data_account_2: TridentAccount,

    pub data_account_3: TridentAccount,

    pub data_account_4: TridentAccount,

    pub data_account_5: TridentAccount,

    pub data_account_6: TridentAccount,

    pub composite_account: CompositeAccountAccounts,
}

/// Composite Account: composite_account_nested
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ProcessRustTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct CompositeAccountNestedAccounts {
    #[account(mut)]
    pub some_account: TridentAccount,

    pub nested_inner: NestedInnerAccounts,
}

#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ProcessRustTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct NestedInnerAccounts {
    pub some_account: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Composite Account: composite_account
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ProcessRustTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct CompositeAccountAccounts {
    pub some_account: TridentAccount,

    #[account(mut, signer)]
    pub signer: TridentAccount,

    pub data_account_1: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
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
}
