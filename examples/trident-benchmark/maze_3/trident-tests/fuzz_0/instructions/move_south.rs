use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")]
# [discriminator ([146u8 , 138u8 , 196u8 , 38u8 , 130u8 , 143u8 , 149u8 , 55u8 ,])]
pub struct MoveSouthInstruction {
    pub accounts: MoveSouthInstructionAccounts,
    pub data: MoveSouthInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct MoveSouthInstructionAccounts {
    pub state: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
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
impl InstructionHooks for MoveSouthInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(
        &mut self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) {
        let state = fuzz_accounts.state.get(1);
        self.accounts.state.set_account_meta(state, false, true);
    }
}
