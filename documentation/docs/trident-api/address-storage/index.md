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
pub fn get(&self, trident: &mut Trident) -> Pubkey
```

**Parameters:**

- `trident` - The Trident instance for random number generation

**Returns:** A randomly selected address from storage.

**Description:** Randomly selects one of the stored addresses using Trident's RNG. This is useful for fuzzing operations that need to work with previously created accounts.

!!! warning "Empty Storage"

    This method will panic if the storage is empty. Always check with `is_empty()` before calling `get()` if you're unsure whether addresses have been stored.

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

```rust
use trident_fuzz::*;
use trident_fuzz::address_storage::{AddressStorage, PdaSeeds};

#[flow]
fn test_combined_address_management(&mut self) {
    let program_id = Pubkey::new_unique();
    
    // Mix of random addresses and PDAs
    for i in 0..10 {
        if i % 2 == 0 {
            // Create random address
            self.fuzz_accounts.all_accounts.insert(self, None);
        } else {
            // Create PDA
            let seeds = PdaSeeds::new(
                &[b"account", &[i as u8]],
                program_id
            );
            self.fuzz_accounts.all_accounts.insert(self, Some(seeds));
        }
    }
    
    // Perform random operations on random accounts
    for _ in 0..20 {
        let account1 = self.fuzz_accounts.all_accounts.get(self);
        let account2 = self.fuzz_accounts.all_accounts.get(self);
        let amount = self.random_from_range(100..1000u64);
        
        // Ensure accounts have lamports
        self.airdrop(&account1, 10_000);
        
        // Random transfer between accounts
        let ix = self.transfer(&account1, &account2, amount);
        self.process_transaction(&[ix], Some("random_transfer"));
    }
}
```



