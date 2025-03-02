use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("AGpdCBtXUyLWKutvMCVDeTywkxgvQVjJk54btLQNLMiZ")]
# [discriminator ([213u8 , 161u8 , 76u8 , 199u8 , 38u8 , 28u8 , 209u8 , 80u8 ,])]
pub struct WithdrawUnlockedInstruction {
    pub accounts: WithdrawUnlockedInstructionAccounts,
    pub data: WithdrawUnlockedInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(WithdrawUnlockedInstructionData)]
#[storage(FuzzAccounts)]
pub struct WithdrawUnlockedInstructionAccounts {
    #[account(signer, mut, storage = recipient)]
    pub recipient: TridentAccount,
    #[account(mut)]
    pub recipient_token_account: TridentAccount,
    #[account(mut,storage = escrow)]
    pub escrow: TridentAccount,
    #[account(mut,storage = escrow_token_account)]
    pub escrow_token_account: TridentAccount,
    #[account(mut,storage = escrow_pda_authority,seeds = [b"ESCROW_PDA_AUTHORITY"])]
    pub escrow_pda_authority: TridentAccount,
    #[account(mut,storage = mint)]
    pub mint: TridentAccount,
    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", skip_snapshot)]
    pub token_program: TridentAccount,
    #[account(address = "11111111111111111111111111111111", skip_snapshot)]
    pub system_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct WithdrawUnlockedInstructionData {}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for WithdrawUnlockedInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let recipient_token_account = fuzz_accounts
            .recipient_token_account
            .get_or_create_token_account(
                self.accounts.recipient_token_account.account_id,
                client,
                None,
                self.accounts.mint.pubkey(),
                self.accounts.recipient.pubkey(),
                0,
                None,
                false,
                0,
                None,
            );
        self.accounts
            .recipient_token_account
            .set_address(recipient_token_account);
    }
}
