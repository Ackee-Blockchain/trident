use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("8bPSKGoWCdAW8Hu3S1hLHPpBv8BNwse4jDyaXNrj3jWB")]
# [discriminator ([175u8 , 175u8 , 109u8 , 31u8 , 13u8 , 152u8 , 155u8 , 237u8 ,])]
pub struct InitializeInstruction {
    pub accounts: InitializeInstructionAccounts,
    pub data: InitializeInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(InitializeInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeInstructionAccounts {}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
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
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
}
