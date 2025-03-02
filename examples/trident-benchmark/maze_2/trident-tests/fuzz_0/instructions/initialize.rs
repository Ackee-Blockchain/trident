use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")]
# [discriminator ([175u8 , 175u8 , 109u8 , 31u8 , 13u8 , 152u8 , 155u8 , 237u8 ,])]
pub struct InitializeInstruction {
    pub accounts: InitializeInstructionAccounts,
    pub data: InitializeInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InitializeInstructionAccounts {
    pub state_author: TridentAccount,
    pub state: TridentAccount,
    #[skip_snapshot]
    #[address("11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct InitializeInstructionData {}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionHooks for InitializeInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let author = fuzz_accounts.state_author.get_or_create_account(
            self.accounts.state_author.account_id,
            client,
            50 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .state_author
            .set_account_meta(author.pubkey(), true, true);

        let state = fuzz_accounts.state.get_or_create_account(
            1,
            client,
            &[b"state"],
            &self.get_program_id(),
        );
        self.accounts.state.set_account_meta(state, false, true);
    }
}
