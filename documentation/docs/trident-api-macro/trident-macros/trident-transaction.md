# TridentTransaction

The `TridentTransaction` macro is used to derive required methods for `Transactions`.

## Derived trait

The macro implements the `TransactionMethods` trait with the corresponding methods:

!!! warning "Manual Implementation Note"
    There is no need to specify any method of this trait manually.


### `get_transaction_name`

Get transaction name

```rust
fn get_transaction_name(&self) -> String;
```

---

### `get_instruction_program_ids`

Get instruction program ids

```rust
fn get_instruction_program_ids(&self) -> Vec<solana_sdk::pubkey::Pubkey>;
```

---

### `get_instruction_data`

Get instruction data
```rust
fn get_instruction_data(
    &mut self,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut Self::IxAccounts,
) -> Vec<Vec<u8>>;
```

---

### `get_instruction_accounts`

Get instruction accounts (including the remaining accounts)


!!! warning "Account Resolution Order"
    The method calls `resolve_accounts` first, then `set_accounts` and `set_remaining_accounts`. Giving the second method precedence over the `TridentAccounts` attributes.

```rust
fn get_instruction_accounts(
    &mut self,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut Self::IxAccounts,
) -> Vec<Vec<AccountMeta>>;
```

---

### `set_snapshot_before`

Set account snapshots before transaction
```rust
fn set_snapshot_before(&mut self, client: &mut impl FuzzClient);
```

---

### `set_snapshot_after`

Set account snapshots after transaction
```rust
fn set_snapshot_after(&mut self, client: &mut impl FuzzClient);
```

---

### `process_transaction`

Process the transaction
```rust
fn process_transaction(
    &mut self,
    client: &mut impl FuzzClient,
    config: &TridentConfig,
    fuzz_accounts: &mut Self::IxAccounts,
) -> Result<(), FuzzingError> {}
```

## Attributes


This macro accepts the following attributes:

### `name`

The custom name of the transaction.

`This attribute is optional`

```rust
#[derive(Arbitrary, Debug, TridentTransaction)]
#[name("Custom Transaction Name")]
pub struct ExampleTransaction {
    pub instruction1: ExampleInstruction,
}
```
