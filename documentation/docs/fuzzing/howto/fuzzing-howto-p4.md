# Define invariants checks

After each successful instruction execution, the `check()` method is called to check the account data invariants.

For each instruction, you can compare the account data **before** and **after** the instruction execution such as:

```rust
fn check(
    &self,
    pre_ix: Self::IxSnapshot,
    post_ix: Self::IxSnapshot,
    _ix_data: Self::IxData,
) -> Result<(), &'static str> {
    if let Some(escrow_pre) = pre_ix.escrow {
        // we can unwrap the receiver account because it
        // has to be initialized before the instruction
        // execution and it is not supposed to be closed
        // after the instruction execution either
        let receiver = pre_ix.receiver.unwrap();
        let receiver_lamports_before = receiver.lamports();
        let receiver_lamports_after = post_ix.receiver.unwrap().lamports();

        if receiver.key() != escrow_pre.receiver.key()
            && receiver_lamports_before < receiver_lamports_after
        {
            return Err("Un-authorized withdrawal");
        }
    }

    Ok(())
}
```
