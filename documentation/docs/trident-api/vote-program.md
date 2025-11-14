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

Creates instructions to initialize a vote account with the specified configuration.

```rust
pub fn initialize_vote_account(
    &mut self,
    from_pubkey: &Pubkey,
    vote_pubkey: &Pubkey,
    node_pubkey: &Pubkey,
    authorized_voter: &Pubkey,
    authorized_withdrawer: &Pubkey,
    commission: u8,
    lamports: u64,
) -> Vec<Instruction>
```

**Parameters:**

- `from_pubkey` - The public key of the account to create the vote account from
- `vote_pubkey` - The public key of the vote account to create
- `node_pubkey` - The public key of the validator's node
- `authorized_voter` - The public key of the authority allowed to submit votes
- `authorized_withdrawer` - The public key of the authority allowed to withdraw from the account
- `commission` - The commission percentage (0-100)
- `lamports` - The number of lamports to transfer to the vote account

**Returns:** A vector of instructions that need to be executed with `process_transaction`.

**Description:** Generates instructions to create a vote account for a validator with the specified configuration including voting authorities and commission rate.

---

## Example Usage

```rust
use trident_fuzz::*;

#[flow]
fn test_vote_account_creation(&mut self) {
    let from_pubkey = self.payer().pubkey();
    let vote_account = self.random_pubkey();
    let node_pubkey = self.random_pubkey();
    let authorized_voter = self.payer().pubkey();
    let authorized_withdrawer = self.payer().pubkey();
    let commission = self.random_from_range(0..=100u8);
    let lamports = 1_000_000_000; // 1 SOL
    
    // Initialize vote account
    let instructions = self.initialize_vote_account(
        &from_pubkey,
        &vote_account,
        &node_pubkey,
        &authorized_voter,
        &authorized_withdrawer,
        commission,
        lamports,
    );
    let result = self.process_transaction(&instructions, Some("initialize_vote"));
    
    // Verify the transaction was successful
    assert!(result.is_success());
}
```
