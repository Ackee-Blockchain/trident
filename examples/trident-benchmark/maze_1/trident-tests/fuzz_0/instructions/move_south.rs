use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")]
#[discriminator([146u8, 138u8, 196u8, 38u8, 130u8, 143u8, 149u8, 55u8])]
pub struct MoveSouthInstruction {
    pub accounts: MoveSouthInstructionAccounts,
    pub data: MoveSouthInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(MoveSouthInstructionData)]
#[storage(FuzzAccounts)]
pub struct MoveSouthInstructionAccounts {
    #[account(
        mut,
        storage::name = state,
        storage::account_id = (0..1)
    )]
    pub state: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MoveSouthInstructionData {
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
impl InstructionHooks for MoveSouthInstruction {
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
