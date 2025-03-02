use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("CdWkp3CY9CAjBQP73SDCwDDfsumwY7e6DDSjrN5u8Cii")]
# [discriminator ([175u8 , 175u8 , 109u8 , 31u8 , 13u8 , 152u8 , 155u8 , 237u8 ,])]
pub struct InitializeInstruction {
    pub accounts: InitializeInstructionAccounts,
    pub data: InitializeInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InitializeInstructionAccounts {
    pub counter: TridentAccount,
    pub user: TridentAccount,
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
        let user = fuzz_accounts.user.get_or_create_account(
            self.accounts.user.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .user
            .set_account_meta(user.pubkey(), true, true);

        let counter = fuzz_accounts.counter.get_or_create_account(
            self.accounts.counter.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .counter
            .set_account_meta(counter.pubkey(), true, true);
    }
}
