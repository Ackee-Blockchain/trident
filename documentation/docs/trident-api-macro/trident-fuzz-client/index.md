# Trident's Fuzz Client


Trident's Client (`FuzzClient`) is a wrapper around the [TridentSVM](../../trident-svm/index.md) that provides methods to manipulate with the execution environment during fuzzing.

This section describes the methods that are available in the `FuzzClient`.


## Trident Client Methods

### `new_client`

Creates a new instance of the fuzzing client with the specified programs and configuration.
```rust
fn new_client(programs: &[ProgramEntrypoint], config: &TridentConfig) -> Self
```

---

### `deploy_native_program`

Deploys a native program to the fuzzing client.
```rust
fn deploy_native_program(program: ProgramEntrypoint)
```

---

### `get_sysvar`

Retrieves `Sysvars` like `Rent`, `Clock`, etc.
```rust
fn get_sysvar<T: Sysvar>() -> T
```

---

### `warp_to_epoch`

Advances the fuzz test to a specific epoch.
```rust
fn warp_to_epoch(warp_epoch: u64)
```

---

### `warp_to_slot`

Advances the fuzz test to a specific slot.
```rust
fn warp_to_slot(warp_slot: u64)
```

---

### `warp_to_timestamp`

Sets the fuzz test to a specific timestamp.
```rust
fn warp_to_timestamp(warp_timestamp: i64)
```

---

### `forward_in_time`

Advances the fuzz test by the specified number of seconds.
```rust
fn forward_in_time(seconds: i64)
```

---

### `set_account_custom`

Creates or overwrites an account at the specified address, bypassing normal runtime checks.
```rust
fn set_account_custom(address: &Pubkey, account: &AccountSharedData)
```

---

### `get_account`

Retrieves the account data at the specified address.
```rust
fn get_account(key: &Pubkey) -> AccountSharedData
```

---

### `payer`

Returns the keypair of the client's payer account.
```rust
fn payer() -> Keypair
```

---

### `clear_accounts`

Removes temporary accounts created during the fuzzing iteration.

!!! warning
    This method is called internally by Trident (after each fuzzing iteration, to reset state to default).

```rust
fn clear_accounts()
```

---

### `get_last_blockhash`

Retrieves the most recent blockhash.

!!! warning
    Currently, this method is not implemented and call to this method will result in a panic.

```rust
fn get_last_blockhash() -> Hash
```

---

### `process_instructions`

Executes a set of instructions and waits for them to be either finalized or rejected.

!!! warning
    This method is called internally by Trident, no need for manual invocation.

```rust
fn process_instructions(instructions: &[Instruction]) -> Result<(), TransactionError>
```
