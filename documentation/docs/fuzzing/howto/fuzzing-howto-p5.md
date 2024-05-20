# Customize instructions generation

It is possible to customize how the instructions are generated and which instructions will be executed at the beginning (`pre_ixs`), in the middle (`ixs`) and at the end (`post_ixs`) of each fuzz iteration. This can be useful for example if your program needs an initialization or you want to fuzz some specific program state.

- Go to the `trident-tests/fuzz_tests/<FUZZ_TEST_NAME>/test_fuzz.rs` and implement the corresponding optional method of the `FuzzDataBuilder<FuzzInstruction>` trait. For example, in order to always call the `initialize` instruction, modify the trait's implementation as follows:

```rust
impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) ->
        arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        Ok(vec![init_ix])
    }
}
```
