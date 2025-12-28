# Account Management Methods

These methods provide functionality for managing accounts in the fuzzing environment, including retrieving, setting, and manipulating account data.

## Account Data Methods

### `get_account`

Retrieves account data for the specified public key.

```rust
pub fn get_account(&mut self, key: &Pubkey) -> AccountSharedData
```

**Parameters:**

- `key` - The public key of the account to retrieve

**Returns:** Account data or default empty account if not found.

---

### `get_account_with_type`

Gets account data and converts it to a specific data type for use in your tests. The discriminator size is automatically determined from the type's `AccountDiscriminator` implementation, unless overridden.

```rust
pub fn get_account_with_type<T: BorshDeserialize + AccountDiscriminator>(
    &mut self,
    key: &Pubkey,
    discriminator_size_override: Option<usize>,
) -> Option<T>
```

**Parameters:**

- `key` - The public key of the account to retrieve
- `discriminator_size_override` - Optional override for discriminator size to skip. If `None`, uses the length from the type's `AccountDiscriminator` implementation.

**Returns:** Deserialized account data or None if deserialization fails.

!!! note "Account Type"

    Use account types contained in the `types.rs` file. These types implement `AccountDiscriminator` trait which provides the discriminator bytes from the IDL.

!!! note "Discriminator Handling"

    This method does not verify the discriminator. It only uses the discriminator length to skip those bytes during deserialization.

---

## Lamport Management

### `airdrop`

Adds lamports to the specified account address.

```rust
pub fn airdrop(&mut self, address: &Pubkey, amount: u64)
```

**Parameters:**

- `address` - The account to receive the lamports
- `amount` - The number of lamports to add

---

### `transfer`

Creates an instruction to transfer SOL from one account to another.

```rust
pub fn transfer(&mut self, from: &Pubkey, to: &Pubkey, amount: u64) -> Instruction
```

**Parameters:**

- `from` - The public key of the account to transfer from
- `to` - The public key of the account to transfer to
- `amount` - The number of lamports to transfer

**Returns:** An instruction that needs to be executed with `process_transaction`.

---

## Utility Methods

### `payer`

Returns the default payer keypair for transactions.

```rust
pub fn payer(&self) -> solana_sdk::signature::Keypair
```

**Returns:** The payer keypair used for transaction fees.

---

### `get_sysvar`

Retrieves a sysvar of the specified type.

```rust
pub fn get_sysvar<T: Sysvar>(&self) -> T
```

**Returns:** The requested sysvar data.

---

### `get_program_data_address_v3`

Derives the program data address for an upgradeable program.

```rust
pub fn get_program_data_address_v3(&self, program_address: &Pubkey) -> Pubkey
```

**Parameters:**

- `program_address` - The public key of the upgradeable program

**Returns:** The derived program data address (PDA).

**Description:** Finds the program data account address for an upgradeable BPF loader program. This is useful when you need to access or verify the program data account associated with an upgradeable program.

---

### `create_program_address`

Creates a program address (PDA) from seeds and a program ID.

```rust
pub fn create_program_address(&self, seeds: &[&[u8]], program_id: &Pubkey) -> Option<Pubkey>
```

**Parameters:**

- `seeds` - Array of seed byte slices used to derive the address
- `program_id` - The program ID to use for derivation

**Returns:** Some(Pubkey) if the seeds produce a valid PDA, None otherwise.

**Description:** Attempts to create a valid program-derived address using the provided seeds and program ID. Unlike `find_program_address`, this does not search for a valid bump seed and will return None if the provided seeds don't produce a valid PDA.

---

### `find_program_address`

Finds a valid program address (PDA) and its bump seed.

```rust
pub fn find_program_address(&self, seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8)
```

**Parameters:**

- `seeds` - Array of seed byte slices used to derive the address
- `program_id` - The program ID to use for derivation

**Returns:** A tuple containing the derived PDA and the bump seed used to generate it.

**Description:** Searches for a valid program-derived address by trying different bump seeds (starting from 255 and counting down) until a valid PDA is found. This is the canonical way to derive PDAs in Solana programs.

---

## Direct Account Manipulation

!!! warning "Testing Environment Only"

    The following methods allow direct manipulation of account state without going through program instructions. This behavior does not reflect normal mainnet conditions where accounts can only be modified by their owning program through proper transaction execution. Use these methods for test setup and specific fuzzing scenarios only.

### `set_account_with_type`

Sets account data for a specific type at the specified address. The data is serialized with its discriminator prepended and stored as a rent-exempt account.

```rust
pub fn set_account_with_type<T: BorshSerialize + AccountDiscriminator>(
    &mut self,
    address: &Pubkey,
    owner: &Pubkey,
    data: &T,
    discriminator_override: Option<&[u8]>,
)
```

**Parameters:**

- `address` - The public key where the account should be stored
- `owner` - The program ID that will own this account
- `data` - The account data to serialize and store
- `discriminator_override` - Optional custom discriminator bytes to use. If `None`, uses the discriminator from the type's `AccountDiscriminator` implementation.

!!! warning "Discriminator Compatibility"

    For older Anchor versions where the IDL does not contain discriminator data, an 8-byte zero discriminator (`[0, 0, 0, 0, 0, 0, 0, 0]`) is used by default. This is typically not a problem for `get_account_with_type` as it only skips the discriminator bytes without validation. However, `set_account_with_type` will serialize this zero discriminator, which will cause issues when the program attempts to deserialize the account since it won't match the expected discriminator. Use `discriminator_override` to provide the correct discriminator if your IDL lacks this data.

---

### `set_account_custom`

Sets custom account data for the specified address.

```rust
pub fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData)
```

**Parameters:**

- `address` - The account address to set
- `account` - The account data to set

---

## Example Usage

```rust
use trident_fuzz::fuzzing::*;

#[flow]
fn test_account_management(&mut self) {
    let user_account = self.trident.random_pubkey();
    let token_account = self.trident.random_pubkey();
    let program_id = Pubkey::new_unique();
    
    // Airdrop lamports to an account
    self.trident.airdrop(&user_account, 10 * LAMPORTS_PER_SOL);
    
    // Get account data and verify balance
    let account_data = self.trident.get_account(&user_account);
    
    // Get account with specific type (discriminator handled automatically)
    if let Some(my_data) = self.trident.get_account_with_type::<MyAccountData>(&token_account, None) {
        println!("Account data: {:?}", my_data);
    }
    
    // Set account with specific type (discriminator and rent handled automatically)
    let new_data = MyAccountData::new(1000, user_account);
    self.trident.set_account_with_type(&token_account, &program_id, &new_data, None);
    
    // Get current clock
    let clock = self.trident.get_sysvar::<Clock>();
    println!("Current timestamp: {}", clock.unix_timestamp);
    
    // Get the default payer
    let payer = self.trident.payer();
    println!("Payer pubkey: {}", payer.pubkey());
    
    // Find a PDA
    let seeds = &[b"my-seed", user_account.as_ref()];
    let (pda, bump) = self.trident.find_program_address(seeds, &program_id);
    
    // Create a PDA with known bump
    let seeds_with_bump = &[b"my-seed", user_account.as_ref(), &[bump]];
    if let Some(pda_verified) = self.trident.create_program_address(seeds_with_bump, &program_id) {
        assert_eq!(pda, pda_verified);
    }
}
```
