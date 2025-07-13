use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(TridentInstruction, Default)]
#[program_id("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")]
# [discriminator ([220u8 , 96u8 , 254u8 , 139u8 , 6u8 , 133u8 , 127u8 , 93u8 ,])]
pub struct MoveEastInstruction {
    pub accounts: MoveEastInstructionAccounts,
    pub data: MoveEastInstructionData,
}
/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(MoveEastInstructionData)]
#[storage(FuzzAccounts)]
pub struct MoveEastInstructionAccounts {
    #[account(
        mut,
        storage::name = state,
        storage::account_id = (0..1)
    )]
    pub state: TridentAccount,
}
/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MoveEastInstructionData {
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
impl InstructionHooks for MoveEastInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(
        &mut self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut Self::IxAccounts,
        rng: &mut TridentRng,
    ) {
        self.data.p0 = rng.gen_range(0..u64::MAX);
        self.data.p1 = rng.gen_range(0..u64::MAX);
        self.data.p2 = rng.gen_range(0..u64::MAX);
        self.data.p3 = rng.gen_range(0..u64::MAX);
        self.data.p4 = rng.gen_range(0..u64::MAX);
        self.data.p5 = rng.gen_range(0..u64::MAX);
        self.data.p6 = rng.gen_range(0..u64::MAX);
        self.data.p7 = rng.gen_range(0..u64::MAX);
    }
}
