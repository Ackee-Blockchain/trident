# Error Handler

Trident allows you to specify a custom error handler for each instruction.

This can be particularly helpful in the following scenarios:

- If a transaction returns an error, you can choose to omit this error and continue the fuzzing process.
- Using the `tx_error_handler`, you can check if the returned error is desired based on the accounts and input data that were used.

!!! tip

    By default, transaction errors are propagated, meaning that if the transaction fails, the fuzzing iteration is stopped, and a new fuzzing iteration is started.

```rust
/// Default implementation
fn tx_error_handler(
    &self,
    e: TransactionError,
    ix_data: Vec<u8>,
    pre_ix_acc_infos: &[SnapshotAccount],
) -> Result<(), TransactionError> {
    Err(e)
}
```

To omit the error and continue with the next instruction in the iteration, you can use the following implementation:

```rust
/// Custom implementation
fn tx_error_handler(
    &self,
    e: TransactionError,
    ix_data: Vec<u8>,
    pre_ix_acc_infos: &[SnapshotAccount],
) -> Result<(), TransactionError> {
    Ok(())
}
```
