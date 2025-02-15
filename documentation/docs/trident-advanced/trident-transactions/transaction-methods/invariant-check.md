# Transaction Invariant Hook

`Transaction invariant check` lets you compare the state of `the accounts before and after the transaction`, or verify that the accounts are in a specific state after the transaction.

This is useful to check that the transaction is valid, or to check that the transaction is not `modifying the accounts in an unexpected way`.

!!! warning "Invariant Check Execution order"
    The `transaction_invariant_check` method is called only after the transaction is `SUCCESSFULLY` executed.


```rust
impl TransactionCustomMethods for ExampleTransaction {
    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // obtain the owners balance after the transaction
        let owner_lamports = self
            .instruction1
            .accounts
            .borrowAccounts
            .owner
            .get_snapshot_after()
            .lamports();

        // compare the balance and return error if not valid
        if owner_lamports > 500 {
            return Err(FuzzingError::with_message(
                "Owner lamports should be less than 500",
            ));
        }
        Ok(())
    }
}

```
