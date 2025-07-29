# TridentAccounts

The `TridentAccounts` macro is used to derive required methods for account structures.


## Derived Traits

The macro implements the following trait:

- `AccountsMethods` - Methods to resolve accounts

!!! warning "Manual Implementation Note"
    There is no need to manually implement any methods of this trait. The macro handles all implementations automatically based on the account structure and attributes.

## Accounts Methods

!!! warning "Internal Method"
    These methods are used internally by Trident and it is not expected to use them manually.

### `resolve_accounts`

Resolves all accounts based on their constraints and dependencies. The macro automatically analyzes seed dependencies and sorts fields in the correct resolution order.

```rust
fn resolve_accounts(
    &mut self,
    trident: &mut Trident,
    ix_accounts: &mut Self::IxAccounts,
    program_id: Pubkey,
    instruction_data: &Self::IxData,
)
```

---

### `to_account_meta`

Converts all accounts to AccountMeta format for Solana instructions.

```rust
fn to_account_meta(&mut self) -> Vec<AccountMeta>
```

---

### `capture_before`

Captures the state of accounts before transaction execution.

```rust
fn capture_before(&mut self, client: &mut impl FuzzClient)
```

---

### `capture_after`

Captures the state of accounts after transaction execution.

```rust
fn capture_after(&mut self, client: &mut impl FuzzClient)
```

---

## Struct-Level Attributes

These attributes are applied to the struct definition itself.

### `instruction_data`

Specifies the instruction data type that will be used with these accounts.

`This attribute is mandatory and applied at the struct level`

```rust
#[derive(TridentAccounts)]
#[instruction_data(ExampleInstructionData)]
pub struct ExampleAccounts {
    // fields...
}
```

---

### `storage`

Specifies the storage type for accounts, which is used to manage account state during fuzzing.

`This attribute is mandatory and applied at the struct level`

```rust
#[derive(TridentAccounts)]
#[storage(ExampleStorage)]
pub struct ExampleAccounts {
    // fields...
}
```

---

