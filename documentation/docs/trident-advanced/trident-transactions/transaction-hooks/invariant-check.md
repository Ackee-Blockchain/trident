# Transaction Invariant Hook

Transaction invariant checks let you compare account states before and after transaction execution (differential analysis), or verify that accounts are in a specific state after the transaction.

This is useful to validate transaction behavior and ensure accounts are not modified in unexpected ways.

!!! warning "Invariant Check Execution Order"
    The `transaction_invariant_check` method is called only after the transaction is `SUCCESSFULLY` executed.


```rust
impl TransactionHooks for ExampleTransaction {
    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // Your custom differential analysis goes here
    }
}
```



## Example

The following example demonstrates:

- Obtaining the owner's balance after the transaction, with `.get_snapshot_after()` (post transaction snapshot)
- Comparing the balance to the expected balance (500 lamports)


```rust
impl TransactionHooks for ExampleTransaction {
    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // Obtain the owner's balance after the transaction
        let owner_lamports = self
            .instruction
            .accounts
            .owner
            .get_snapshot_after()
            .lamports();

        // Compare the balance and return error if not valid
        if owner_lamports > 500 {
            return Err(FuzzingError::with_message(
                "Owner lamports should be less than 500",
            ));
        }
        Ok(())
    }
}
```
