use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")]
# [discriminator ([220u8 , 96u8 , 254u8 , 139u8 , 6u8 , 133u8 , 127u8 , 93u8 ,])]
pub struct MoveEastInstruction {
    pub accounts: MoveEastInstructionAccounts,
    pub data: MoveEastInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(MoveEastInstructionData)]
#[storage(FuzzAccounts)]
pub struct MoveEastInstructionAccounts {
    #[account(mut,storage = state)]
    state: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct MoveEastInstructionData {
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
impl InstructionHooks for MoveEastInstruction {
    type IxAccounts = FuzzAccounts;
}
