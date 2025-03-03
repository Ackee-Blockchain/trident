use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")]
# [discriminator ([146u8 , 138u8 , 196u8 , 38u8 , 130u8 , 143u8 , 149u8 , 55u8 ,])]
pub struct MoveSouthInstruction {
    pub accounts: MoveSouthInstructionAccounts,
    pub data: MoveSouthInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(MoveSouthInstructionData)]
#[storage(FuzzAccounts)]
pub struct MoveSouthInstructionAccounts {
    #[account(mut,storage = state)]
    state: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct MoveSouthInstructionData {
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
impl InstructionHooks for MoveSouthInstruction {
    type IxAccounts = FuzzAccounts;
}
