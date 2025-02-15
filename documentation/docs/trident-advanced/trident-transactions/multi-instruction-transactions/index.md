# Multi-Instruction Transactions

Trident supports fuzzing of multiple instructions within one transaction.

By default separate folders for `instructions` and `transactions` are created. For creating multi-instruction transactions, modify the desired `transaction` within its corresponding `.rs` file, or create a new one.


```rust
// example_transaction.rs
use crate::fuzz_transactions::FuzzAccounts;
use crate::instructions::*;
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct ExampleTransaction {
    pub instruction1: ExampleInstruction1, // instruction one
    pub instruction2: ExampleInstruction2, // instruction two
}
/// Custom Transaction Methods
///
/// Provides hooks for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
impl TransactionCustomMethods for ExampleTransaction {}
```

A newly created transactions need to be added to the `FuzzTransactions` enum in the `fuzz_transactions.rs` file. Check the [FuzzTransactions](../../../trident-api-macro/trident-types/fuzz_transactions/index.md) for more details.
