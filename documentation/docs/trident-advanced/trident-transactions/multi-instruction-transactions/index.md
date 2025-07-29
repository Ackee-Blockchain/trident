# Multi-Instruction Transactions

Trident supports fuzzing of multiple instructions within one transaction.

By default, separate folders for `instructions` and `transactions` are created. To create multi-instruction transactions, either modify an existing `transaction` in its corresponding `.rs` file or create a new one.




!!! warning "Instruction Resolution Order"
    In multi-instruction transactions, instruction hooks are executed in the same order as the instructions are defined in the transaction structure.


## Example

The following example demonstrates:

- Creating a multi-instruction transaction
- Adding the transaction to the `FuzzTransactions` enum


```rust
// example_transaction.rs
use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct ExampleTransaction {
    pub instruction1: ExampleInstruction1, // instruction one
    pub instruction2: ExampleInstruction2, // instruction two
}


impl TransactionHooks for ExampleTransaction {}
```


