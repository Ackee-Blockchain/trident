# TridentAccount attributes

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

### `account(storage::name)`

Specifies which storage to use for the account. This determines where the account address is stored and managed.

`This attribute is optional`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(storage::name = owner)]
    pub owner: TridentAccount,
}
```

---


### `account(storage::account_id)`

Specifies a range of random numbers generated and mapped to address (Pubkey).

A bigger range results in more random account addresses being fuzzed.

`This attribute is optional`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(storage::name = owner)]
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

Specifies the program ID for PDA derivation. If not provided, the program ID of the instruction will be used.

`This attribute is applied at the field level and is used with seeds`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(
        storage = custom_pda,
        seeds = [b"seed"],
        program_id = pubkey!("11111111111111111111111111111111")
    )]
    pub custom_pda: TridentAccount,
}
```


---

### `account(lamports)`

Specifies the lamports for the account. If not provided, the default is 500 * LAMPORTS_PER_SOL.

`This attribute is optional but requires the storage attribute`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(
        storage = custom_pda,
        lamports = 5 * LAMPORTS_PER_SOL
    )]
    pub wallet: TridentAccount,
}
```

---

### `account(space)`

Specifies the space for the account. If not provided, the default is 0.

`This attribute is optional but requires the storage attribute and owner attribute`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(
        storage = custom_pda,
        space = 8 + 100,
        owner = pubkey!("program id goes here")
    )]
    pub custom_pda: TridentAccount,
}
```

---

### `account(owner)`

Specifies the owner for the account. If not provided, the system program is the default.

`This attribute is optional but requires the storage attribute`

```rust
#[derive(TridentAccounts)]
pub struct ExampleAccounts {
    #[account(
        storage = custom_pda,
        owner = pubkey!("program id goes here")
    )]
    pub custom_pda: TridentAccount,
}
```
