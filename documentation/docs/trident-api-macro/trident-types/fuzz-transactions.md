# FuzzTransactions

`FuzzTransactions` is an enum that contains `all the transactions that can be used for random selection sequence`, if default callback is used.

By default, Trident generates the enum based on the program's idl, i.e. for each instruction Trident generates a transaction which contains the instruction.

On demand, you can add your own transactions to the enum.

```rust
#[derive(Arbitrary, FuzzTestExecutor)]
pub enum FuzzTransactions {
    Transaction1(Transaction1),
    Transaction2(Transaction2),
    Transaction3(Transaction3),
    Transaction4(Transaction4),
    Transaction5(Transaction5),
    Transaction6(Transaction6),
    Transaction7(Transaction7),
    Transaction8(Transaction8),
    Transaction9(Transaction9),
    Transaction10(Transaction10),
    Transaction11(Transaction11),
    /// ...
}
```
