use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit")]
# [discriminator ([18u8 , 187u8 , 169u8 , 213u8 , 94u8 , 180u8 , 86u8 , 152u8 ,])]
pub struct InitializeFnInstruction {
    pub accounts: InitializeFnInstructionAccounts,
    pub data: InitializeFnInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(InitializeFnInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeFnInstructionAccounts {
    #[account(mut,signer,storage = author)]
    pub author: TridentAccount,
    #[account(
        mut,
        storage = hello_world_account,
        seeds = [b"hello_world_seed"],
    )]
    pub hello_world_account: TridentAccount,
    #[account(address = "11111111111111111111111111111111", skip_snapshot)]
    pub system_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct InitializeFnInstructionData {
    input: u8,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeFnInstruction {
    type IxAccounts = FuzzAccounts;
}
