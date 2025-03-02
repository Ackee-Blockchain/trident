use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("4VS6YQzYbdEVbt9iw1eoUnpNzqC5TJPoSeJH9qFfgm4m")]
# [discriminator ([119u8 , 192u8 , 67u8 , 41u8 , 47u8 , 82u8 , 152u8 , 27u8 ,])]
pub struct InitVestingInstruction {
    pub accounts: InitVestingInstructionAccounts,
    pub data: InitVestingInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InitVestingInstructionAccounts {
    pub sender: TridentAccount,
    pub sender_token_account: TridentAccount,
    pub escrow: TridentAccount,
    pub escrow_token_account: TridentAccount,
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
pub struct InitVestingInstructionData {
    pub recipient: TridentPubkey,
    pub amount: u64,
    pub start_at: u64,
    pub end_at: u64,
    pub interval: u64,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionHooks for InitVestingInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let recipient = fuzz_accounts.recipient.get_or_create_account(
            self.data.recipient.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.data.recipient.pubkey = recipient.pubkey();
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let recipient = fuzz_accounts.recipient.get(self.data.recipient.account_id);

        let mint =
            fuzz_accounts
                .mint
                .get_or_create_mint_account(0, client, 6, &recipient.pubkey(), None);
        self.accounts
            .mint
            .set_account_meta(mint.pubkey(), false, false);

        let sender = fuzz_accounts.sender.get_or_create_account(
            self.accounts.sender.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .sender
            .set_account_meta(sender.pubkey(), true, true);

        let sender_token_account = fuzz_accounts
            .sender_token_account
            .get_or_create_token_account(
                self.accounts.sender_token_account.account_id,
                client,
                mint.pubkey(),
                sender.pubkey(),
                u64::MAX,
                None,
                false,
                0,
                None,
            );
        self.accounts.sender_token_account.set_account_meta(
            sender_token_account.pubkey(),
            false,
            true,
        );

        let escrow = fuzz_accounts.escrow.get_or_create_account(
            self.accounts.escrow.account_id,
            client,
            &[&recipient.pubkey().to_bytes(), b"ESCROW_SEED"],
            &self.get_program_id(),
        );
        self.accounts.escrow.set_account_meta(escrow, false, true);

        let escrow_token_account = fuzz_accounts
            .escrow_token_account
            .get_or_create_token_account(
                self.accounts.escrow_token_account.account_id,
                client,
                mint.pubkey(),
                sender.pubkey(),
                0,
                None,
                false,
                0,
                None,
            );
        self.accounts.escrow_token_account.set_account_meta(
            escrow_token_account.pubkey(),
            false,
            true,
        );
    }
}
