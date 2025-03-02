use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("dk5VmuCSjrG6iRVXRycKZ6mS4rDCyvBrYJvcfyqWGcU")]
# [discriminator ([13u8 , 245u8 , 180u8 , 103u8 , 254u8 , 182u8 , 121u8 , 4u8 ,])]
pub struct InvestInstruction {
    pub accounts: InvestInstructionAccounts,
    pub data: InvestInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InvestInstructionAccounts {
    pub investor: TridentAccount,
    pub project: TridentAccount,
    pub state: TridentAccount,
    #[skip_snapshot]
    #[address("11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct InvestInstructionData {
    pub amount: u64,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionHooks for InvestInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let investor = fuzz_accounts.investor.get_or_create_account(
            self.accounts.investor.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .investor
            .set_account_meta(investor.pubkey(), true, true);

        let state = fuzz_accounts.state.get(self.accounts.state.account_id);
        self.accounts.state.set_account_meta(state, false, true);

        let project = fuzz_accounts.project.get(self.accounts.project.account_id);
        self.accounts.project.set_account_meta(project, false, true);
    }
}
