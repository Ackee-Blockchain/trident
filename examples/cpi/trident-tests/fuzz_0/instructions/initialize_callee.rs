use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("CWjKHxkHU7kqRKqNutPAbxogKg3K1crH61gwwzsHjpC4")]
#[discriminator([164u8, 75u8, 79u8, 32u8, 57u8, 23u8, 116u8, 175u8])]
pub struct InitializeCalleeInstruction {
    pub accounts: InitializeCalleeInstructionAccounts,
    pub data: InitializeCalleeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeCalleeInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeCalleeInstructionAccounts {
    #[account(
        signer,
        storage::name = signer,
        storage::account_id = (0..1)
    )]
    pub signer: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeCalleeInstructionData {
    pub input: u16,
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
    fn set_accounts(
        &mut self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut Self::IxAccounts,
        rng: &mut TridentRng,
    ) {
        self.data.input = rng.gen_range(0..u16::MAX);
    }
}
