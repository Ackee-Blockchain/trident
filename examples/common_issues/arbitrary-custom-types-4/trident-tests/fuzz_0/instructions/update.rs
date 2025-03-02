use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("CdWkp3CY9CAjBQP73SDCwDDfsumwY7e6DDSjrN5u8Cii")]
# [discriminator ([219u8 , 200u8 , 88u8 , 176u8 , 158u8 , 63u8 , 253u8 , 127u8 ,])]
pub struct UpdateInstruction {
    pub accounts: UpdateInstructionAccounts,
    pub data: UpdateInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct UpdateInstructionAccounts {
    pub counter: TridentAccount,
    pub authority: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct UpdateInstructionData {
    pub input: InputUpdatePrameters,
    pub variant: InputUpdateVariant,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionHooks for UpdateInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let user = fuzz_accounts.user.get_or_create_account(
            self.accounts.authority.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .authority
            .set_account_meta(user.pubkey(), true, true);

        let counter = fuzz_accounts.counter.get_or_create_account(
            self.accounts.counter.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .counter
            .set_account_meta(counter.pubkey(), false, true);
    }
}
