# TridentTransactionResult

The `TridentTransactionResult` struct encapsulates the outcome of executing a transaction in the Trident fuzzing environment. It provides methods to inspect transaction success/failure status, access logs, extract error information, inspect CPI inner instructions, and read Solana return data.

## Overview

`TridentTransactionResult` is returned by [`process_transaction`](../index.md#process_transaction) and provides access to:

- Transaction success/failure status
- Raw transaction error via `status()`
- Log messages generated during execution
- Compute units consumed
- Inner instructions (resolved CPIs with actual pubkeys)
- Return data emitted by the executed program
- Transaction timestamp
- Detection of program panics (`ProgramFailedToComplete`)

## Core Methods

### `is_success`

Returns `true` if the transaction executed successfully.

```rust
pub fn is_success(&self) -> bool
```

**Returns:** `true` if the transaction completed without errors, `false` otherwise.

**Description:** A successful transaction means all instructions were executed without errors and the transaction was committed to the ledger state.

---

### `status`

Returns a reference to the raw transaction result.

```rust
pub fn status(&self) -> &Result<(), TransactionError>
```

**Returns:** A reference to the `Result<(), TransactionError>` from the transaction execution.

**Description:** Provides direct access to the underlying Solana transaction result type for advanced error handling and pattern matching on specific `TransactionError` variants.

---

### `logs`

Returns the transaction logs as a formatted string.

```rust
pub fn logs(&self) -> String
```

**Returns:** A formatted string containing all log messages in chronological order.

**Description:** Gets all log messages generated during transaction execution, including program logs (from `msg!()` macro), system messages, and error messages.

---

### `compute_units_consumed`

Returns the number of compute units consumed by the transaction.

```rust
pub fn compute_units_consumed(&self) -> u64
```

**Returns:** The total compute units consumed during execution.

**Description:** Useful for monitoring program efficiency and ensuring transactions stay within compute budget limits.

---

### `inner_instructions`

Returns the resolved inner instructions (CPIs) for the transaction.

```rust
pub fn inner_instructions(&self) -> Option<&ResolvedInnerInstructionsList>
```

**Returns:**

- `Some(&ResolvedInnerInstructionsList)` - If the transaction produced inner instructions
- `None` - If no inner instructions were generated

**Description:** Inner instructions represent cross-program invocations (CPIs) made during transaction execution. Unlike raw Solana inner instructions that use index-based references, these are **resolved** with actual `Pubkey` values for the program and accounts involved.

Each `ResolvedInnerInstruction` contains:

- `program_id: Pubkey` - The program that was invoked
- `accounts: Vec<Pubkey>` - The accounts passed to the CPI
- `data: Vec<u8>` - The instruction data
- `stack_height: u8` - The CPI depth level

---

### `return_data`

Returns the raw Solana return data for the transaction, if any program emitted it via `set_return_data`.

```rust
pub fn return_data(&self) -> Option<&TransactionReturnData>
```

**Returns:**

- `Some(&TransactionReturnData)` - If a program emitted return data during execution
- `None` - If no return data was set

**Description:** The returned value contains the emitting `program_id` and the raw returned `data` bytes.

---

### `transaction_timestamp`

Returns the Unix timestamp when the transaction was processed.

```rust
pub fn transaction_timestamp(&self) -> i64
```

**Returns:** Unix timestamp in seconds.

**Description:** The timestamp corresponds to the Clock sysvar's `unix_timestamp` at execution time. Useful for testing time-dependent logic and verifying transaction ordering.

---

### `is_program_failed_to_complete`

Checks if the transaction failed because a program panicked or aborted.

```rust
pub fn is_program_failed_to_complete(&self) -> bool
```

**Returns:** `true` if the transaction error is `TransactionError::InstructionError(_, InstructionError::ProgramFailedToComplete)`.

**Description:** This typically indicates the on-chain program panicked during execution (e.g., due to an `unwrap()` on an invalid value or an explicit `panic!()`). Trident uses this internally for metrics to track program panics separately from other transaction failures.

---

## Example Usage

### Basic Transaction Verification

```rust
#[flow]
fn test_transaction(&mut self) {
    let ix = your_program::instruction::initialize(
        self.trident.payer().pubkey(),
    );

    let result = self.trident.process_transaction(&[ix], Some("initialize"));

    if result.is_success() {
        // transaction succeeded, check state
    }

    // Access logs
    println!("{}", result.logs());
}
```

### Error Handling with Status

```rust
#[flow]
fn test_error_handling(&mut self) {
    let ix = your_program::instruction::transfer(
        self.trident.payer().pubkey(),
        recipient,
        amount,
    );

    let result = self.trident.process_transaction(&[ix], Some("transfer"));

    match result.status() {
        Ok(()) => {
            // transaction succeeded
        }
        Err(transaction_error) => {
            println!("Transaction failed: {}", transaction_error);
        }
    }
}
```

### Inspecting Compute Units

```rust
#[flow]
fn test_compute_budget(&mut self) {
    let result = self.trident.process_transaction(&[ix], Some("heavy_compute"));

    let cu = result.compute_units_consumed();
    println!("Compute units used: {}", cu);

    invariant!(
        cu < 200_000,
        "Transaction consumed too many compute units: {}",
        cu
    );
}
```

### Inspecting Inner Instructions (CPIs)

```rust
#[flow]
fn test_cpi_calls(&mut self) {
    let result = self.trident.process_transaction(&[ix], Some("with_cpi"));

    if let Some(inner_ixs) = result.inner_instructions() {
        for (i, top_level_cpis) in inner_ixs.iter().enumerate() {
            for cpi in top_level_cpis {
                println!(
                    "Instruction {} CPI -> program: {}, stack_height: {}",
                    i, cpi.program_id, cpi.stack_height
                );
            }
        }
    }
}
```

### Timestamp Verification

```rust
#[flow]
fn test_timestamp(&mut self) {
    let result = self.trident.process_transaction(&[ix], Some("test"));
    let timestamp = result.transaction_timestamp();

    invariant!(
        timestamp >= expected_min_timestamp,
        "Transaction timestamp {} is before expected {}",
        timestamp,
        expected_min_timestamp
    );
}
```
