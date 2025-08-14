# Trident

The `Trident` struct is the main orchestrator for fuzzing operations. It manages the fuzzing client, random number generation, and provides methods for executing transactions and collecting metrics.

```rust
pub struct Trident {
    client: TridentSVM,
    fuzzing_data: TridentFuzzingData,
    rng: TridentRng,
}
```

## Core Methods

### `execute_transaction`

Executes a transaction with all transaction hooks enabled, including pre/post transaction hooks and error handling.

```rust
pub fn execute_transaction<T>(
    &mut self,
    transaction: &mut T,
    transaction_name_override: Option<&str>,
) where
    T: TransactionHooks + TransactionGetters + TransactionSetters + TransactionPrivateMethods + std::fmt::Debug
```

**Parameters:**
- `transaction` - The transaction to execute
- `transaction_name_override` - Optional custom name for the transaction (used in metrics)

---

### `get_client`

Returns a mutable reference to the underlying fuzzing client.

```rust
pub fn get_client(&mut self) -> &mut impl FuzzClient
```

---

### `airdrop`

Adds lamports to the specified account address.

```rust
pub fn airdrop(&mut self, address: &Pubkey, amount: u64)
```

**Parameters:**
- `address` - The account to receive the lamports
- `amount` - The number of lamports to add

---

## Random Number Generation Methods

### `gen_range`

Generates a random number within the specified range.

```rust
pub fn gen_range<T, R>(&mut self, range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>
```

---

### `gen_pubkey`

Generates a random public key.

```rust
pub fn gen_pubkey(&mut self) -> Pubkey
```

---

### `gen_string`

Generates a random string of the specified length.

```rust
pub fn gen_string(&mut self, length: usize) -> String
```

---

### `fill_bytes`

Fills the provided byte array with random data.

```rust
pub fn fill_bytes(&mut self, bytes: &mut [u8])
```

---

## Metrics Methods

### `add_histogram_metric`

Adds a value to a histogram metric. Only records if the fuzzing metrics are enabled.

```rust
pub fn add_histogram_metric(&mut self, metric_name: &str, value: f64)
```

---

### `add_accumulator_metric`

Adds a value to an accumulator metric. Only records if the fuzzing metrics are enabled.

```rust
pub fn add_accumulator_metric(&mut self, metric_name: &str, value: f64)
```

---

## Regression Testing Methods

### `add_to_regression`

Adds an account to the regression test. Only records if the fuzzing regression is enabled.

```rust
pub fn add_to_regression(&mut self, account: &Pubkey, account_name: &str)
```

---
