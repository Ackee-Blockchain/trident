# TransactionResult

The `TransactionResult` struct encapsulates the outcome of executing a transaction in the Trident fuzzing environment. It provides methods to inspect transaction success/failure status, access logs, and extract error information.

## Overview

`TransactionResult` is returned by [`process_transaction`](../index.md#process_transaction) and provides access to:

- Transaction success/failure status
- Log messages generated during execution
- Custom program error codes
- Transaction timestamp

## Core Methods

### `is_success`

Returns `true` if the transaction executed successfully.

```rust
pub fn is_success(&self) -> bool
```

**Returns:** `true` if the transaction completed without errors, `false` otherwise.

**Description:** A successful transaction means all instructions were executed without errors and the transaction was committed to the ledger state.

---

### `is_error`

Returns `true` if the transaction failed with an error.

```rust
pub fn is_error(&self) -> bool
```

**Returns:** `true` if the transaction failed, `false` if it succeeded.

**Description:** A failed transaction indicates that one or more instructions encountered an error during execution.

---

### `logs`

Returns the transaction logs.

```rust
pub fn logs(&self) -> String
```

**Returns:** A formatted string containing all log messages in chronological order.

**Description:** Gets all log messages generated during transaction execution, including program logs (from `msg!()` macro), system messages, and error messages, formatted as a single string.

---

### `get_result`

Returns the raw transaction result.

```rust
pub fn get_result(&self) -> &solana_sdk::transaction::Result<()>
```

**Returns:** A reference to the `Result<(), TransactionError>` from the transaction execution.

**Description:** Provides direct access to the underlying Solana transaction result type for advanced error handling.

---

### `get_custom_error_code`

Extracts the custom error code if the transaction failed with a custom error.

```rust
pub fn get_custom_error_code(&self) -> Option<u32>
```

**Returns:**

- `Some(error_code)` - If the transaction failed with a custom program error
- `None` - For other error types or successful transactions

**Description:** If the transaction failed due to a program's custom error, this method returns the numeric error code.

---

### `is_custom_error_with_code`

Checks if the transaction failed with a specific custom error code.

```rust
pub fn is_custom_error_with_code(&self, error_code: u32) -> bool
```

**Parameters:**

- `error_code` - The expected custom error code

**Returns:** `true` if the transaction failed with the specified custom error code.

**Description:** Convenience method to check if the transaction failed with a particular program-defined error code.

---

### `get_transaction_timestamp`

Returns the Unix timestamp when the transaction was processed.

```rust
pub fn get_transaction_timestamp(&self) -> u64
```

**Returns:** Unix timestamp in seconds.

**Description:** The timestamp corresponds to the Clock sysvar's `unix_timestamp` at execution time. Useful for testing time-dependent logic and verifying transaction ordering.

---

## Example Usage

### Basic Transaction Verification

```rust
use trident_fuzz::*;

#[flow]
fn test_transaction(&mut self) {
    let ix = your_program::instruction::initialize(
        self.payer().pubkey(),
    );
    
    let result = self.process_transaction(&[ix], Some("initialize"));
    
    // Check if transaction succeeded
    assert!(result.is_success());
    
    // Access logs
    println!("{}", result.logs());
}
```

### Error Handling

```rust
use trident_fuzz::*;

#[flow]
fn test_error_handling(&mut self) {
    let ix = your_program::instruction::transfer(
        self.payer().pubkey(),
        recipient,
        amount,
    );
    
    let result = self.process_transaction(&[ix], Some("transfer"));
    
    // Check for specific custom error
    if result.is_custom_error_with_code(6000) {
        println!("Transaction failed with InsufficientFunds error");
    }
    
    // Or extract error code
    if let Some(error_code) = result.get_custom_error_code() {
        println!("Custom error code: {}", error_code);
    }
}
```

### Timestamp Verification

```rust
use trident_fuzz::*;

#[flow]
fn test_timestamp(&mut self) {
    let result = self.process_transaction(&instructions, Some("test"));
    let timestamp = result.get_transaction_timestamp();
    
    // Verify transaction occurred after a specific time
    assert!(timestamp >= expected_min_timestamp);
}
```

