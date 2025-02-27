# FuzzAccounts

`FuzzAccounts` is a struct that contains `all the accounts that can be used in the fuzz test` i.e. `storage` for accounts.

By default, Trident generates the struct based on the program's idl, i.e. for each account Trident generates a field in the struct.

On demand, you can add your own accounts to the struct, these accounts are meant to be used within the `set_accounts`, `set_remaining_accounts` and potentially within the `set_data` methods, with the corresponding account indexes.


```rust
pub struct FuzzAccounts {
    pub account1: AccountsStorage,
    pub account2: AccountsStorage,
    pub account3: AccountsStorage,
    pub account4: AccountsStorage,
    pub account5: AccountsStorage,
    pub account6: AccountsStorage,
    /// ....
}
```
## Implemented Methods

The following section contains the methods that are available for the `AccountsStorage` struct.


### `is_empty`

Returns true if no accounts are stored.
```rust
fn is_empty(&self) -> bool
```

---

### `get_or_create`

Gets an existing account or creates a new one with specified metadata.
```rust
fn get_or_create(
    account_id: AccountId,
    client: &mut impl FuzzClient,
    seeds: Option<PdaSeeds>,
    account_metadata: Option<AccountMetadata>
) -> Pubkey
```

---

### `get_or_create_token_account`

!!! warning "Token Feature Requirement"
    This method is only available if the `token` feature is enabled.

Creates or retrieves a token account with specified parameters.
```rust
fn get_or_create_token_account(
    account_id: AccountId,
    client: &mut impl FuzzClient,
    seeds: Option<PdaSeeds>,
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
    delegate: Option<Pubkey>,
    is_native: bool,
    delegated_amount: u64,
    close_authority: Option<Pubkey>
) -> Pubkey
```

---

### `get_or_create_mint_account`

!!! warning "Token Feature Requirement"
    This method is only available if the `token` feature is enabled.

Creates or retrieves a mint account for tokens.
```rust
fn get_or_create_mint_account(
    account_id: AccountId,
    client: &mut impl FuzzClient,
    seeds: Option<PdaSeeds>,
    decimals: u8,
    owner: &Pubkey,
    freeze_authority: Option<Pubkey>
) -> Pubkey
```

---

### `get_or_create_delegated_account`

!!! warning "Stake Feature Requirement"
    This method is only available if the `stake` feature is enabled.

Creates or retrieves a delegated stake account.
```rust
fn get_or_create_delegated_account(
    account_id: AccountId,
    client: &mut impl FuzzClient,
    seeds: Option<PdaSeeds>,
    voter_pubkey: Pubkey,
    staker: Pubkey,
    withdrawer: Pubkey,
    stake: u64,
    activation_epoch: Epoch,
    deactivation_epoch: Option<Epoch>,
    lockup: Option<Lockup>
) -> Pubkey
```

---

### `get_or_create_initialized_account`

!!! warning "Stake Feature Requirement"
    This method is only available if the `stake` feature is enabled.

Creates or retrieves an initialized stake account.
```rust
fn get_or_create_initialized_account(
    account_id: AccountId,
    client: &mut impl FuzzClient,
    seeds: Option<PdaSeeds>,
    staker: Pubkey,
    withdrawer: Pubkey,
    lockup: Option<Lockup>
) -> Pubkey
```

---

### `get_or_create_vote_account`

!!! warning "Vote Feature Requirement"
    This method is only available if the `vote` feature is enabled.

Creates or retrieves a vote account.
```rust
fn get_or_create_vote_account(
    account_id: AccountId,
    client: &mut impl FuzzClient,
    seeds: Option<PdaSeeds>,
    node_pubkey: &Pubkey,
    authorized_voter: &Pubkey,
    authorized_withdrawer: &Pubkey,
    commission: u8,
    clock: &Clock
) -> Pubkey
```


## PdaSeeds and AccountMetadata

The `PdaSeeds` and `AccountMetadata` types are optional parameters that can be used to create a Program Derived Address (`Pda`) and pass additional metadata to the account creation process, respectively.

### `PdaSeeds`

```rust
pub struct PdaSeeds<'a> {
    pub seeds: &'a [&'a [u8]],
    pub program_id: Pubkey,
}
impl<'a> PdaSeeds<'a> {
    pub fn new(seeds: &'a [&'a [u8]], program_id: Pubkey) -> Self {
        Self { seeds, program_id }
    }
}
```

---

### `AccountMetadata`

```rust
pub struct AccountMetadata {
    pub lamports: u64,
    pub space: usize,
    pub owner: Pubkey,
}
impl AccountMetadata {
    pub fn new(lamports: u64, space: usize, owner: Pubkey) -> Self {
        Self {
            lamports,
            space,
            owner,
        }
    }
}
```
