# Attributes

This macro accepts the following attributes:


### `program_id`

The program to which the Instruction belongs.

`This attribute is mandatory`

```rust
#[derive(Arbitrary, Debug, TridentInstruction)]
#[program_id("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")]
pub struct ExampleInstruction {
    pub accounts: ExampleInstructionAccounts,
    pub data: ExampleInstructionData,
}
```

### `discriminator`

The discriminator of the Instruction.

`This attribute is mandatory`

```rust
#[derive(Arbitrary, Debug, TridentInstruction)]
#[discriminator([33u8, 132u8, 147u8, 228u8, 151u8, 192u8, 72u8, 89u8])]
pub struct ExampleInstruction {
    pub accounts: ExampleInstructionAccounts,
    pub data: ExampleInstructionData,
}
```
