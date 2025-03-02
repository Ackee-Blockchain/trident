use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("4VS6YQzYbdEVbt9iw1eoUnpNzqC5TJPoSeJH9qFfgm4m")]
# [discriminator ([213u8 , 161u8 , 76u8 , 199u8 , 38u8 , 28u8 , 209u8 , 80u8 ,])]
pub struct WithdrawUnlockedInstruction {
    pub accounts: WithdrawUnlockedInstructionAccounts,
    pub data: WithdrawUnlockedInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct WithdrawUnlockedInstructionAccounts {
    pub recipient: TridentAccount,
    pub recipient_token_account: TridentAccount,
    pub escrow: TridentAccount,
    pub escrow_token_account: TridentAccount,
    pub escrow_pda_authority: TridentAccount,
    pub mint: TridentAccount,
    #[skip_snapshot]
    #[address("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,
    #[skip_snapshot]
    #[address("11111111111111111111111111111111")]
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
impl InstructionHooks for WithdrawUnlockedInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let recipient = fuzz_accounts
            .recipient
            .get(self.accounts.recipient.account_id);
        self.accounts
            .recipient
            .set_account_meta(recipient.pubkey(), true, true);

        let mint = fuzz_accounts.mint.get(0);
        self.accounts
            .mint
            .set_account_meta(mint.pubkey(), false, true);

        let recipient_token_account = fuzz_accounts
            .recipient_token_account
            .get_or_create_token_account(
                self.accounts.recipient_token_account.account_id,
                client,
                mint.pubkey(),
                recipient.pubkey(),
                0,
                None,
                false,
                0,
                None,
            );
        self.accounts.recipient_token_account.set_account_meta(
            recipient_token_account.pubkey(),
            false,
            true,
        );

        let escrow = fuzz_accounts.escrow.get(self.accounts.escrow.account_id);
        self.accounts.escrow.set_account_meta(escrow, false, true);

        let escrow_pda_authority = fuzz_accounts.escrow_pda_authority.get_or_create_account(
            self.accounts.escrow_pda_authority.account_id,
            client,
            &[b"ESCROW_PDA_AUTHORITY"],
            &self.get_program_id(),
        );
        self.accounts
            .escrow_pda_authority
            .set_account_meta(escrow_pda_authority, false, false);

        let escrow_token_account = fuzz_accounts
            .escrow_token_account
            .get(self.accounts.escrow_token_account.account_id);
        self.accounts.escrow_token_account.set_account_meta(
            escrow_token_account.pubkey(),
            false,
            true,
        );
    }
}
