use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("CWjKHxkHU7kqRKqNutPAbxogKg3K1crH61gwwzsHjpC4")]
# [discriminator ([164u8 , 75u8 , 79u8 , 32u8 , 57u8 , 23u8 , 116u8 , 175u8 ,])]
pub struct InitializeCalleeInstruction {
    pub accounts: InitializeCalleeInstructionAccounts,
    pub data: InitializeCalleeInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(InitializeCalleeInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeCalleeInstructionAccounts {
    #[account(signer)]
    signer: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct InitializeCalleeInstructionData {
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
impl InstructionHooks for InitializeCalleeInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, _client: &mut impl FuzzClient, _fuzz_accounts: &mut Self::IxAccounts) {
        // nothing to set
    }
    fn set_accounts(
        &mut self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut Self::IxAccounts,
    ) {
        // nothing to set
    }
}
