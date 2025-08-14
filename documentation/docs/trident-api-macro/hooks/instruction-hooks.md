# Instruction Hooks

These methods can be overridden to customize instruction behavior during fuzzing.

### `set_data`

Override this method to customize how instruction data is set during fuzzing.

```rust
fn set_data(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
    // Custom data setting logic
}
```

---

### `set_accounts`

Override this method to customize how instruction accounts are set during fuzzing.

```rust
fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
    // Custom account setting logic
}
```

---

### `set_remaining_accounts`

Override this method to customize how remaining accounts are set during fuzzing.

```rust
fn set_remaining_accounts(
    &mut self,
    trident: &mut Trident,
    fuzz_accounts: &mut Self::IxAccounts,
) {
    // Custom remaining accounts setting logic
}
```

---