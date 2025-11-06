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

Creates and initializes a stake account with the specified configuration.

```rust
pub fn create_initialized_account(
    &mut self,
    address: Pubkey,
    staker: Pubkey,
    lockup: Lockup,
) -> solana_sdk::transaction::Result<()>
```

**Parameters:**

- `address` - The public key for the stake account
- `staker` - The authority that can manage the stake account
- `lockup` - The lockup configuration for the stake account

**Returns:** `Result<()>` indicating success or failure.

**Description:** Creates a stake account that can be used to delegate SOL to validators for earning staking rewards.

---

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
use solana_stake_interface::state::Lockup;

#[flow]
fn test_stake_account_creation(&mut self) {
    let stake_account = self.random_pubkey();
    let staker = self.payer().pubkey();
    
    // Create lockup configuration
    let lockup = Lockup {
        unix_timestamp: 0, // No timestamp lockup
        epoch: 0,          // No epoch lockup
        custodian: staker, // Staker is also custodian
    };
    
    // Create initialized stake account
    let result = self.create_initialized_account(
        stake_account,
        staker,
        lockup,
    );
    assert!(result.is_ok());
    
    // Verify the account was created
    let account_data = self.get_account(&stake_account);
    assert!(account_data.lamports() > 0);
    assert_eq!(account_data.owner(), &solana_stake_interface::program::ID);
}
```

## Commented Out Methods

### `create_delegated_account` (TODO)

```rust
// pub fn create_delegated_account(
//     &mut self,
//     address: Pubkey,
//     voter_pubkey: Pubkey,
//     staker: Pubkey,
//     withdrawer: Pubkey,
//     stake: u64,
//     activation_epoch: Epoch,
//     deactivation_epoch: Option<Epoch>,
//     lockup: Option<Lockup>,
// )
```

**Status:** TODO - This method is commented out in the source code and not yet implemented.

**Description:** Would create a stake account that is already delegated to a validator with the specified delegation parameters.

---

