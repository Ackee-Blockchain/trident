use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(TridentInstruction, Default)]
#[program_id("77skervubsozZaRdojomG7FK8T2QQppxtSqG8ag9D4qV")]
# [discriminator ([204u8 , 76u8 , 200u8 , 172u8 , 185u8 , 14u8 , 99u8 , 166u8 ,])]
pub struct InitializeCallerInstruction {
    pub accounts: InitializeCallerInstructionAccounts,
    pub data: InitializeCallerInstructionData,
}
/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeCallerInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeCallerInstructionAccounts {
    #[account(
        signer,
        mut,
        storage::name = signer,
        storage::account_id = (0..1)
    )]
    signer: TridentAccount,
    #[account(address = "CWjKHxkHU7kqRKqNutPAbxogKg3K1crH61gwwzsHjpC4")]
    program: TridentAccount,
}
/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
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
    fn set_data(
        &mut self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut Self::IxAccounts,
        rng: &mut TridentRng,
    ) {
        self.data.input = rng.gen_range(0..u16::MAX);
    }
}
