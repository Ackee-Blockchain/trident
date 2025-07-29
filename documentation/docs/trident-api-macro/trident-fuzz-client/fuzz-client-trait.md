# Trident's Fuzz Client


Trident's Client (`FuzzClient`) is a wrapper around the [TridentSVM](../../trident-svm/index.md) that provides methods to manipulate the execution environment during fuzzing.

This section describes the methods that are available in the `FuzzClient`.


## Trident Client Methods

### `new_client`

Creates a new instance of the fuzzing client.

!!! warning "Internal Method"
    This method is internal and should not be called directly.

```rust
fn new_client() -> Self
```

---

### `deploy_entrypoint`

!!! warning "Syscalls Feature Requirement"
    This method is only available if the `syscall-v1` or `syscall-v2` features are enabled.

Deploys a program through its entrypoint.
```rust
fn deploy_entrypoint(&mut self, program: TridentEntrypoint)
```

---

### `deploy_program`

Deploys a program as binary.
```rust
fn deploy_program(&mut self, program: TridentProgram)
```

---

### `get_sysvar`

Retrieves `Sysvars` like `Rent`, `Clock`, etc.
```rust
fn get_sysvar<T: Sysvar>(&self) -> T
```

---

### `warp_to_epoch`

Advances the fuzz test to a specific epoch.
```rust
fn warp_to_epoch(&mut self, warp_epoch: u64)
```

---

### `warp_to_slot`

Advances the fuzz test to a specific slot.
```rust
fn warp_to_slot(&mut self, warp_slot: u64)
```

---

### `warp_to_timestamp`

Sets the fuzz test to a specific timestamp.
```rust
fn warp_to_timestamp(&mut self, warp_timestamp: i64)
```

---

### `forward_in_time`

Advances the fuzz test by the specified number of seconds.
```rust
fn forward_in_time(&mut self, seconds: i64)
```

---

### `set_account_custom`

Creates or overwrites an account at the specified address, bypassing normal runtime checks.
```rust
fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData)
```

---

### `get_account`

Retrieves the account data at the specified address.
```rust
fn get_account(&mut self, key: &Pubkey) -> AccountSharedData
```

---

### `payer`

Returns the keypair of the client's payer account.
```rust
fn payer(&self) -> Keypair
```

---

### `get_last_blockhash`

Retrieves the most recent blockhash.

!!! warning
    Currently, this method is not implemented and a call to this method will result in a panic.

```rust
fn get_last_blockhash(&self) -> Hash
```

---

### `_process_instructions`

Executes a set of instructions and waits for them to be either finalized or rejected.

!!! warning "Internal Method"
    This method is internal and should not be called directly.

```rust
fn _process_instructions(
    &mut self,
    instructions: &[Instruction],
) -> trident_svm::prelude::solana_svm::transaction_processor::LoadAndExecuteSanitizedTransactionsOutput
```

---

### `_clear_accounts`

Removes temporary accounts created during the fuzzing iteration.

!!! warning "Internal Method"
    This method is internal and should not be called directly.

```rust
fn _clear_accounts(&mut self)
```
