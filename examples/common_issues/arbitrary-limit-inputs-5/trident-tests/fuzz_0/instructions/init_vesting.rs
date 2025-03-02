use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("AGpdCBtXUyLWKutvMCVDeTywkxgvQVjJk54btLQNLMiZ")]
# [discriminator ([119u8 , 192u8 , 67u8 , 41u8 , 47u8 , 82u8 , 152u8 , 27u8 ,])]
pub struct InitVestingInstruction {
    pub accounts: InitVestingInstructionAccounts,
    pub data: InitVestingInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(InitVestingInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitVestingInstructionAccounts {
    #[account(signer, mut,storage = sender)]
    pub sender: TridentAccount,
    #[account(mut)]
    pub sender_token_account: TridentAccount,
    #[account(mut,storage = escrow,seeds = [instruction_data.recipient.get_pubkey().as_ref(), b"ESCROW_SEED"])]
    pub escrow: TridentAccount,
    #[account(mut)]
    pub escrow_token_account: TridentAccount,
    pub mint: TridentAccount,
    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", skip_snapshot)]
    pub token_program: TridentAccount,
    #[account(address = "11111111111111111111111111111111", skip_snapshot)]
    pub system_program: TridentAccount,
}
/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct InitVestingInstructionData {
    recipient: TridentPubkey,
    amount: u64,
    start_at: u64,
    end_at: u64,
    interval: u64,
}
// -------------------------------------------------------------------
// -------------------------------------------------------------------
// Implement Arbitrary
impl<'a> Arbitrary<'a> for InitVestingInstructionData {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        // obtain AccountId
        let recipient = AccountId::arbitrary(u)?;

        // limit the generated amount to the 1_000_000
        let amount = u.int_in_range(1..=1_000_000)?;

        // now we want to obtain
        // - start_at
        // - end_at
        // - interval
        // however we want to limit the data such that:
        // - start_at < end_at
        // - end_at - start_at > interval
        // - interval has lower limit of 500 and upper limit of 1000.

        let start_at: u64 = u.int_in_range(1_000_000..=5_000_000)?;
        let end_at: u64 = u.int_in_range(1_000_000..=5_000_000)?;
        let interval: u64 = u.int_in_range(500..=1000)?;

        // ensure that start_at < end_at
        if start_at >= end_at {
            return Err(arbitrary::Error::IncorrectFormat);
        }

        // ensure that end_at - start_at > interval
        match end_at.checked_sub(start_at) {
            Some(diff) => {
                if diff <= interval {
                    return Err(arbitrary::Error::IncorrectFormat);
                }
            }
            None => return Err(arbitrary::Error::IncorrectFormat),
        }

        Ok(InitVestingInstructionData {
            recipient: TridentPubkey::from(recipient),
            amount,
            start_at,
            end_at,
            interval,
        })
    }
    // -------------------------------------------------------------------
    // -------------------------------------------------------------------
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitVestingInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let recipient = fuzz_accounts.recipient.get_or_create(
            self.data.recipient.account_id,
            client,
            None,
            None,
        );

        self.data.recipient.set_pubkey(recipient);
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let sender = self.accounts.sender.pubkey();

        let mint = fuzz_accounts.mint.get_or_create_mint_account(
            self.accounts.mint.account_id,
            client,
            None,
            6,
            &sender,
            None,
        );

        self.accounts.mint.set_address(mint);

        let sender_token_account = fuzz_accounts
            .sender_token_account
            .get_or_create_token_account(
                self.accounts.sender_token_account.account_id,
                client,
                None,
                mint,
                sender,
                u64::MAX,
                None,
                false,
                0,
                None,
            );
        self.accounts
            .sender_token_account
            .set_address(sender_token_account);

        let escrow_token_account = fuzz_accounts
            .escrow_token_account
            .get_or_create_token_account(
                self.accounts.escrow_token_account.account_id,
                client,
                None,
                mint,
                self.accounts.sender.pubkey(),
                0,
                None,
                false,
                0,
                None,
            );

        self.accounts
            .escrow_token_account
            .set_address(escrow_token_account);
    }
}
