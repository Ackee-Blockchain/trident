# Random Generation Methods

These methods provide functionality for generating random data in your fuzz tests, enabling comprehensive testing with varied inputs.

## Random Value Generation

### `random_from_range`

Generates a random number within the specified range using **uniform distribution**.

```rust
pub fn random_from_range<T, R>(&mut self, range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>
```

**Parameters:**

- `range` - The range to generate the random number from

**Returns:** Random value within the specified range.

**Description:** Generates a random value of the specified type within the given range using uniform distribution (all values have equal probability). Supports various numeric types.

---

### `random_log_uniform`

Generates a random number with **log-uniform distribution**.

```rust
pub fn random_log_uniform<T: LogUniform>(&mut self) -> T
```

**Supported types:** `u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`

**Returns:** Random value with log-uniform distribution.

**Description:** With uniform distribution on u64, there are only 1000 numbers below 1000, but 9 quintillion above 2^63 - so 99.99% of samples are astronomically large. Log-uniform fixes this by giving equal probability to each **order of magnitude**: the range 1-10 is as likely as 1M-10M or 1B-10B. This ensures you actually test small, medium, AND large values.

**Example:**

```rust
// Equal probability of getting values in ranges:
// 0-255, 256-65535, 65536-4294967295, etc.
let amount: u64 = trident.random_log_uniform();
```

---

### `random_biased`

Generates a random number with a **biased distribution optimized for fuzzing**.

```rust
pub fn random_biased<T: BiasedValue>(&mut self) -> T
```

**Supported types:** `u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`

**Returns:** Random value with bias toward edge cases and interesting values.

**Description:** Combines multiple strategies for maximum bug-finding potential:

| Probability | Strategy | Description |
|-------------|----------|-------------|
| 25% | Exact interesting | Boundaries (0, MAX), powers of 2, type limits |
| 25% | Near interesting | ±5% offset from interesting values |
| 25% | Log-uniform | Equal probability per magnitude |
| 25% | Uniform random | Full range exploration |

The "near interesting" offset scales with the value (percentage-based), so it works appropriately for all numeric types. This is the **recommended method** for generating random numbers in fuzz tests.

**Example:**

```rust
// High chance of edge cases, good magnitude coverage, some randomness
let amount: u64 = trident.random_biased();
let index: u32 = trident.random_biased();
```

**Interesting values include:**

- Boundary values: `0`, `1`, `MAX`, `MIN`
- Near-boundary: `MAX-1`, `MIN+1`
- Powers of 2: `2`, `4`, `8`, `16`, `32`, `64`, `128`, `256`...
- Type transitions: `127/128`, `255/256`, `32767/32768`, `65535/65536`...

---

### `random_pubkey`

Generates a random public key.

```rust
pub fn random_pubkey(&mut self) -> Pubkey
```

**Returns:** A randomly generated public key.

**Description:** Creates a random 32-byte public key, useful for testing with various account addresses.

---

### `random_string`

Generates a random string of the specified length.

```rust
pub fn random_string(&mut self, length: usize) -> String
```

**Parameters:**

- `length` - Length of the string to generate

**Returns:** Random string of the specified length.

**Description:** Generates a random string containing alphanumeric characters, useful for testing string inputs.

---

### `random_bytes`

Fills the provided byte array with random data.

```rust
pub fn random_bytes(&mut self, bytes: &mut [u8])
```

**Parameters:**

- `bytes` - Byte array to fill with random data

**Description:** Fills the provided byte slice with random data, useful for testing with arbitrary byte sequences.

---

### `random_bool`

Generates a random boolean value.

```rust
pub fn random_bool(&mut self) -> bool
```

**Returns:** A random boolean value.

**Description:** Creates a random boolean value with 50% probability for true or false, useful for testing conditional logic and feature flags.

---

### `random_keypair`

Generates a random Solana keypair.

```rust
pub fn random_keypair(&mut self) -> Keypair
```

**Returns:** A randomly generated Ed25519 keypair.

**Description:** Creates a cryptographically secure random Ed25519 keypair, useful for generating test signers, authority accounts, and testing signature verification.

---

## Example Usage

```rust
use trident_fuzz::*;

#[flow]
fn test_token_operations_with_random_data(&mut self) {
    // Generate random account addresses
    let user_account = self.random_pubkey();
    let mint_account = self.random_pubkey();
    
    // Generate random keypairs for signers and authorities
    let authority = self.random_keypair();
    let signer = self.random_keypair();
    
    // Generate random amounts - three distribution options:
    
    // 1. Uniform: equal probability in range (good for bounded values)
    let fee_basis_points = self.random_from_range(0..10000u16);
    
    // 2. Log-uniform: equal probability per magnitude (good for wide ranges)
    let transfer_amount: u64 = self.random_log_uniform();
    
    // 3. Biased: optimized for fuzzing (recommended for most cases)
    let deposit_amount: u64 = self.random_biased();
    
    // Generate random configuration values
    let decimals = self.random_from_range(0..9u8);
    let commission: u8 = self.random_biased(); // catches edge cases like 0, 127, 128, 255
    
    // Generate random string data for metadata
    let token_name = self.random_string(20);
    let token_symbol = self.random_string(5);
    let uri = format!("https://example.com/{}", self.random_string(10));
    
    // Generate random binary data
    let mut seed_data = [0u8; 32];
    self.random_bytes(&mut seed_data);
    
    // Generate random boolean values for feature flags
    let is_enabled = self.random_bool();
    let should_freeze = self.random_bool();
    
    // Use all the random data in your program testing
    // ... your program instructions with random inputs ...
}
```


