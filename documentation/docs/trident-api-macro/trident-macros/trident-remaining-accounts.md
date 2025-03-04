# TridentRemainingAccounts

The `TridentRemainingAccounts` macro is used to derive required methods for `remaining account structures`.

The struct must have exactly one field that holds the remaining accounts and is of type `[TridentAccount; X]`, with `X` being the number of remaining accounts, required.

```rust
#[derive(Arbitrary, Debug, TridentRemainingAccounts)]
pub struct ExampleRemainingAccounts {
    pub remaining_accounts: [TridentAccount; 2],
}
```

## Derived Traits

The macro implements the following trait:

- `RemainingAccountsMethods` - Methods to resolve remaining accounts

!!! warning "Manual Implementation Note"
    There is no need to manually implement any methods of this trait. The macro handles all implementations automatically based on the account structure and attributes.

!!! warning "Dynamic Sizing Behavior"
    The macro will only process accounts until it encounters the first unset remaining account. This allows for dynamic sizing of remaining accounts.


## Remaining Accounts Methods

!!! warning "Internal Method"
    These methods are used internally by Trident and is not expected to use them manually.

The macro implements the `RemainingAccountsMethods` trait with the corresponding methods:

### `to_account_meta`

Convert accounts to account metas
```rust
fn to_account_meta(&mut self) -> Vec<AccountMeta>;
```

---

### `capture_before`

Capture account state before transaction execution
```rust
fn capture_before(&mut self, client: &mut impl FuzzClient);
```

---

### `capture_after`

Capture account state after transaction execution
```rust
fn capture_after(&mut self, client: &mut impl FuzzClient);
```
