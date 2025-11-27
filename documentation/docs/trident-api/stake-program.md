# Stake Program Methods

!!! info "Feature Flag Required"

    These methods are available when the `stake` feature is enabled:
    
    ```toml
    [dependencies]
    trident-fuzz = { features = ["stake"] }
    ```

The Stake Program methods provide functionality for working with Solana's stake program in your fuzz tests.

## Stake Account Operations

### `create_initialized_account`

Creates instructions to initialize a stake account without delegation.

```rust
pub fn create_initialized_account(
    &mut self,
    from_pubkey: &Pubkey,
    stake_pubkey: &Pubkey,
    authorized: &Authorized,
    lockup: Lockup,
    lamports: u64,
) -> Vec<Instruction>
```

**Parameters:**

- `from_pubkey` - The public key of the account funding the stake account creation
- `stake_pubkey` - The public key of the stake account to create
- `authorized` - The authorized staker and withdrawer authorities
- `lockup` - The lockup configuration for the stake account
- `lamports` - The number of lamports to transfer to the stake account

**Returns:** A vector of instructions that need to be executed with `process_transaction`.

**Description:** Generates instructions to create a stake account that can be used to delegate SOL to validators for earning staking rewards.

### `create_and_delegate_account`

Creates instructions to create and delegate a stake account.

```rust
pub fn create_and_delegate_account(
    &mut self,
    from_pubkey: &Pubkey,
    stake_pubkey: &Pubkey,
    vote_pubkey: &Pubkey,
    authorized: &Authorized,
    lockup: Lockup,
    lamports: u64,
) -> Vec<Instruction>
```

**Parameters:**

- `from_pubkey` - The public key of the account funding the stake account creation
- `stake_pubkey` - The public key of the stake account to create
- `vote_pubkey` - The public key of the vote account to delegate to
- `authorized` - The authorized staker and withdrawer authorities
- `lockup` - The lockup configuration for the stake account
- `lamports` - The number of lamports to transfer to the stake account

**Returns:** A vector of instructions that need to be executed with `process_transaction`.

**Description:** Generates instructions to create a new stake account and immediately delegate it to the specified vote account, combining both operations.

---

## Configuration Types

### Authorized Structure

The `Authorized` struct contains:

- **staker** - The public key authorized to manage staking operations
- **withdrawer** - The public key authorized to withdraw from the account

## Lockup Configuration

The `Lockup` struct contains:

- **unix_timestamp** - Unix timestamp when the lockup expires
- **epoch** - Epoch when the lockup expires  
- **custodian** - Optional custodian that can manage the account during lockup

## Stake Account States

Stake accounts progress through several states:

1. **Uninitialized** - Account exists but not initialized
2. **Initialized** - Account is initialized but not delegated
3. **Stake** - Account is delegated to a validator
4. **RewardsPool** - Account is part of the rewards pool (deprecated)

## Example Usage

```rust
use trident_fuzz::*;
use solana_stake_interface::state::{Authorized, Lockup};

#[flow]
fn test_stake_account_creation(&mut self) {
    let from_pubkey = self.payer().pubkey();
    let stake_account = self.random_pubkey();
    let vote_account = self.random_pubkey();
    let lamports = 1_000_000_000; // 1 SOL
    
    // Create authorized authorities
    let authorized = Authorized {
        staker: from_pubkey,
        withdrawer: from_pubkey,
    };
    
    // Create lockup configuration
    let lockup = Lockup {
        unix_timestamp: 0, // No timestamp lockup
        epoch: 0,          // No epoch lockup
        custodian: from_pubkey, // Payer is also custodian
    };
    
    // Create initialized stake account
    let instructions = self.create_initialized_account(
        &from_pubkey,
        &stake_account,
        &authorized,
        lockup,
        lamports,
    );
    let result = self.process_transaction(&instructions, Some("create_stake"));
    assert!(result.is_success());
    
    // Or create and delegate in one transaction
    let delegated_stake = self.random_pubkey();
    let instructions = self.create_and_delegate_account(
        &from_pubkey,
        &delegated_stake,
        &vote_account,
        &authorized,
        lockup,
        lamports,
    );
    let result = self.process_transaction(&instructions, Some("create_and_delegate"));
    assert!(result.is_success());
}
```


