# Multi-Instruction Transactions

Trident supports fuzzing of multiple instructions within one transaction.

By default, separate folders for `instructions` and `transactions` are created. To create multi-instruction transactions, either modify an existing `transaction` in its corresponding `.rs` file or create a new one.




!!! warning "Instruction Resolution Order"
    In multi-instruction transactions, instruction hooks are executed in the same order as the instructions are defined in the transaction structure.

Any newly created transaction must be added to the `FuzzTransactions` enum in the `fuzz_transactions.rs` file. See the [FuzzTransactions](../../../trident-api-macro/trident-types/fuzz-transactions.md) page for more details.



## Example

The following example demonstrates:

- Creating a multi-instruction transaction
- Adding the transaction to the `FuzzTransactions` enum


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


impl TransactionHooks for ExampleTransaction {}
```

--- 

```rust
//  fuzz_transactions.rs
/// FuzzTransactions contains all available transactions
///
/// You can create your own transactions by adding new variants to the enum.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-transactions/
#[derive(Arbitrary, TransactionSelector)]
pub enum FuzzTransactions {
    SomeTransaction(SomeTransaction),
    AnotherTransaction(AnotherTransaction),
    SomeAnotherTransaction(SomeAnotherTransaction),
    // Transaction with multiple instructions
    ExampleTransaction(ExampleTransaction),
}
```
