# Random Generation Methods

These methods provide functionality for generating random data in your fuzz tests, enabling comprehensive testing with varied inputs.

## Random Value Generation

### `random_from_range`

Generates a random number within the specified range.

```rust
pub fn random_from_range<T, R>(&mut self, range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>
```

**Parameters:**

- `range` - The range to generate the random number from

**Returns:** Random value within the specified range.

**Description:** Generates a random value of the specified type within the given range. Supports various numeric types.

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

## Example Usage

```rust
use trident_fuzz::*;

#[flow]
fn test_token_operations_with_random_data(&mut self) {
    // Generate random account addresses
    let user_account = self.random_pubkey();
    let mint_account = self.random_pubkey();
    
    // Generate random amounts for financial operations
    let transfer_amount = self.random_from_range(1..1_000_000u64);
    let fee_basis_points = self.random_from_range(0..10000u16);
    
    // Generate random configuration values
    let decimals = self.random_from_range(0..9u8);
    let commission = self.random_from_range(0..100u8);
    
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


