# Address Storage

The `AddressStorage` struct provides a convenient way to manage and track unique public key addresses during fuzz testing. It can generate random addresses, derive Program Derived Addresses (PDAs), and allows you to randomly select from stored addresses for use in your fuzz tests.

Internally, `AddressStorage` uses a `Vec` + `HashMap` combination for O(1) insert, remove, contains, and random access operations.

!!! note "Order Not Preserved"
    Insertion order is not preserved after removals due to swap-remove optimization. This should not affect fuzz testing since addresses are typically selected randomly.

## Overview

Address storage is particularly useful when you need to:

- Track multiple unique accounts created during fuzzing
- Randomly select from previously created accounts
- Add or remove addresses dynamically during test execution
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

**Returns:** The newly created address (or existing one if it was already stored).

**Description:** Generates a new address and stores it if not already present. If PDA seeds are provided, attempts to derive a PDA. If derivation fails or no seeds are provided, generates a random keypair address using Trident's RNG. Duplicate addresses are silently ignored.

---

### `insert_with_address`

Inserts an existing address into storage without generating a new one.

```rust
pub fn insert_with_address(&mut self, address: Pubkey) -> bool
```

**Parameters:**

- `address` - The address to store

**Returns:**

- `true` - If the address was inserted
- `false` - If the address already existed in storage

**Description:** Stores a pre-existing address if not already present. This is useful when you need to track addresses created elsewhere, such as associated token accounts or addresses returned from program instructions.

---

### `remove`

Removes an address from storage.

```rust
pub fn remove(&mut self, address: &Pubkey) -> bool
```

**Parameters:**

- `address` - The address to remove

**Returns:**

- `true` - If the address was removed
- `false` - If the address wasn't found in storage

**Description:** Removes an address from storage using O(1) swap-remove. Note that this may change the order of elements in storage, but this does not affect random selection behavior.

---

### `contains`

Checks if an address exists in storage.

```rust
pub fn contains(&self, address: &Pubkey) -> bool
```

**Parameters:**

- `address` - The address to check

**Returns:** `true` if the address is in storage, `false` otherwise.

**Description:** Performs an O(1) lookup to check if an address is currently stored.

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
use trident_fuzz::fuzzing::*;

#[flow]
fn test_address_storage(&mut self) {
    // Generate and store random addresses
    for _ in 0..5 {
        let address = self.fuzz_accounts.user_accounts.insert(&mut self.trident, None);
        println!("Created address: {}", address);
    }
    
    // Check storage state
    assert_eq!(self.fuzz_accounts.user_accounts.len(), 5);
    assert!(!self.fuzz_accounts.user_accounts.is_empty());
    
    // Randomly select an address for testing
    if let Some(random_user) = self.fuzz_accounts.user_accounts.get(&mut self.trident) {
        // Use the address in a transaction
        self.trident.airdrop(&random_user, 1_000_000);
    }
}
```

### Removing and Checking Addresses

```rust
use trident_fuzz::fuzzing::*;

#[flow]
fn test_remove_addresses(&mut self) {
    // Create some addresses
    let addr1 = self.fuzz_accounts.user_accounts.insert(&mut self.trident, None);
    let addr2 = self.fuzz_accounts.user_accounts.insert(&mut self.trident, None);
    let addr3 = self.fuzz_accounts.user_accounts.insert(&mut self.trident, None);
    
    assert_eq!(self.fuzz_accounts.user_accounts.len(), 3);
    
    // Check if an address exists
    assert!(self.fuzz_accounts.user_accounts.contains(&addr1));
    
    // Remove an address
    let was_removed = self.fuzz_accounts.user_accounts.remove(&addr2);
    assert!(was_removed);
    assert_eq!(self.fuzz_accounts.user_accounts.len(), 2);
    
    // Address no longer exists
    assert!(!self.fuzz_accounts.user_accounts.contains(&addr2));
    
    // Trying to remove again returns false
    assert!(!self.fuzz_accounts.user_accounts.remove(&addr2));
}
```

### Working with PDAs

```rust
use trident_fuzz::fuzzing::*;

#[flow]
fn test_pda_storage(&mut self) {
    let program_id = your_program::program_id();
    
    // Create a PDA with seeds using PdaSeeds::new
    let seeds = PdaSeeds::new(&[b"vault_seed"], program_id);
    let pda = self.fuzz_accounts.pda_accounts.insert(&mut self.trident, Some(seeds));
    
    // Airdrop to payer for transaction fees
    let payer = self.trident.payer().pubkey();
    self.trident.airdrop(&payer, 10 * LAMPORTS_PER_SOL);
    
    // Use the PDA in your program instructions
    let ix = YourInstruction::data(YourInstructionData::new())
        .accounts(YourInstructionAccounts::new(payer, pda))
        .instruction();
    
    let result = self.trident.process_transaction(&[ix], Some("initialize_pda"));
    assert!(result.is_success());
}
```