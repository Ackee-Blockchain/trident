# Instructions Sequences

Trident allows you to specify Custom Instruction Sequences you would like to execute.

Possible Instruction sequences are split into 3 parts

- pre-Instructions
- Instructions
- post-Instructions

For example if you program always needs to start with some kind of Initialization instruction, you can specify this Initialize Instruction in `pre_ixs` as shown in the source code below.



!!! tip

    - returning `Ok(vec![])` will result in None Instructions executed in the corresponding part.



```rust
// test_fuzz.rs

// do not forget to include the required structures
use fuzz_instructions::InitVesting;
use fuzz_instructions::WithdrawUnlocked;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(
        u: &mut arbitrary::Unstructured
    ) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix =
            FuzzInstruction::InitVesting(InitVesting::arbitrary(u)?);

        Ok(vec![init_ix])
    }
    fn ixs(
        u: &mut arbitrary::Unstructured
    ) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let withdraw_ix =
            FuzzInstruction::WithdrawUnlocked(WithdrawUnlocked::arbitrary(u)?);

        Ok(vec![withdraw_ix])
    }
    fn post_ixs(
        _u: &mut arbitrary::Unstructured
    ) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(vec![])
    }
}
```


!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
