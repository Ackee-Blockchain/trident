use crate::fuzz_transactions::FuzzAccounts;
use crate::instructions::*;
use trident_fuzz::fuzzing::*;
/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct InitializeTransaction {
    pub instruction: InitializeInstruction,
}
/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for InitializeTransaction {
    type IxAccounts = FuzzAccounts;
}
