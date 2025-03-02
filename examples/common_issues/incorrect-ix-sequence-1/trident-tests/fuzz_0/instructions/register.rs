use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use incorrect_ix_sequence_1::PROJECT_SEED;
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("dk5VmuCSjrG6iRVXRycKZ6mS4rDCyvBrYJvcfyqWGcU")]
# [discriminator ([211u8 , 124u8 , 67u8 , 15u8 , 211u8 , 194u8 , 178u8 , 240u8 ,])]
pub struct RegisterInstruction {
    pub accounts: RegisterInstructionAccounts,
    pub data: RegisterInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct RegisterInstructionAccounts {
    pub project_author: TridentAccount,
    pub project: TridentAccount,
    pub state: TridentAccount,
    #[skip_snapshot]
    #[address("11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct RegisterInstructionData {}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionHooks for RegisterInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let project_author = fuzz_accounts.project_author.get_or_create_account(
            self.accounts.project_author.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .project_author
            .set_account_meta(project_author.pubkey(), true, true);

        let state = fuzz_accounts.state.get(self.accounts.state.account_id);
        self.accounts.state.set_account_meta(state, false, true);

        let project = fuzz_accounts.project.get_or_create_account(
            self.accounts.project.account_id,
            client,
            &[
                project_author.pubkey().as_ref(),
                state.as_ref(),
                PROJECT_SEED.as_ref(),
            ],
            &self.get_program_id(),
        );
        self.accounts.project.set_account_meta(project, false, true);
    }
}
