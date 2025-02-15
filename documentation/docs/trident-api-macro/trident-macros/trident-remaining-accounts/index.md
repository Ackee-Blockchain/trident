# TridentRemainingAccounts

The `TridentRemainingAccounts` macro is used to derive required methods for `remaining account structures`.

The struct must have exactly one field that holds the remaining accounts and is of type `[TridentAccount; X]`, with `X` being the number of remaining accounts, required.

```rust
#[derive(Arbitrary, Debug, TridentRemainingAccounts)]
pub struct ExampleRemainingAccounts {
    pub remaining_accounts: [TridentAccount; 2],
}
```

!!! warning "Dynamic Sizing Behavior"
    The macro will only process accounts until it encounters the first unset remaining account. This allows for dynamic sizing of remaining accounts.


## Derived trait

The macro implements the `AccountsMethods` trait with the corresponding methods:

!!! warning "Manual Implementation Note"
    There is no need to specify any method of this trait manually.

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
