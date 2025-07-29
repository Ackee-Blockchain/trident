# Transaction Hooks

These methods can be overridden to customize transaction behavior during fuzzing.

### `pre_transaction`

Override this method to perform custom actions before the transaction is executed.

```rust
fn pre_transaction(&self, client: &mut impl FuzzClient) {
    // Custom pre-transaction logic
    // e.g., set up accounts, perform checks, etc.
}
```

---

### `post_transaction`

Override this method to perform custom actions after the transaction is executed.

```rust
fn post_transaction(&self, client: &mut impl FuzzClient) {
    // Custom post-transaction logic
    // e.g., cleanup, additional checks, etc.
}
```

---

### `transaction_invariant_check`

Override this method to perform custom invariant checks on the transaction results.

```rust
fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
    // Custom invariant checks
    // Return Err(FuzzingError) if invariants are violated
    Ok(())
}
```

**Returns:**

- `Ok(())` if all invariants pass
- `Err(FuzzingError)` if any invariant is violated

---

### `transaction_error_handler`

Override this method to handle transaction errors with custom logic.

```rust
fn transaction_error_handler(&self, error: TransactionError) {
    // Custom error handling logic
    // e.g., logging, state cleanup, etc.
}
```

---