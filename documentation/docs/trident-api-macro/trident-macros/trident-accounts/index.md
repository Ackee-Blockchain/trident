# TridentAccounts

The `TridentAccounts` macro is used to derive required methods for `account structures`.



## Derived trait

The macro implements the `AccountsMethods` trait with the corresponding methods:

!!! warning "Manual Implementation Note"
    There is no need to specify any method of this trait manually.

### `resolve_accounts`

This method resolves accounts based on the specified `account` attributes.

```rust
fn resolve_accounts(
    &mut self,
    client: &mut impl FuzzClient,
    ix_accounts: &mut Self::IxAccounts,
);
```

---

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
