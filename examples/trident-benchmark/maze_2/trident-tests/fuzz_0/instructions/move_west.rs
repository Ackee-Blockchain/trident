use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")]
#[discriminator([122u8, 187u8, 56u8, 38u8, 248u8, 122u8, 182u8, 106u8])]
pub struct MoveWestInstruction {
    pub accounts: MoveWestInstructionAccounts,
    pub data: MoveWestInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(MoveWestInstructionData)]
#[storage(FuzzAccounts)]
pub struct MoveWestInstructionAccounts {
    #[account(
        mut,
        storage::name = state,
        storage::account_id = (0..1)
    )]
    pub state: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MoveWestInstructionData {
    pub p0: u64,

    pub p1: u64,

    pub p2: u64,

    pub p3: u64,

    pub p4: u64,

    pub p5: u64,

    pub p6: u64,

    pub p7: u64,
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

    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        self.data.p0 = trident.gen_range(0..u64::MAX);
        self.data.p1 = trident.gen_range(0..u64::MAX);
        self.data.p2 = trident.gen_range(0..u64::MAX);
        self.data.p3 = trident.gen_range(0..u64::MAX);
        self.data.p4 = trident.gen_range(0..u64::MAX);
        self.data.p5 = trident.gen_range(0..u64::MAX);
        self.data.p6 = trident.gen_range(0..u64::MAX);
        self.data.p7 = trident.gen_range(0..u64::MAX);
    }
}
