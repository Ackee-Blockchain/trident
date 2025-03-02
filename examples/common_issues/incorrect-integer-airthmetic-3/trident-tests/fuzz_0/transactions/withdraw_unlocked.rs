use crate::fuzz_transactions::FuzzAccounts;
use crate::instructions::*;
use crate::types::*;
use solana_sdk::program_pack::Pack;
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct WithdrawUnlockedTransaction {
    pub instruction: WithdrawUnlockedInstruction,
}
/// Custom Transaction Methods
///
/// Provides hooks for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
impl TransactionHooks for WithdrawUnlockedTransaction {
    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        if let Ok(escrow) = Escrow::deserialize(
            &mut self
                .instruction
                .accounts
                .escrow
                .get_snapshot_before()
                .data_no_discriminator(),
        ) {
            let recipient = self
                .instruction
                .accounts
                .recipient
                .get_snapshot_before()
                .pubkey();

            let recipient_token_account_pre = match spl_token::state::Account::unpack(
                self.instruction
                    .accounts
                    .recipient_token_account
                    .get_snapshot_before()
                    .data(),
            ) {
                Ok(recipient_token_account_pre) => recipient_token_account_pre,
                Err(_) => return Ok(()),
            };

            let recipient_token_account_post = match spl_token::state::Account::unpack(
                self.instruction
                    .accounts
                    .recipient_token_account
                    .get_snapshot_after()
                    .data(),
            ) {
                Ok(recipient_token_account_post) => recipient_token_account_post,
                Err(_) => return Ok(()),
            };

            if escrow.recipient.pubkey == recipient {
                if recipient_token_account_pre.amount == recipient_token_account_post.amount {
                    // Recipient was not able to withdraw
                    return Err(FuzzingError::CustomMessage("BALANCE MISMATCH".to_string()));
                } else if recipient_token_account_pre.amount + escrow.amount
                    != recipient_token_account_post.amount
                {
                    if recipient_token_account_pre.amount + escrow.amount
                        > recipient_token_account_post.amount
                    {
                        // Recipient withdraw less
                        return Err(FuzzingError::Custom(15));
                    } else {
                        // Recipient withdraw more
                        return Err(FuzzingError::Custom(2));
                    }
                }
            }
        }
        Ok(())
    }
}
