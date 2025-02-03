# Customize Instruction Data


Trident allows you to customize instruction data.

Trident by default generates random data for instructions, however, you can customize the data to select specific values.

For example, your Initialize Instruction expects two arguments: `start_at` and `end_at`. You know that in order for the Instruction to make sense, it is required that `start_at` < `end_at`. Moreover, there should be a significant difference between these two. This can be utilized with the **Arbitrary crate**.


```rust
#[derive(Arbitrary, Debug)]
pub struct InitVestingData {
    pub recipient: AccountId,
    #[arbitrary(
        with = |u: &mut arbitrary::Unstructured| u.int_in_range(1..=1_000_000)
    )]
    pub amount: u64,
    // we want start_at smaller than end_at
    // and for testing purposes we can run tests with times from the past
    #[arbitrary(
        with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=1_000_000)
    )]
    pub start_at: u64,
    #[arbitrary(
        with = |u: &mut arbitrary::Unstructured| u.int_in_range(1_001_001..=1_050_000)
    )]
    pub end_at: u64,
    #[arbitrary(
        with = |u: &mut arbitrary::Unstructured| u.int_in_range(1..=1000)
    )]
    pub interval: u64,
}
```

## Implement Arbitrary

There are macros available to use with Arbitrary, however, it is possible to Implement the arbitrary function by yourself.


```rust
// -------------------------------------------------------------------
// -------------------------------------------------------------------
// Implement Arbitrary
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
    // -------------------------------------------------------------------
    // -------------------------------------------------------------------
}
```

!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
