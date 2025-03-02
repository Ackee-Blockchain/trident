use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("BM8vocQeC2VuDf1KhbHLsZxTh7owzDNTAkKyZoTxFiUs")]
#[discriminator([219u8, 200u8, 88u8, 176u8, 158u8, 63u8, 253u8, 127u8])]
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
    pub input1: u8,
    pub input2: u8,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionHooks for UpdateInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(
        &mut self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) {
        let counter = fuzz_accounts.counter.get(self.accounts.counter.account_id);
        self.accounts
            .counter
            .set_account_meta(counter.pubkey(), false, true);

        let authority = fuzz_accounts.user.get(self.accounts.authority.account_id);
        self.accounts
            .authority
            .set_account_meta(authority.pubkey(), true, true);
    }
}
