# Address Storage

The `AddressStorage` struct provides a convenient way to manage and track public key addresses during fuzz testing. It can generate random addresses, derive Program Derived Addresses (PDAs), and allows you to randomly select from stored addresses for use in your fuzz tests.

## Overview

Address storage is particularly useful when you need to:

- Track multiple accounts created during fuzzing
- Randomly select from previously created accounts
- Manage both regular keypair addresses and PDAs
- Ensure reproducible address generation using Trident's RNG

## Core Methods

### `insert`

Inserts a new address into storage, either by deriving a PDA or generating a random keypair.

```rust
pub fn insert(&mut self, trident: &mut Trident, seeds: Option<PdaSeeds>) -> Pubkey
```

**Parameters:**

- `trident` - The Trident instance for random number generation
- `seeds` - Optional PDA seeds for deriving a program-derived address

**Returns:** The newly created and stored address.

**Description:** Generates a new address and stores it. If PDA seeds are provided, attempts to derive a PDA. If derivation fails or no seeds are provided, generates a random keypair address using Trident's RNG.

---

### `insert_with_address`

Inserts an existing address into storage without generating a new one.

```rust
pub fn insert_with_address(&mut self, address: Pubkey)
```

**Parameters:**

- `address` - The address to store

**Description:** Stores a pre-existing address. This is useful when you need to track addresses created elsewhere, such as associated token accounts or addresses returned from program instructions.

---

### `get`

Retrieves a random address from storage.

```rust
pub fn get(&self, trident: &mut Trident) -> Option<Pubkey>
```

**Parameters:**

- `trident` - The Trident instance for random number generation

**Returns:**

- `Some(Pubkey)` - A randomly selected address from storage
- `None` - If the storage is empty

**Description:** Randomly selects one of the stored addresses using Trident's RNG. This is useful for fuzzing operations that need to work with previously created accounts.

---

### `get_except`

Retrieves a random address from storage, excluding specified addresses.

```rust
pub fn get_except(&self, trident: &mut Trident, except_addresses: &[Pubkey]) -> Option<Pubkey>
```

**Parameters:**

- `trident` - The Trident instance for random number generation
- `except_addresses` - Slice of addresses to exclude from selection

**Returns:**

- `Some(Pubkey)` - A randomly selected address that is not in the exclusion list
- `None` - If storage is empty or all addresses are in the exclusion list

**Description:** Randomly selects one of the stored addresses using Trident's RNG, ensuring the selected address is not in the exclusion list. This is particularly useful for fuzzing operations that require distinct accounts, such as transfers where the sender and receiver must be different addresses.

---

### `is_empty`

Checks if the storage contains any addresses.

```rust
pub fn is_empty(&self) -> bool
```

**Returns:** `true` if no addresses are stored, `false` otherwise.

---

### `len`

Returns the number of stored addresses.

```rust
pub fn len(&self) -> usize
```

**Returns:** The count of addresses currently in storage.

---

## PDA Support

### `PdaSeeds`

A helper struct for specifying seeds when deriving Program Derived Addresses.

```rust
pub struct PdaSeeds<'a> {
    pub seeds: &'a [&'a [u8]],
    pub program_id: Pubkey,
}
```

**Fields:**

- `seeds` - The seeds to use for PDA derivation
- `program_id` - The program ID to use for PDA derivation

### `PdaSeeds::new`

Creates a new `PdaSeeds` instance.

```rust
pub fn new(seeds: &'a [&'a [u8]], program_id: Pubkey) -> Self
```

**Parameters:**

- `seeds` - The seeds to use for PDA derivation
- `program_id` - The program ID to use for PDA derivation

**Returns:** A new `PdaSeeds` instance.

---

## Example Usage

### Basic Address Management

```rust
use trident_fuzz::*;
use trident_fuzz::address_storage::AddressStorage;

#[flow]
fn test_address_storage(&mut self) {
    // Generate and store random addresses
    for _ in 0..5 {
        let address = self.fuzz_accounts.user_accounts.insert(self, None);
        println!("Created address: {}", address);
    }
    
    // Check storage state
    assert_eq!(self.fuzz_accounts.user_accounts.len(), 5);
    assert!(!self.fuzz_accounts.user_accounts.is_empty());
    
    // Randomly select an address for testing
    let random_user = self.fuzz_accounts.user_accounts.get(self);
    
    // Use the address in a transaction
    self.airdrop(&random_user, 1_000_000);
    let account = self.get_account(&random_user);
    assert_eq!(account.lamports(), 1_000_000);
}
```

### Working with PDAs

```rust
use trident_fuzz::*;
use trident_fuzz::address_storage::{AddressStorage, PdaSeeds};

#[flow]
fn test_pda_storage(&mut self) {
    let program_id = Pubkey::new_unique();
    
    // Create multiple PDAs with different seeds
    for i in 0..3 {
        let seed = format!("vault-{}", i);
        let seeds = PdaSeeds::new(
            &[seed.as_bytes()],
            program_id
        );
        
        let pda = self.fuzz_accounts.pda_accounts.insert(self, Some(seeds));
        println!("Created PDA: {}", pda);
    }
    
    // Randomly select a PDA for testing
    let random_pda = self.fuzz_accounts.pda_accounts.get(self);
    
    // Use the PDA in your program instructions
    let ix = your_program::instruction::initialize(
        program_id,
        random_pda,
        self.payer().pubkey(),
    );
    
    let result = self.process_transaction(&[ix], Some("initialize_pda"));
    assert!(result.is_success());
}
```