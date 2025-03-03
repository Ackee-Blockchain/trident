use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")]
# [discriminator ([122u8 , 187u8 , 56u8 , 38u8 , 248u8 , 122u8 , 182u8 , 106u8 ,])]
pub struct MoveWestInstruction {
    pub accounts: MoveWestInstructionAccounts,
    pub data: MoveWestInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(MoveWestInstructionData)]
#[storage(FuzzAccounts)]
pub struct MoveWestInstructionAccounts {
    #[account(mut,storage = state)]
    state: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct MoveWestInstructionData {
    p0: u64,
    p1: u64,
    p2: u64,
    p3: u64,
    p4: u64,
    p5: u64,
    p6: u64,
    p7: u64,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for MoveWestInstruction {
    type IxAccounts = FuzzAccounts;
}
