# SPL Token Methods

!!! info "Feature Flag Required"

    These methods are available when the `token` feature is enabled:
    
    ```toml
    [dependencies]
    trident-fuzz = { features = ["token"] }
    ```

The SPL Token methods provide convenient functions for working with SPL Token program operations in your fuzz tests.

## Mint Operations

### `initialize_mint`

Creates and initializes a new SPL Token mint.

```rust
pub fn initialize_mint(
    &mut self,
    payer: &Pubkey,
    mint_address: &Pubkey,
    decimals: u8,
    owner: &Pubkey,
    freeze_authority: Option<&Pubkey>,
) -> TransactionResult
```

**Parameters:**

- `payer` - The payer covering the rent
- `mint_address` - The public key for the new mint account
- `decimals` - Number of decimal places for the token
- `owner` - The mint authority that can mint new tokens
- `freeze_authority` - Optional authority that can freeze token accounts

**Returns:** `TransactionResult` indicating success or failure.

**Description:** Creates a new SPL Token mint that can be used to mint tokens with the specified decimal precision and authorities.

---

### `mint_to`

Mints tokens to a specified token account.

```rust
pub fn mint_to(
    &mut self,
    token_account_address: &Pubkey,
    mint_address: &Pubkey,
    mint_authority: &Pubkey,
    amount: u64,
) -> TransactionResult
```

**Parameters:**

- `token_account_address` - The token account to mint tokens to
- `mint_address` - The mint to mint tokens from
- `mint_authority` - The authority allowed to mint tokens
- `amount` - The number of tokens to mint (in base units)

**Returns:** `TransactionResult` indicating success or failure.

**Description:** Creates new tokens and adds them to the specified token account.

---

## Token Account Operations

### `initialize_token_account`

Creates and initializes a new SPL Token account.

```rust
pub fn initialize_token_account(
    &mut self,
    payer: &Pubkey,
    token_account_address: &Pubkey,
    mint: &Pubkey,
    owner: &Pubkey,
) -> TransactionResult
```

**Parameters:**

- `payer` - The payer covering the rent
- `token_account_address` - The public key for the new token account
- `mint` - The mint this account will hold tokens for
- `owner` - The owner of the token account

**Returns:** `TransactionResult` indicating success or failure.

**Description:** Creates a new token account that can hold tokens from the specified mint for the given owner.

---

### `initialize_associated_token_account`

Creates an associated token account for a given mint and owner.

```rust
pub fn initialize_associated_token_account(
    &mut self,
    payer: &Pubkey,
    mint: &Pubkey,
    owner: &Pubkey,
) -> TransactionResult
```

**Parameters:**

- `payer` - The payer covering the rent
- `mint` - The mint for the associated token account
- `owner` - The owner of the associated token account

**Returns:** `TransactionResult` indicating success or failure.

**Description:** Creates an associated token account - a deterministic token account address derived from the owner and mint.

---

## Utility Methods

### `get_associated_token_address`

Calculates the associated token account address for a given mint and owner.

```rust
pub fn get_associated_token_address(
    &self,
    mint: &Pubkey,
    owner: &Pubkey,
    program_id: &Pubkey,
) -> Pubkey
```

**Parameters:**

- `mint` - The mint public key
- `owner` - The owner public key
- `program_id` - The token program ID (usually SPL Token program ID)

**Returns:** The calculated associated token account address.

**Description:** Calculates what the associated token account address would be for the given mint and owner without creating the account.

---

## Example Usage

```rust
use trident_fuzz::*;

#[flow]
fn test_token_operations(&mut self) {
    let mint_keypair = self.random_pubkey();
    let owner = self.payer().pubkey();
    
    // Initialize a mint
    let result = self.initialize_mint(
        &mint_keypair,
        6, // 6 decimals
        &owner,
        Some(&owner), // freeze authority
    );
    assert!(result.is_success());
    
    // Create associated token account
    let result = self.initialize_associated_token_account(
        &mint_keypair,
        &owner,
    );
    assert!(result.is_success());
    
    // Get the associated token account address
    let token_account = self.get_associated_token_address(
        &mint_keypair,
        &owner,
        &spl_token_interface::ID,
    );
    
    // Mint some tokens
    let result = self.mint_to(
        &token_account,
        &mint_keypair,
        &owner,
        1_000_000, // 1 token with 6 decimals
    );
    assert!(result.is_success());
}
```
