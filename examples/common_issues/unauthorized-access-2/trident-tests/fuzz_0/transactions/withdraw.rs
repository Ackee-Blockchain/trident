use crate::fuzz_transactions::FuzzAccounts;
use crate::instructions::*;
use crate::types::*;
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct WithdrawTransaction {
    pub instruction: WithdrawInstruction,
}
/// Custom Transaction Methods
///
/// Provides hooks for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
impl TransactionHooks for WithdrawTransaction {
    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        if let Ok(escrow_pre) = Escrow::deserialize(
            &mut self
                .instruction
                .accounts
                .escrow
                .get_snapshot_before()
                .data_no_discriminator(),
        ) {
            let receiver_key = self
                .instruction
                .accounts
                .receiver
                .get_snapshot_before()
                .pubkey();
            let receiver_lamports_before = self
                .instruction
                .accounts
                .receiver
                .get_snapshot_before()
                .lamports();
            let receiver_lamports_after = self
                .instruction
                .accounts
                .receiver
                .get_snapshot_after()
                .lamports();

            // If the Receiver (i.e. Signer in the Context) and stored Receiver inside Escrow Account,
            // do not match, however the receiver`s balance increased, we found an Error
            if receiver_key != escrow_pre.receiver.pubkey
                && receiver_lamports_before < receiver_lamports_after
            {
                return Err(FuzzingError::CustomMessage("Balance Mismatch".to_string()));
            }
        }

        Ok(())
    }
}
