# Transactions Sequences

Transactions sequences are a way to group transactions together. They are useful when you want to fuzz test a sequence of transactions, or when you want to fuzz test a transaction that depends on the previous one.

!!! warning "Sequence Method Behavior"
    Pay attention to these three cases which can occur:

    - If the `*_sequence` method is not defined, random transactions will be executed.
    - If the `*_sequence` method is defined with a custom sequence, this sequence will be executed.
    - If the `*_sequence` method is defined, but the method returns an empty vector, no transactions will be executed for that part.

```rust
impl FuzzSequenceBuilder<FuzzTransactions> for TransactionsSequence {
    fn starting_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        // Init Lending Market
        let sq1 = sequence!(
            [
                Transaction1,
                Transaction2,
                Transaction3
            ],
            fuzzer_data
        );

        Ok(sq1)
    }
    fn middle_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        let sq1 = sequence!(
            [
                Transaction4,
                Transaction5,
                Transaction6,
                Transaction7,
            ],
            fuzzer_data
        );

        Ok(sq1)
    }
    fn ending_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        Ok(vec![])
    }
}
```

More about transaction sequences can be found in the [Transaction Flows](../../../trident-advanced/trident-transactions/trident-fuzzing-flows/index.md) section.
