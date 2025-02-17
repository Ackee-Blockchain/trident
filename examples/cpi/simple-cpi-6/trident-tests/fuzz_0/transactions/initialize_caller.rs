use crate::fuzz_transactions::FuzzAccounts;
use crate::instructions::*;
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct InitializeCallerTransaction {
    pub instruction: InitializeCallerInstruction,
}
/// Custom Transaction Methods
///
/// Provides hooks for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
impl TransactionCustomMethods for InitializeCallerTransaction {}
