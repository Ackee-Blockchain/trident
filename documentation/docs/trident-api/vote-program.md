# Vote Program Methods

!!! info "Feature Flag Required"

    These methods are available when the `vote` feature is enabled:
    
    ```toml
    [dependencies]
    trident-fuzz = { features = ["vote"] }
    ```

The Vote Program methods provide functionality for working with Solana's vote program in your fuzz tests.

## Vote Account Operations

### `initialize_vote_account`

Creates and initializes a vote account with the specified configuration.

```rust
pub fn initialize_vote_account(
    &mut self,
    address: Pubkey,
    node_pubkey: &Pubkey,
    authorized_voter: &Pubkey,
    authorized_withdrawer: &Pubkey,
    commission: u8,
    clock: &Clock,
)
```

**Parameters:**

- `address` - The public key for the vote account
- `node_pubkey` - The validator's node public key
- `authorized_voter` - The authority allowed to submit votes
- `authorized_withdrawer` - The authority allowed to withdraw from the account
- `commission` - The commission percentage (0-100)
- `clock` - The current clock sysvar for initialization

**Returns:** None (void method).

**Description:** Creates a vote account for a validator with the specified configuration including voting authorities and commission rate.

---

## Example Usage

```rust
use trident_fuzz::*;
use solana_sdk::clock::Clock;

#[flow]
fn test_vote_account_creation(&mut self) {
    let vote_account = self.random_pubkey();
    let node_pubkey = self.random_pubkey();
    let authorized_voter = self.payer().pubkey();
    let authorized_withdrawer = self.payer().pubkey();
    let commission = self.random_from_range(0..=100u8);
    
    // Get current clock
    let clock = self.get_sysvar::<Clock>();
    
    // Initialize vote account
    self.initialize_vote_account(
        vote_account,
        &node_pubkey,
        &authorized_voter,
        &authorized_withdrawer,
        commission,
        &clock,
    );
    
    // Verify the account was created
    let account_data = self.get_account(&vote_account);
    assert!(account_data.lamports() > 0);
    assert_eq!(account_data.owner(), &solana_sdk::vote::program::ID);
}
```
