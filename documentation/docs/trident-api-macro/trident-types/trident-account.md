# TridentAccount

Trident Account is a wrapper around `AccountMeta`, `SnapshotAccount` and `AccountId`.

`AccountMeta` is type which is used within `Transaction`, it specifies account address, `isSigner` and `isWritable` flags.

`SnapshotAccount` is Trident's custom type which is used to capture account state before and after the transaction.

`AccountId` is randomly generated number which is used to identify account within its corresponding `AccountStorage`.


```rust
#[derive(Debug, Clone)]
pub struct TridentAccount {
    pub account_id: AccountId,
    account_meta: Option<AccountMeta>,
    snapshot_before: Option<SnapshotAccount>,
    snapshot_after: Option<SnapshotAccount>,
}
```

## Implemented Methods

The following section contains the methods that are available for the `TridentAccount` struct.

### `set_account_meta`

Sets the account metadata with specified address and flags.
```rust
fn set_account_meta(&mut self, address: Pubkey, is_signer: bool, is_writable: bool)
```

---

### `get_snapshot_before`

Returns the account snapshot taken before transaction execution.
```rust
fn get_snapshot_before(&self) -> &SnapshotAccount
```

---

### `get_snapshot_after`

Returns the account snapshot taken after transaction execution.
```rust
fn get_snapshot_after(&self) -> &SnapshotAccount
```

---

### `set_is_signer`

Sets the account as a signer.
```rust
fn set_is_signer(&mut self)
```

---

### `set_is_writable`

Sets the account as writable.
```rust
fn set_is_writable(&mut self)
```

---

### `set_address`

Sets the account's address.
```rust
fn set_address(&mut self, address: Pubkey)
```

---

### `pubkey`

Returns the account's public key.
```rust
fn pubkey(&self) -> Pubkey
```

---

### `capture_before`

!!! warning "Internal Method"
    This method is internal and should not be used directly.

Captures the account state before transaction execution.
```rust
fn capture_before(&mut self, client: &mut impl FuzzClient)
```

---

### `capture_after`

!!! warning "Internal Method"
    This method is internal and should not be used directly.

Captures the account state after transaction execution.
```rust
fn capture_after(&mut self, client: &mut impl FuzzClient)
```


---

### `to_account_meta`

!!! warning "Internal Method"
    This method is internal and should not be used directly.

Returns the `AccountMeta` if set, panics if not set.
```rust
fn to_account_meta(&self) -> AccountMeta
```


---

### `is_account_meta_set`

!!! warning "Internal Method"
    This method is internal and should not be used directly.

Returns whether the account meta has been set.
```rust
fn is_account_meta_set(&self) -> bool
```
