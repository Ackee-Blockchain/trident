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

Gets account data and converts it to a specific data type for use in your tests.

```rust
pub fn get_account_with_type<T: BorshDeserialize>(
    &mut self,
    key: &Pubkey,
    discriminator_size: usize,
) -> Option<T>
```

**Parameters:**

- `key` - The public key of the account to retrieve
- `discriminator_size` - Size of the discriminator to skip when deserializing

**Returns:** Deserialized account data or None if deserialization fails.

!!! note "Account Type"

    Use account types contained in the `types.rs` file to get the correct type for the account.

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

Transfers SOL from one account to another by creating and executing a system program transfer instruction.

```rust
pub fn transfer(&mut self, from: &Pubkey, to: &Pubkey, amount: u64) -> TransactionResult
```

**Parameters:**

- `from` - The public key of the account to transfer from
- `to` - The public key of the account to transfer to
- `amount` - The number of lamports to transfer

**Returns:** A `TransactionResult` indicating success or failure of the transfer.

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

## Example Usage

```rust
use trident_fuzz::*;

#[flow]
fn test_account_management(&mut self) {
    let user_account = self.random_pubkey();
    let token_account = self.random_pubkey();
    let amount = self.random_from_range(1000..10000u64);
    
    // Airdrop lamports to an account
    self.airdrop(&user_account, amount);
    
    // Get account data and verify balance
    let account_data = self.get_account(&user_account);
    assert_eq!(account_data.lamports(), amount);
    
    // Execute a transfer and check the result
    let result = self.transfer(&user_account, &token_account, 500);
    assert!(result.is_success());
    
    // Get account with specific type (example with a custom struct)
    if let Some(my_data) = self.get_account_with_type::<MyAccountData>(&token_account, 8) {
        // Use the deserialized data
        println!("Account data: {:?}", my_data);
    }
    
    // Get current clock for time-based operations
    let clock = self.get_sysvar::<Clock>();
    println!("Current timestamp: {}", clock.unix_timestamp);
    
    // Get the default payer for transactions
    let payer = self.payer();
    println!("Payer pubkey: {}", payer.pubkey());
}

// Example custom account data structure
#[derive(BorshDeserialize, Debug)]
struct MyAccountData {
    value: u64,
    authority: Pubkey,
}
```
