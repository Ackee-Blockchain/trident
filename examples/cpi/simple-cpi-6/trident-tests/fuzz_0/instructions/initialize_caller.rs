use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("FWtSodrkUnovFPnNRCxneP6VWh6JH6jtQZ4PHoP8Ejuz")]
# [discriminator ([204u8 , 76u8 , 200u8 , 172u8 , 185u8 , 14u8 , 99u8 , 166u8 ,])]
pub struct InitializeCallerInstruction {
    pub accounts: InitializeCallerInstructionAccounts,
    pub data: InitializeCallerInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(InitializeCallerInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeCallerInstructionAccounts {
    #[account(signer, mut,storage = signer)]
    pub signer: TridentAccount,
    #[account(address = "HJR1TK8bgrUWzysdpS1pBGBYKF7zi1tU9cS4qj8BW8ZL")]
    pub program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct InitializeCallerInstructionData {
    input: u16,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeCallerInstruction {
    type IxAccounts = FuzzAccounts;
}
