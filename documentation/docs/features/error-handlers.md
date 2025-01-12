# Error Handler

Trident allows you to specify custom error handler for each Instruction.

This can be particularly helpful:

- If Transaction returns Error, you can specify to omit this error and continue with the fuzzing instruction.
- Using the `tx_error_handler` you can check if the error returned is desired based on the Accounts and Input data that were used.

!!! tip

    The default behavior of the function is that the error is returned.

```rust
/// default implementation
fn tx_error_handler(
    &self,
    e: FuzzClientErrorWithOrigin,
    ix_data: Vec<u8>,
    pre_ix_acc_infos: &[SnapshotAccount],
) -> Result<(), FuzzClientErrorWithOrigin> {
    Err(e)
}
```

To omit the Error and continue with the next Instruction in the iteration, you can do

```rust
/// custom implementation
fn tx_error_handler(
    &self,
    e: FuzzClientErrorWithOrigin,
    ix_data: Vec<u8>,
    pre_ix_acc_infos: &[SnapshotAccount],
) -> Result<(), FuzzClientErrorWithOrigin> {
    Ok(())
}
```
