use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("8bPSKGoWCdAW8Hu3S1hLHPpBv8BNwse4jDyaXNrj3jWB")]
#[discriminator([175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8])]
pub struct InitializeInstruction {
    pub accounts: InitializeInstructionAccounts,
    pub data: InitializeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeInstructionAccounts {}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeInstruction {
    type IxAccounts = FuzzAccounts;
}
