# Typed Accounts

If you need to work with `data accounts` in the `Transaction hooks`, you will first need to deserialize the accounts. For deserialization into known structs, Trident generates `types.rs`, where each struct derives `BorshDeserialize` and `BorshSerialize`.


```rust
// types.rs
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct ExampleDataAccount {
    pub data: u64,
}
```


```rust
// example_transaction.rs

#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct ExampleTransaction {
    pub instruction: ExampleInstruction,
}

impl TransactionHooks for ExampleTransaction {
    fn post_transaction(&self, client: &mut impl FuzzClient) {
        // Take account
        let example_account =
            self.instruction.accounts.example_data_account.get_snapshot_after();

        // obtain data without the first 8 bytes (account discriminator) and
        // deserialize the data
        let example_account =
            ExampleDataAccount::deserialize(
                &mut example_account.data_no_discriminator()
            ).unwrap();

        // your additional custom code here ...
    }
}
```
