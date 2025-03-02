use crate::fuzz_transactions::FuzzAccounts;
use crate::instructions::*;
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct InitVestingTransaction {
    pub instruction: InitVestingInstruction,
}
/// Custom Transaction Methods
///
/// Provides hooks for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
impl TransactionHooks for InitVestingTransaction {}
