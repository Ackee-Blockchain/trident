# TridentAccounts

The `TridentAccounts` macro is used to derive required methods for account structures.


## Derived Traits

The macro implements the following trait:

- `AccountsMethods` - Methods to resolve accounts

!!! warning "Manual Implementation Note"
    There is no need to manually implement any methods of this trait. The macro handles all implementations automatically based on the account structure and attributes.

## Accounts Methods

!!! warning "Internal Method"
    These methods are used internally by Trident and is not expected to use them manually.

### `resolve_accounts`

Resolves all accounts based on their constraints and dependencies. The macro automatically analyzes seed dependencies and sorts fields in the correct resolution order.

```rust
fn resolve_accounts(
    &mut self,
    client: &mut impl FuzzClient,
    storage_accounts: &mut Self::IxAccounts,
    program_id: Pubkey,
    instruction_data: &Self::IxData,
)
```

---

### `to_account_meta`

Converts all accounts to AccountMeta format for Solana instructions.

```rust
fn to_account_meta(&mut self) -> Vec<AccountMeta>
```

---

### `capture_before`

Captures the state of accounts before transaction execution.

```rust
fn capture_before(&mut self, client: &mut impl FuzzClient)
```

---

### `capture_after`

Captures the state of accounts after transaction execution.

```rust
fn capture_after(&mut self, client: &mut impl FuzzClient)
```

---

## Struct-Level Attributes

These attributes are applied to the struct definition itself.

### `instruction_data`

Specifies the instruction data type that will be used with these accounts.

`This attribute is mandatory and applied at the struct level`

```rust
#[derive(TridentAccounts)]
#[instruction_data(ExampleInstructionData)]
pub struct ExampleAccounts {
    // fields...
}
```

---

### `storage`

Specifies the storage type for accounts, which is used to manage account state during fuzzing.

`This attribute is mandatory and applied at the struct level`

```rust
#[derive(TridentAccounts)]
#[storage(ExampleStorage)]
pub struct ExampleAccounts {
    // fields...
}
```

---

## Field-Level Attributes

These attributes are applied to individual account fields using the `#[account(...)]` syntax.

### `account(mut)`

Marks the account as mutable. This will set the `is_writable` flag to `true` in the generated `AccountMeta`.

`This attribute is optional`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(mut)]
    pub mutable_account: TridentAccount,
}
```

---

### `account(signer)`

Marks the account as a signer. This will set the `is_signer` flag to `true` in the generated `AccountMeta`.

`This attribute is optional`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(signer)]
    pub authority: TridentAccount,
}
```

---

### `account(address)`

Sets a fixed address for the account. Useful for program IDs that have known addresses.

`This attribute is optional`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}
```

---

### `account(storage)`

Specifies which storage to use for the account. This determines where the account address is stored and managed.

`This attribute is optional`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(storage = owner)]
    pub owner: TridentAccount,
}
```

---

### `account(skip_snapshot)`

Excludes the account from state snapshots. Useful for accounts that don't need to be tracked for state changes.

`This attribute is optional`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(skip_snapshot)]
    pub ignored_for_snapshots: TridentAccount,
}
```

---

### `account(seeds)`

Defines Program Derived Address (PDA) seeds for the account. The seeds can include references to other accounts in the struct, allowing for dependency-based PDA derivation.

`This attribute is optional but requires the storage attribute`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(storage = pdas, seeds = [b"seed", authority.key().as_ref()])]
    pub pda_account: TridentAccount,
}
```

---

### `account(program_id)`

Specifies the program ID for PDA derivation. If not provided, the program ID of Instruction will be used.

`This attribute is applied at the field level and is used with seeds`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(
        storage = custom_pda,
        seeds = [b"seed"],
        program_id = pubkey!("11111111111111111111111111111111"))]
    pub custom_pda: TridentAccount,
}
```
