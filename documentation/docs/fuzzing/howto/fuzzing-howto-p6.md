# How to use Custom Data Types

If you use Custom Types as Instruction data arguments, you may encounter a problem that the Custom Type does not implement

- [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html) trait
- [Arbitrary](https://docs.rs/arbitrary/latest/arbitrary/) trait

For example:
```rust

#[program]
pub mod your_program {
    pub fn init_vesting(
            ctx: Context<InitVesting>,
            recipient: Pubkey,
            amount: u64,
            start_at: u64,
            end_at: u64,
            interval: u64,
            // Custom Type input argument
            _input_variant: CustomEnumInput,
    ) -> Result<()> {
        _init_vesting(ctx, recipient, amount, start_at, end_at, interval)
    }
}

...

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum CustomEnumInput {
    InputVariant1,
    InputVariant2,
    InputVariant3,
}
```

Then inside the `fuzz_instructions.rs`, you may see:

```rust
#[derive(Arbitrary, Debug)]
pub struct InitVestingData {
    pub recipient: AccountId,
    pub amount: u64,
    pub start_at: u64,
    pub end_at: u64,
    pub interval: u64,
    /// IMPORTANT:
    /// your_program::CustomEnumInput does not derive
    /// Arbitrary, nor Debug trait
    pub _input_variant: your_program::CustomEnumInput,
}
```

To resolve this issue, you have two options.

## Derive Debug and Arbitrary traits inside your program
This option necessitates updating the source code of your on-chain program, which might be undesirable. If you prefer not to modify your program, consider the alternative option provided below.

Inside `programs/<YOUR_PROGRAM>/Cargo.toml` file include:
```toml
...
[dependencies]
...
arbitrary = "1.3.0"
...
```

and within the program source code:
```rust
// derive Debug, Arbitrary
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Arbitrary)]
pub enum CustomEnumInput {
    InputVariant1,
    InputVariant2,
    InputVariant3,
}
```
## Derive Debug and Arbitrary traits inside the Fuzz Test
You can redefine the custom type within the `fuzz_instructions.rs` file, along with all the necessary traits.
```rust
// Redefine the Custom Type inside the fuzz_instructions.rs,
// but this time with all of the required traits.
#[derive(Arbitrary,Debug, Clone, Copy)]
pub enum CustomEnumInput {
    InputVariant1,
    InputVariant2,
    InputVariant3,
}
```
Then, you would also need to implement the [`std::convert::From<T>`](https://doc.rust-lang.org/std/convert/trait.From.html) trait to enable conversion between the newly defined Custom Type and the Custom Type used within your program.
```rust
// implement std::convert::From to convert between CustomEnumInput
// and your_program::CustomEnumInput as these are distinc Data Types.
impl std::convert::From<CustomEnumInput> for your_program::CustomEnumInput {
    fn from(val: CustomEnumInput) -> Self {
        match val {
            CustomEnumInput::InputVariant1 => {
                your_program::CustomEnumInput::InputVariant1
            }
            CustomEnumInput::InputVariant2 => {
                your_program::CustomEnumInput::InputVariant2
            }
            CustomEnumInput::InputVariant3 => {
                your_program::CustomEnumInput::InputVariant3
            }
        }
    }
}
```
Finally, within the `get_data` function, you can proceed as follows:
```rust
impl<'info> IxOps<'info> for InitVestingData {
    ...
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        // cast variable into correct Data Type
        let variant = self.data._input_variant.into();

        let data = your_program::instruction::InitVestingData {
        ...
        _input_variant:variant,
        ...
        };
        Ok(data)
    }
    ...
}
```

## Example
For a practical example, please refer to the [Examples](../fuzzing-examples.md) section.
