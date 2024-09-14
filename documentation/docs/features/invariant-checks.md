# Invariant Checks


Trident allows you to (optionally) specify Invariant Checks for each Instruction.

The Invariant Check will be called after the Instruction was successfully invoked. Within the Invariant Check you can compare the contents of Accounts before and after the Instruction was called.

!!! important

    Returning error in the Invariant Check is considered as detected undesired behavior (i.e. issue/crash detected).

```rust
fn check(
    &self,
    _pre_ix: Self::IxSnapshot,
    post_ix: Self::IxSnapshot,
    _ix_data: Self::IxData,
) -> Result<(), FuzzingError> {
    if let Some(hello_world_account) = post_ix.hello_world_account {
        if hello_world_account.input == 253 {
            return Err(FuzzingError::Custom(1));
        }
    }
    Ok(())
}
```

!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
