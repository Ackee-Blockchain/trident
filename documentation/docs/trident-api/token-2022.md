# Token 2022 Methods

!!! info "Feature Flag Required"

    These methods are available when the `token` feature is enabled:
    
    ```toml
    [dependencies]
    trident-fuzz = { features = ["token"] }
    ```

The Token 2022 methods provide comprehensive support for the SPL Token 2022 program, including all extensions and proper initialization order handling.

## Mint Operations

### `initialize_mint_2022`

Creates instructions to initialize a Token 2022 mint with specified extensions.

```rust
pub fn initialize_mint_2022(
    &mut self,
    payer: &Pubkey,
    mint_address: &Pubkey,
    decimals: u8,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    extensions: &[MintExtension],
) -> Vec<Instruction>
```

**Parameters:**

- `payer` - The payer covering the rent
- `mint_address` - The public key for the new mint
- `decimals` - Number of decimal places for the token
- `mint_authority` - Authority that can mint new tokens
- `freeze_authority` - Optional authority that can freeze accounts
- `extensions` - Array of extensions to enable on the mint

**Returns:** A vector of instructions that need to be executed with `process_transaction`.

**Description:** Generates instructions to create a Token 2022 mint with the specified extensions enabled. You can combine multiple extensions to create mints with advanced functionality like transfer fees, interest-bearing tokens, or metadata.

---

### `mint_to_2022`

Creates an instruction to mint tokens to a Token 2022 account.

```rust
pub fn mint_to_2022(
    &mut self,
    token_account_address: &Pubkey,
    mint_address: &Pubkey,
    mint_authority: &Pubkey,
    amount: u64,
) -> Instruction
```

**Parameters:**

- `token_account_address` - The account to mint tokens to
- `mint_address` - The mint to mint tokens from
- `mint_authority` - The authority allowed to mint tokens
- `amount` - The number of tokens to mint (in base units)

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to mint the specified amount of tokens to the target Token 2022 account.

---

## Token Account Operations

### `initialize_token_account_2022`

Creates instructions to initialize a Token 2022 token account with specified extensions.

```rust
pub fn initialize_token_account_2022(
    &mut self,
    payer: &Pubkey,
    token_account_address: &Pubkey,
    mint: &Pubkey,
    owner: &Pubkey,
    extensions: &[AccountExtension],
) -> Vec<Instruction>
```

**Parameters:**

- `payer` - The payer covering the rent
- `token_account_address` - The public key for the new token account
- `mint` - The mint this account will hold tokens for
- `owner` - The owner of the token account
- `extensions` - Array of extensions to enable on the account

**Returns:** A vector of instructions that need to be executed with `process_transaction`.

**Description:** Generates instructions to create a Token 2022 account with the specified extensions enabled. Extensions like immutable owner, memo transfers, or CPI guard can be added to enhance account security and functionality.

---

### `initialize_associated_token_account_2022`

Creates instructions to initialize an associated Token 2022 account with specified extensions.

```rust
pub fn initialize_associated_token_account_2022(
    &mut self,
    payer: &Pubkey,
    mint: &Pubkey,
    owner: &Pubkey,
    extensions: &[AccountExtension],
) -> Vec<Instruction>
```

**Parameters:**

- `payer` - The payer covering the rent
- `mint` - The mint this account will hold tokens for
- `owner` - The owner of the token account
- `extensions` - Array of additional extensions to enable on the account

**Returns:** A vector of instructions that need to be executed with `process_transaction`.

**Description:** Generates instructions to create an associated Token 2022 account with additional extensions. The account is automatically funded and any mint-required extensions are included, plus any additional extensions you specify.

---

## Transfer Operations

### `transfer_checked`

Creates an instruction to transfer tokens between Token 2022 accounts with amount and decimals verification.

```rust
pub fn transfer_checked(
    &mut self,
    source: &Pubkey,
    destination: &Pubkey,
    mint: &Pubkey,
    authority: &Pubkey,
    signers: &[&Pubkey],
    amount: u64,
    decimals: u8,
) -> Instruction
```

**Parameters:**

- `source` - The source token account
- `destination` - The destination token account  
- `mint` - The mint of the tokens being transferred
- `authority` - The authority allowed to transfer from the source account
- `signers` - Additional signers if using multisig
- `amount` - The number of tokens to transfer (in base units)
- `decimals` - The number of decimals for the mint (for verification)

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to transfer tokens between accounts with built-in verification of amount and decimals to prevent transfer errors.

---

## Account Inspection Methods

!!! note "Use Unified Methods"

    For inspecting token accounts and mints, use the unified methods from the [SPL Token API](./spl-token.md):
    
    - `get_token_account` - Works with both SPL Token and Token 2022 accounts
    - `get_mint` - Works with both SPL Token and Token 2022 mints

    These methods automatically detect the token program and deserialize all extensions for Token 2022 accounts.

---

## Extension Types

### Mint Extensions

Available mint extensions include:

- **TransferFeeConfig** - Configures transfer fees for the mint
- **MintCloseAuthority** - Allows closing the mint account
- **InterestBearingConfig** - Enables interest-bearing tokens
- **NonTransferable** - Makes tokens non-transferable
- **PermanentDelegate** - Sets a permanent delegate for all accounts
- **TransferHook** - Configures transfer hook program
- **MetadataPointer** - Points to token metadata
- **GroupPointer** - Points to token group
- **GroupMemberPointer** - Points to group member data
- **ScaledUiAmount** - Configures UI amount scaling
- **Pausable** - Allows pausing token operations
- **DefaultAccountState** - Sets default state for new accounts
- **TokenMetadata** - Stores token metadata on-chain
- **TokenGroup** - Creates token groups
- **TokenGroupMember** - Adds tokens to groups

### Account Extensions

Available account extensions include:

- **ImmutableOwner** - Makes the account owner immutable
- **MemoTransfer** - Requires memos for transfers
- **CpiGuard** - Prevents CPI calls to the account

## Example Usage

```rust
use trident_fuzz::*;

#[flow]
fn test_token_2022_operations(&mut self) {
    let mint_keypair = self.random_pubkey();
    let owner = self.payer().pubkey();
    
    // Create mint with transfer fee extension
    let extensions = vec![
        MintExtension::TransferFeeConfig {
            transfer_fee_config_authority: Some(owner),
            withdraw_withheld_authority: Some(owner),
            transfer_fee_basis_points: 100, // 1% fee
            maximum_fee: 1_000_000,
        }
    ];
    
    let instructions = self.initialize_mint_2022(
        &owner,
        &mint_keypair,
        6, // 6 decimals
        &owner,
        Some(&owner), // freeze authority
        &extensions,
    );
    let result = self.process_transaction(&instructions, Some("initialize_mint_2022"));
    assert!(result.is_success());
    
    // Create token account with memo transfer extension
    let account_extensions = vec![
        AccountExtension::MemoTransfer {
            require_incoming_transfer_memos: true,
        }
    ];
    
    let instructions = self.initialize_associated_token_account_2022(
        &owner,
        &mint_keypair,
        &owner,
        &account_extensions,
    );
    let result = self.process_transaction(&instructions, Some("create_ata_2022"));
    assert!(result.is_success());
    
    // Get mint data with extensions
    let mint_data = self.get_mint(mint_keypair).unwrap();
    println!("Mint has {} extensions", mint_data.extensions.len());
}
```
