# Constructing Instructions

Trident allows you to construct instructions similarly to tests written using JavaScript and Anchor framework tests.

The following source code shows an example of how a transaction can be constructed:


```rust
// Obtain addresses from address storages
let author = self.fuzz_accounts.author.insert(&mut self.trident, None);

let hello_world = self.fuzz_accounts.hello_world_account.insert(
    &mut self.trident,
    Some(PdaSeeds {
        seeds: &[b"hello_world_seed"],
        program_id: hello_world::program_id(),
    }),
);

// Obtain random input
let input = self.trident.random_from_range(0..u8::MAX);

// Construct instruction
let ix = InitializeFnInstruction::data(InitializeFnInstructionData::new(input))
    .accounts(InitializeFnInstructionAccounts::new(author, hello_world))
    .instruction();

// Execute transaction
let res = self.trident.process_transaction(&[ix], Some("Initialize"));
```

!!! warning "Available Types"

    All types used for instruction construction are generated automatically by Trident and stored within the `types.rs` file.

Constructing instructions consists of two steps:


## Setting Up Data

The first step is to set up the data for the instruction using the `data` method. In the example above, we use the `random_from_range` method to generate a random number between 0 and `u8::MAX`, then create the instruction data using the `InitializeFnInstructionData::new` method.

```rust
let input = self.trident.random_from_range(0..u8::MAX);

let ix = InitializeFnInstruction::data(InitializeFnInstructionData::new(input))
```

## Setting Up Accounts

The second step is to set up the accounts for the instruction using the `accounts` method. In the example above, we use the `InitializeFnInstructionAccounts::new` method to create the instruction accounts.

### Address Storages

Address storages are important containers for managing addresses used in your tests:

- When you call `insert()` without seeds, it adds a random address to the storage
- When you call `insert()` with specified seeds, it inserts a Program Derived Address (PDA) to the storage
- When you call `get()` method, it returns a random address from the storage
- Storages allow you to create test scenarios where some addresses are correct and others are incorrect, enabling comprehensive access pattern testing


```rust
// Obtain addresses from address storages
let author = self.fuzz_accounts.author.insert(&mut self.trident, None);

let hello_world = self.fuzz_accounts.hello_world_account.insert(
    &mut self.trident,
    Some(PdaSeeds {
        seeds: &[b"hello_world_seed"],
        program_id: hello_world::program_id(),
    }),
);

// Construct instruction with accounts
let ix = InitializeFnInstruction::data(InitializeFnInstructionData::new(input))
    .accounts(InitializeFnInstructionAccounts::new(author, hello_world))
    .instruction();
```


## Setting Up Remaining Accounts

Remaining accounts are accounts that are not specified in the instruction accounts. You can specify remaining accounts using the `remaining_accounts` method.


```rust
// Obtain remaining accounts
let remaining_accounts = vec![
    AccountMeta::new(pubkey!("11111111111111111111111111111111"), false),
];

let ix = InitializeFnInstruction::data(InitializeFnInstructionData::new(input))
    .accounts(InitializeFnInstructionAccounts::new(author, hello_world))
    .remaining_accounts(remaining_accounts)
    .instruction();
```