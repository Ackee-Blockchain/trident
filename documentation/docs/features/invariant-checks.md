# Invariant Checks

Trident allows you to (optionally) specify invariant checks for each Instruction.

The invariant check will be called after the Instruction has been successfully invoked. Within the invariant check, you can compare the contents of accounts before and after the Instruction.

!!! important

    Returning an error in the Invariant Check is considered a detection of undesired behavior (i.e., issue/crash detected).

```rust
fn check(
    &self,
    pre_ix: &[SnapshotAccount],
    post_ix: &[SnapshotAccount],
    ix_data: Vec<u8>,
) -> Result<(), FuzzingError> {
    if let Ok(hello_world_account) =
        StoreHelloWorld::deserialize(&mut post_ix[1].data_no_discriminator())
    {
        if hello_world_account.input == 253 {
            return Err(FuzzingError::Custom(1));
        }
    }
    Ok(())
}
```

!!! important

    The order of accounts within the array is the same as the order of accounts in the instruction input of your program.


## Account Deserialization

The `SnapshotAccount` provides methods to obtain parts of the account (e.g., address, its data, owner, etc.).


If you want to deserialize data into a struct defined within your program, ensure that the struct is present in the fuzz test template and has derived the `BorshDeserialize` and `BorshSerialize` traits, as shown in the example below:


```rust
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct StoreHelloWorld {
    recipient: Pubkey,
    input: u8,
}
```



!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
