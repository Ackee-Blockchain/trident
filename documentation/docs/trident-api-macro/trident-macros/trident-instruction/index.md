# TridentInstruction

The `TridentInstruction` macro is used to derive required methods for `Instructions`.


## Derived trait

The macro implements the `InstructionMethods` trait with the corresponding methods:

!!! warning "Manual Implementation Note"
    There is no need to specify any method of this trait manually.

### `get_discriminator`

Get instruction discriminator

```rust
fn get_discriminator(&self) -> Vec<u8>;
```

---

### `get_program_id`

Get instruction program id

```rust
fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey;
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

### `to_account_metas`

Convert accounts to account metas

```rust
fn to_account_metas(&mut self) -> Vec<AccountMeta>;
```

---

### `resolve_accounts`

Resolve accounts

```rust
fn resolve_accounts(
    &mut self,
    client: &mut impl FuzzClient,
    ix_accounts: &mut Self::IxAccounts,
);
```
