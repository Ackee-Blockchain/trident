use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("5XvBmfPNcHLCgbRK4nRYvfodAnhjArHSed2B3rhkF1Ug")]
# [discriminator ([183u8 , 18u8 , 70u8 , 156u8 , 148u8 , 109u8 , 161u8 , 34u8 ,])]
pub struct WithdrawInstruction {
    pub accounts: WithdrawInstructionAccounts,
    pub data: WithdrawInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct WithdrawInstructionAccounts {
    pub receiver: TridentAccount,
    pub escrow: TridentAccount,
    #[skip_snapshot]
    #[address("11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct WithdrawInstructionData {}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionHooks for WithdrawInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let receiver = fuzz_accounts.receiver.get_or_create_account(
            self.accounts.receiver.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .receiver
            .set_account_meta(receiver.pubkey(), true, true);

        let escrow = fuzz_accounts.escrow.get(self.accounts.escrow.account_id);
        self.accounts.escrow.set_account_meta(escrow, false, true);
    }
}
