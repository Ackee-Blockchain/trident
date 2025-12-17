# Stake Program Methods

!!! info "Feature Flag Required"

    These methods are available when the `stake` feature is enabled:
    
    ```toml
    [dependencies]
    trident-fuzz = { features = ["stake"] }
    ```

The Stake Program methods provide functionality for working with Solana's stake program in your fuzz tests.

---

## Stake Account Creation

### `create_initialized_account`

Creates instructions to initialize a stake account without delegation.

```rust
pub fn create_initialized_account(
    &mut self,
    payer: &Pubkey,
    stake_pubkey: &Pubkey,
    withdrawer_pubkey: &Pubkey,
    staker_pubkey: &Pubkey,
    unix_timestamp: i64,
    epoch: u64,
    custodian: &Pubkey,
) -> Vec<Instruction>
```

**Parameters:**

- `payer` - The public key of the account funding the stake account creation
- `stake_pubkey` - The public key of the stake account to create
- `withdrawer_pubkey` - The public key of the withdrawer authority
- `staker_pubkey` - The public key of the staker authority
- `unix_timestamp` - The Unix timestamp before which the stake cannot be withdrawn
- `epoch` - The epoch before which the stake cannot be withdrawn
- `custodian` - The public key of the custodian who can modify the lockup

**Returns:** A vector of instructions that need to be executed with `process_transaction`.

**Description:** Generates instructions to create a new stake account with the specified authorities and lockup configuration, but does not delegate it to any vote account. The stake account will be funded with the minimum rent-exempt balance.

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

## Stake Account Management

### `split_stake`

Creates instructions to split a stake account.

```rust
pub fn split_stake(
    &mut self,
    stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    lamports: u64,
    split_stake_pubkey: &Pubkey,
) -> Vec<Instruction>
```

**Parameters:**

- `stake_pubkey` - The public key of the source stake account to split from
- `authorized_pubkey` - The public key of the stake authority
- `lamports` - The number of lamports to split into the new stake account
- `split_stake_pubkey` - The public key of the new stake account to receive the split stake

**Returns:** A vector of instructions that need to be executed with `process_transaction`.

**Description:** Generates instructions to split a portion of the stake from an existing stake account into a new stake account.

### `merge_stake`

Creates instructions to merge two stake accounts.

```rust
pub fn merge_stake(
    &mut self,
    destination_stake_pubkey: &Pubkey,
    source_stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
) -> Vec<Instruction>
```

**Parameters:**

- `destination_stake_pubkey` - The public key of the stake account to merge into
- `source_stake_pubkey` - The public key of the stake account to merge from
- `authorized_pubkey` - The public key of the stake authority

**Returns:** A vector of instructions that need to be executed with `process_transaction`.

**Description:** Generates instructions to merge a source stake account into a destination stake account. Both accounts must have the same authorized staker and withdrawer, and be in a compatible state.

### `move_stake`

Creates an instruction to move stake between accounts.

```rust
pub fn move_stake(
    &mut self,
    source_stake_pubkey: &Pubkey,
    destination_stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    lamports: u64,
) -> Instruction
```

**Parameters:**

- `source_stake_pubkey` - The public key of the source stake account
- `destination_stake_pubkey` - The public key of the destination stake account
- `authorized_pubkey` - The public key of the stake authority
- `lamports` - The number of lamports worth of stake to move

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to move a specified amount of active stake from one stake account to another. Both accounts must be delegated to the same vote account.

### `move_lamports`

Creates an instruction to move lamports between stake accounts.

```rust
pub fn move_lamports(
    &mut self,
    source_stake_pubkey: &Pubkey,
    destination_stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    lamports: u64,
) -> Instruction
```

**Parameters:**

- `source_stake_pubkey` - The public key of the source stake account
- `destination_stake_pubkey` - The public key of the destination stake account
- `authorized_pubkey` - The public key of the stake authority
- `lamports` - The number of lamports to move

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to move excess lamports (not part of active stake) from one stake account to another.

---

## Delegation

### `delegate_stake`

Creates an instruction to delegate a stake account to a vote account.

```rust
pub fn delegate_stake(
    &mut self,
    stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    vote_pubkey: &Pubkey,
) -> Instruction
```

**Parameters:**

- `stake_pubkey` - The public key of the stake account to delegate
- `authorized_pubkey` - The public key of the stake authority
- `vote_pubkey` - The public key of the vote account to delegate to

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to delegate an initialized stake account to a validator's vote account.

### `deactivate_stake`

Creates an instruction to deactivate a stake account.

```rust
pub fn deactivate_stake(
    &mut self,
    stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
) -> Instruction
```

**Parameters:**

- `stake_pubkey` - The public key of the stake account to deactivate
- `authorized_pubkey` - The public key of the stake authority

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to deactivate a delegated stake account. After deactivation, the stake will enter a cooldown period before it can be withdrawn.

### `deactivate_delinquent_stake`

Creates an instruction to deactivate a delinquent validator's stake.

```rust
pub fn deactivate_delinquent_stake(
    &mut self,
    stake_pubkey: &Pubkey,
    delinquent_vote_account: &Pubkey,
    reference_vote_account: &Pubkey,
) -> Instruction
```

