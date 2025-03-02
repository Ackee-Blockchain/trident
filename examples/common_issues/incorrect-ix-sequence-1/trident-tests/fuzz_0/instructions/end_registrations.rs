use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("dk5VmuCSjrG6iRVXRycKZ6mS4rDCyvBrYJvcfyqWGcU")]
#[discriminator([6u8, 118u8, 91u8, 63u8, 166u8, 46u8, 132u8, 233u8])]
pub struct EndRegistrationsInstruction {
    pub accounts: EndRegistrationsInstructionAccounts,
    pub data: EndRegistrationsInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct EndRegistrationsInstructionAccounts {
    pub author: TridentAccount,
    pub state: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct EndRegistrationsInstructionData {}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionHooks for EndRegistrationsInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(
        &mut self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) {
        let author = fuzz_accounts.author.get(self.accounts.author.account_id);
        self.accounts
            .author
            .set_account_meta(author.pubkey(), true, true);

        let state = fuzz_accounts.state.get(self.accounts.state.account_id);
        self.accounts.state.set_account_meta(state, false, true);
    }
}
