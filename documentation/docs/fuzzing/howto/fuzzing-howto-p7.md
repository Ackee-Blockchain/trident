# How to use Arbitrary crate

The [Arbitrary](https://docs.rs/arbitrary/latest/arbitrary/) crate in Rust is used for generating well-typed, structured instances of data from raw byte buffers, making it useful for fuzzing by producing random but structured data for tests.

By implementing the [Arbitrary](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait for Instruction Data structures, you can guide the fuzzing tool to generate meaningful instances of these structures, thus ensuring a more effective and targeted fuzzing process.



Let`s say your Solana program contains instruction, with a similar logic as the example below:
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
    ) -> Result<()> {
        _init_vesting(ctx, recipient, amount, start_at, end_at, interval)
    }
}
...
pub fn _init_vesting(
    ctx: Context<InitVesting>,
    recipient: Pubkey,
    amount: u64,
    start_at: u64,
    end_at: u64,
    interval: u64,
) -> Result<()> {
    ...
    // the Instruction Data arguments are not completely random
    // and should have the following restrictions
    require!(amount > 0, VestingError::InvalidAmount);
    require!(end_at > start_at, VestingError::InvalidTimeRange);
    require!(end_at - start_at > interval, VestingError::InvalidInterval);
    require!(interval > 0, VestingError::InvalidInterval);
    ...

}
```

For this purpose, you can limit the generated Instruction Data structure that is sent to the instruction by the fuzzer, in the following ways

## Customizing single fields
You can limit the generated Instruction Input Data by customizing particular fields, Check [Customizing single fields](https://github.com/rust-fuzz/arbitrary?tab=readme-ov-file#customizing-single-fields) for more details.

For the example specified above, with the customization we can limit the particular fields such that
```rust
// Instruction Data structure automatically generated
// inside fuzz_instructions.rs
...
#[derive(Arbitrary, Debug)]
pub struct InitVestingData {
    pub recipient: AccountId,
    // specify the range for amount
    #[arbitrary(
        with = |u: &mut arbitrary::Unstructured| u.int_in_range(1..=1_000_000)
    )]
    pub amount: u64,
    // specify the range for start_at , this way it will
    // be always smaller than end_at
    #[arbitrary(
        with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1_000_000)
    )]
    pub start_at: u64,
    // specify the range for end_at
    #[arbitrary(
        with = |u: &mut arbitrary::Unstructured|
        u.int_in_range(1_001_001..=1_050_000)
    )]
    pub end_at: u64,
    // specify the range for interval
    #[arbitrary(
        with = |u: &mut arbitrary::Unstructured| u.int_in_range(1..=1000)
    )]
    pub interval: u64,
}
...
```

## Implementing Arbitrary By Hand
Alternatively, you can write Arbitrary implementation by yourself.
```rust
// for the given example above, this structure is automatically generated
// with the fuzzer inside fuzz_instructions.rs
#[derive(Arbitrary, Debug)]
pub struct InitVestingData {
    pub recipient: AccountId,
    pub amount: u64,
    pub start_at: u64,
    pub end_at: u64,
    pub interval: u64,
}
```

Now, instead of using an automatically derived Arbitrary trait, you can implement the trait by hand
```rust
...
#[derive(Debug)]
pub struct InitVestingData {
    pub recipient: AccountId,
    pub amount: u64,
    pub start_at: u64,
    pub end_at: u64,
    pub interval: u64,
}
impl<'a> Arbitrary<'a> for InitVestingData {
    fn arbitrary(
        u: &mut arbitrary::Unstructured<'a>
    ) -> arbitrary::Result<Self> {
        // obtain AccountId
        let recipient = AccountId::arbitrary(u)?;

        // limit the generated amount to the 1_000_000
        let amount = u.int_in_range(1..=1_000_000)?;

        // now we want to obtain
        // - start_at
        // - end_at
        // - interval
        // however we want to limit the data such that:
        // - start_at < end_at
        // - end_at - start_at > interval
        // - interval has lower limit of 500 and upper limit of 1000.

        let start_at: u64 = u.int_in_range(1_000_000..=5_000_000)?;
        let end_at: u64 = u.int_in_range(1_000_000..=5_000_000)?;
        let interval: u64 = u.int_in_range(500..=1000)?;

        // ensure that start_at < end_at
        if start_at >= end_at {
            return Err(arbitrary::Error::IncorrectFormat);
        }

        // ensure that end_at - start_at > interval
        match end_at.checked_sub(start_at) {
            Some(diff) => {
                if diff <= interval {
                    return Err(arbitrary::Error::IncorrectFormat);
                }
            }
            None => return Err(arbitrary::Error::IncorrectFormat),
        }

        Ok(InitVestingData {
            recipient,
            amount,
            start_at,
            end_at,
            interval,
        })
    }
}
...
```
## Example
For a practical example, please refer to the [Examples](../fuzzing-examples.md) section.