**Parameters:**

- `stake_pubkey` - The public key of the stake account to deactivate
- `delinquent_vote_account` - The public key of the delinquent vote account
- `reference_vote_account` - The public key of a non-delinquent vote account for comparison

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to deactivate stake delegated to a delinquent validator by providing a reference vote account that is not delinquent.

---

## Authorization

### `authorize`

Creates an instruction to authorize a new staker or withdrawer.

```rust
pub fn authorize(
    &mut self,
    stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    new_authorized_pubkey: &Pubkey,
    stake_authorize: StakeAuthorize,
    custodian_pubkey: Option<&Pubkey>,
) -> Instruction
```

**Parameters:**

- `stake_pubkey` - The public key of the stake account
- `authorized_pubkey` - The public key of the current authority being replaced
- `new_authorized_pubkey` - The public key of the new authority
- `stake_authorize` - The type of authorization to change (Staker or Withdrawer)
- `custodian_pubkey` - Optional custodian required if the stake is locked

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to change the staker or withdrawer authority of a stake account.

### `authorize_checked`

Creates an instruction to authorize a new staker or withdrawer with signature verification.

```rust
pub fn authorize_checked(
    &mut self,
    stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    new_authorized_pubkey: &Pubkey,
    stake_authorize: StakeAuthorize,
    custodian_pubkey: Option<&Pubkey>,
) -> Instruction
```

**Parameters:**

- `stake_pubkey` - The public key of the stake account
- `authorized_pubkey` - The public key of the current authority being replaced
- `new_authorized_pubkey` - The public key of the new authority (must sign)
- `stake_authorize` - The type of authorization to change (Staker or Withdrawer)
- `custodian_pubkey` - Optional custodian required if the stake is locked

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to change the staker or withdrawer authority of a stake account, requiring the new authority to sign the transaction.

---

## Lockup Management

### `set_lockup`

Creates an instruction to set the lockup parameters of a stake account.

```rust
pub fn set_lockup(
    &mut self,
    stake_pubkey: &Pubkey,
    unix_timestamp: Option<i64>,
    epoch: Option<u64>,
    custodian: Option<Pubkey>,
    custodian_pubkey: &Pubkey,
) -> Instruction
```

**Parameters:**

- `stake_pubkey` - The public key of the stake account
- `unix_timestamp` - Optional new Unix timestamp for the lockup
- `epoch` - Optional new epoch for the lockup
- `custodian` - Optional new custodian public key
- `custodian_pubkey` - The public key of the current custodian

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to modify the lockup configuration of a stake account. Only the custodian can modify the lockup while it is active.

### `set_lockup_checked`

Creates an instruction to set the lockup parameters with signature verification.

```rust
pub fn set_lockup_checked(
    &mut self,
    stake_pubkey: &Pubkey,
    unix_timestamp: Option<i64>,
    epoch: Option<u64>,
    custodian: Option<Pubkey>,
    custodian_pubkey: &Pubkey,
) -> Instruction
```

**Parameters:**

- `stake_pubkey` - The public key of the stake account
- `unix_timestamp` - Optional new Unix timestamp for the lockup
- `epoch` - Optional new epoch for the lockup
- `custodian` - Optional new custodian public key (must sign if provided)
- `custodian_pubkey` - The public key of the current custodian

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to modify the lockup configuration of a stake account, requiring the new custodian (if provided) to sign the transaction.

---

## Withdrawal

### `withdraw`

Creates an instruction to withdraw lamports from a stake account.

```rust
pub fn withdraw(
    &mut self,
    stake_pubkey: &Pubkey,
    withdrawer_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    lamports: u64,
    custodian_pubkey: Option<&Pubkey>,
) -> Instruction
```

**Parameters:**

- `stake_pubkey` - The public key of the stake account to withdraw from
- `withdrawer_pubkey` - The public key of the withdrawer authority
- `to_pubkey` - The public key of the destination account to receive the lamports
- `lamports` - The number of lamports to withdraw
- `custodian_pubkey` - Optional custodian required if the stake is locked

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction to withdraw lamports from a stake account to a destination account. The stake must be deactivated and the cooldown period must have elapsed for full withdrawal.

---

## Utility Methods

### `get_minimum_delegation`

Creates an instruction to get the minimum delegation amount.

```rust
pub fn get_minimum_delegation(&mut self) -> Instruction
```

**Returns:** An instruction that needs to be executed with `process_transaction`.

**Description:** Generates an instruction that queries the minimum amount of lamports required for a stake account delegation.

### `get_stake_account`

Retrieves and deserializes a stake account state.

```rust
pub fn get_stake_account(&self, stake_pubkey: &Pubkey) -> Option<StakeStateV2>
```

**Parameters:**

- `stake_pubkey` - The public key of the stake account to retrieve

**Returns:** An `Option<StakeStateV2>` containing the deserialized stake state, or `None` if the account does not exist or deserialization fails.

**Description:** Fetches the account data for a stake account and deserializes it into a `StakeStateV2` struct.

---