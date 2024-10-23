# Instructions Sequences

Trident allows you to specify __custom Instruction squences__ you would like to execute.

Possible Instruction sequences are split into 3 parts

- __pre-Instructions__
- __Instructions__
- __post-Instructions__

For example if you program always needs to start with some kind of Initialization instruction, you can specify this Initialize Instruction using the `pre_sequence()` macro as shown in the source code below.

```rust
// test_fuzz.rs

/// ...

struct InstructionsSequence;
/// Define instruction sequences for invocation.
/// `pre` runs at the start, `middle` in the middle, and `post` at the end.
/// For example, to call `InitializeFn`, `UpdateFn` and then `WithdrawFn` during
/// each fuzzing iteration:
/// ```
/// impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
///     pre_sequence!(InitializeFn,UpdateFn);
///     middle_sequence!(WithdrawFn);
///}
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/instructions-sequences/#instructions-sequences
impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
    pre_sequence!(InitializeFn);
    middle_sequence!();
    post_sequence!();
}

/// ...

```

!!! tip

    - The arguments to the macro are variants of `FuzzInstruction` specified in `fuzz_instructions.rs`.
    - Empty macro parameters (such as `middle_sequence!()`), will skip that section, meaning no instructions will be executed during the section.
    - If no macro is defined for a section, a random instruction sequence will be generated for the section.


## Manual trait override

It is not necessary to use the macro as explained above. The trait implementation (i.e., the methods) can be implemented manually, as shown in the code below. This approach allows for greater customization if needed. The rules are the same as described above.

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
