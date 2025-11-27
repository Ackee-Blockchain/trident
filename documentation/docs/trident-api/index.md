# Trident

The `Trident` struct is the main orchestrator for fuzzing operations. It manages the fuzzing client, random number generation, and provides methods for executing transactions and collecting metrics.

```rust
pub struct Trident {
    client: TridentSVM,
    fuzzing_data: TridentFuzzingData,
    rng: TridentRng,
}
```

## Core Transaction Processing Methods

### `process_transaction`

Processes a transaction with the given instructions and returns the result with logs.

```rust
pub fn process_transaction(
    &mut self,
    instructions: &[Instruction],
    log_as: Option<&str>,
) -> TransactionResult
```

**Parameters:**

- `instructions` - Array of instructions to execute in the transaction
- `log_as` - Optional name for the transaction (used in metrics and logging). If `None`, the transaction will not be logged.

**Returns:** [`TransactionResult`](./transaction-result/index.md) containing success/failure status and transaction logs.

**Description:** Executes a transaction containing one or more instructions and returns the results. Use this to test your program's instructions with various inputs. Provide a transaction name to enable metrics collection and logging for that transaction. See [TransactionResult](./transaction-result/index.md) for available methods to inspect the transaction outcome.

---

### `deploy_program`

Deploys a program to the fuzzing environment.

```rust
pub fn deploy_program(&mut self, program: TridentProgram)
```

**Parameters:**

- `program` - The program binary and metadata to deploy

**Description:** Makes a program available for testing by deploying it to the fuzzing environment.

---

### `deploy_entrypoint` (syscall-v2 feature)

Deploys an entrypoint program to the fuzzing environment.

```rust
#[cfg(feature = "syscall-v2")]
pub fn deploy_entrypoint(&mut self, program: TridentEntrypoint)
```

**Parameters:**

- `program` - The entrypoint program to deploy

**Description:** Makes an entrypoint program available for testing when using the syscall-v2 feature.

---

## Not Yet Implemented

### `get_last_blockhash`

```rust
pub fn get_last_blockhash(&self) -> Hash
```

**Status:** TODO - Not yet implemented for TridentSVM.

---

## Method Categories

### Specialized Method Groups

- **[Account Management](./account-management.md)** - Account data retrieval, manipulation, and lamport management
- **[Time & Clock](./time-clock.md)** - Time manipulation and clock-related operations for testing time-dependent logic
- **[Random Generation](./random-generation.md)** - Generate random data for comprehensive fuzz testing
- **[Metrics & Regression](./metrics-regression.md)** - Collect metrics and track account states for regression testing

### Native Program Methods

- **[System Program Methods](./system-program.md)** - Account creation, allocation, assignment, and SOL transfers

### Feature-Gated Methods

- **[SPL Token Methods](./spl-token.md)** - Available with `token` feature
- **[Token 2022 Methods](./token-2022.md)** - Available with `token` feature  
- **[Vote Program Methods](./vote-program.md)** - Available with `vote` feature
- **[Stake Program Methods](./stake-program.md)** - Available with `stake` feature

## Getting Started

For most fuzz tests, you'll primarily use:

1. **`process_transaction`** - Execute your program instructions
2. **[Random generation methods](./random-generation.md)** - Create varied test inputs
3. **[Account management methods](./account-management.md)** - Set up and inspect account states

## Example Usage

```rust
use trident_fuzz::*;

#[flow]
fn basic_fuzz_example(&mut self) {
    // Generate random test data
    let amount = self.random_from_range(1..1000u64);
    let recipient = self.random_pubkey();
    
    // Create and execute transaction with logging
    let transfer_ix = self.transfer(&self.payer().pubkey(), &recipient, amount);
    let result = self.process_transaction(&[transfer_ix], Some("Transfer Test"));
    
    // Verify results
    assert!(result.is_success());
    
    // Or execute without logging (pass None)
    let result = self.process_transaction(&[transfer_ix], None);
}
```
