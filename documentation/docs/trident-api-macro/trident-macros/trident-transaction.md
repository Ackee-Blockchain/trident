# TridentTransaction

The `TridentTransaction` macro is used to derive required methods for `Transactions`. This procedural macro automatically implements transaction-related functionality for structs that represent transactions.







## Derived Traits

The macro implements the following traits:

- `TransactionGetters` - Methods to retrieve transaction data
- `TransactionSetters` - Methods to set up transaction state

!!! warning "Manual Implementation Note"
    There is no need to manually implement the TransactionGetters and TransactionSetters. The macro handles these implementations automatically based on the structure of your transaction.


## Transaction Getters

!!! warning "Internal Method"
    These methods are used internally by Trident and it is not expected to use them manually.

### `get_transaction_name`

Returns the name of the transaction. If a custom name is provided via the `#[name("...")]` attribute, that name will be used; otherwise, the struct name is used.

```rust
fn get_transaction_name(&self) -> String
```

---

### `get_instruction_discriminators`

Returns the instruction discriminators (identifiers) for all instructions in the transaction.

```rust
fn get_instruction_discriminators(&self) -> Vec<Vec<u8>>
```

---

### `get_instruction_program_ids`

Returns the program IDs that will process the instructions.

```rust
fn get_instruction_program_ids(&self) -> Vec<solana_sdk::pubkey::Pubkey>
```

---

### `get_instruction_data`

Returns the instruction-specific data/parameters for all instructions in the transaction.

```rust
fn get_instruction_data(&mut self, client: &mut impl FuzzClient) -> Vec<Vec<u8>>
```

---

### `get_instruction_accounts`

Returns the account metadata needed for the instructions.

```rust
fn get_instruction_accounts(&mut self, client: &mut impl FuzzClient) -> Vec<Vec<AccountMeta>>
```

---

## Transaction Setters

!!! warning "Internal Method"
    `set_snapshot_before`, `set_snapshot_after` and `set_instructions` methods are used internally by Trident and is not expected to use them manually.

### `build`

Creates a new transaction instance from fuzzer data.

```rust
fn build(trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) -> Self
where
    Self: Default
```

---

### `set_snapshot_before`

Captures the state of all accounts before transaction execution.

```rust
fn set_snapshot_before(&mut self, client: &mut impl FuzzClient)
```

---

### `set_snapshot_after`

Captures the state of all accounts after transaction execution.

```rust
fn set_snapshot_after(&mut self, client: &mut impl FuzzClient)
```

---

### `set_instructions`

Sets up all instructions for the transaction.

!!! warning "Instruction Setup Order"

    The order in which the instruction inputs are set is:

    1. `set_data` - Sets up instruction-specific data
    2. `resolve_accounts` - Resolves account addresses
    3. `set_accounts` - Sets up account metadata
    4. `set_remaining_accounts` - Sets up any additional accounts

```rust
fn set_instructions(
    &mut self,
    trident: &mut Trident,
    fuzz_accounts: &mut Self::IxAccounts,
)
```