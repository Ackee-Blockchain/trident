# Transaction Flows

Transaction flows are a way to define `a sequence of transactions` that will be fuzzed. Flows are important in order to focus on some specific transaction logic, instead of random transaction sequences.

By default there are no flows defined by Trident, so random transaction sequences will be fuzzed (random enum variants are taken from the `FuzzTransactions` enum).

In order to define custom flow, Trident provides three methods:

`starting_sequence`

`middle_sequence`

`ending_sequence`


Each method works with its corresponding random date.

!!! warning "Sequence Method Behavior"
    Pay attention to these three cases which can occur:

    - If the `*_sequence` method is not defined, random transactions will be executed.
    - If the `*_sequence` method is defined with a custom sequence, this sequence will be executed.
    - If the `*_sequence` method is defined, but the method returns an empty vector, no transactions will be executed for that part.

```rust
// test_fuzz.rs
impl FuzzSequenceBuilder<FuzzTransactions> for TransactionsSequence {
    fn starting_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        // starting sequence is empty, so nothing will be
        // executed at the start of each fuzzing iteration
        Ok(vec![])
    }
    fn middle_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        // in the middle of each fuzzing iteration,
        // this sequence will be executed
        let flow1 = sequence!(
            [
                Transaction1,
                Transaction2,
                Transaction3
            ],
            fuzzer_data
        );

        Ok(flow1)
    }

    // ending sequence is not defined, so at the end of each fuzzing
    // iteration, random transactions will be executed
}
```

```rust
// test_fuzz.rs
impl FuzzSequenceBuilder<FuzzTransactions> for TransactionsSequence {
    // the middle and ending sequences return empty vectors, which means
    // that during every fuzzing iteration only Transaction1 and Transaction2
    // will be executed in this given order
    fn starting_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        let flow1 = sequence!(
            [
                Transaction1,
                Transaction2,
            ],
            fuzzer_data
        );

        Ok(flow1)
    }
    fn middle_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        Ok(vec![])
    }
    fn ending_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        Ok(vec![])
    }
}
```
