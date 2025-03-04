# TridentTransaction

The `TridentTransaction` macro is used to derive required methods for `Transactions`. This procedural macro automatically implements transaction-related functionality for structs that represent transactions.

## Derived Traits

The macro implements the following traits:

- `TransactionGetters` - Methods to retrieve transaction data
- `TransactionSetters` - Methods to set up transaction state
- `TransactionMethods` - Core transaction execution methods

!!! warning "Manual Implementation Note"
    There is no need to manually implement the getter, setter, or core methods. The macro handles these implementations automatically based on the structure of your transaction.

## Transaction Methods

### `build`

Creates a new transaction instance from fuzzer data.

```rust
fn build(
    fuzzer_data: &mut FuzzerData,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut Self::IxAccounts,
) -> arbitrary::Result<Self>
```

---

### `execute`

Execute the transaction with the TransactionHooks.

```rust
fn execute(&mut self, client: &mut impl FuzzClient) -> Result<(), FuzzingError>
```

---

### `execute_no_hooks`

Execute the transaction without the TransactionHooks (simplified version).

```rust
fn execute_no_hooks(&mut self, client: &mut impl FuzzClient) -> Result<(), TransactionError>
```

---

## Transaction Getters

!!! warning "Internal Method"
    These methods are used internally by Trident and is not expected to use them manually.

### `get_transaction_name`

Returns the name of the transaction. If a custom name is provided via the `#[name("...")]` attribute, that name will be used; otherwise, the struct name is used.

```rust
fn get_transaction_name(&self) -> String
```

---

### `get_instruction_discriminators`

Returns the instruction discriminators (identifiers) for all instructions in the transaction.

```rust
fn get_instruction_discriminators(&self) -> Vec<Vec<u8>>
```

---

### `get_instruction_program_ids`

Returns the program IDs that will process the instructions.

```rust
fn get_instruction_program_ids(&self) -> Vec<solana_sdk::pubkey::Pubkey>
```

---

### `get_instruction_data`

Returns the instruction-specific data/parameters for all instructions in the transaction.

```rust
fn get_instruction_data(&mut self, client: &mut impl FuzzClient) -> Vec<Vec<u8>>
```

---

### `get_instruction_accounts`

Returns the account metadata needed for the instructions.

```rust
fn get_instruction_accounts(&mut self, client: &mut impl FuzzClient) -> Vec<Vec<AccountMeta>>
```

---

## Transaction Setters

!!! warning "Internal Method"
    These methods are used internally by Trident and is not expected to use them manually.

### `set_snapshot_before`

Captures the state of all accounts before transaction execution.

```rust
fn set_snapshot_before(&mut self, client: &mut impl FuzzClient)
```

---

### `set_snapshot_after`

Captures the state of all accounts after transaction execution.

```rust
fn set_snapshot_after(&mut self, client: &mut impl FuzzClient)
```

---

### `set_instructions`

Sets up all instructions for the transaction.

!!! warning "Instruction Setup Order"

    The order in which the instructions inputs are set are:

    1. `set_data` - Sets up instruction-specific data
    2. `resolve_accounts` - Resolves account addresses
    3. `set_accounts` - Sets up account metadata
    4. `set_remaining_accounts` - Sets up any additional accounts

```rust
fn set_instructions(
    &mut self,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut Self::IxAccounts,
)
```

---

## Struct-Level Attributes

These attributes are applied to the struct definition itself.

### `name`

The custom name of the transaction. This is optional - if not provided, the struct name will be used.

```rust
#[derive(Arbitrary, Debug, TridentTransaction)]
#[name("Custom Transaction Name")]
pub struct ExampleTransaction {
    pub instruction1: ExampleInstruction,
}
```
