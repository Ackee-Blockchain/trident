# Attributes

This macro accepts the following attributes:

### `mut`

Marks an account as mutable.

`This attribute is optional`

```rust
#[derive(Arbitrary, Debug, TridentAccounts)]
pub struct ExampleAccounts {
    pub token_program: TridentAccount,
    #[account(mut)]
    pub user: TridentAccount,
}
```

### `signer`

Marks an account as a signer.

`This attribute is optional`

```rust
#[derive(Arbitrary, Debug, TridentAccounts)]
pub struct ExampleAccounts {
    pub token_program: TridentAccount,
    #[account(signer)]
    pub user: TridentAccount,
}
```

### `storage`

Specifies the storage location for an account.

`This attribute is optional`

```rust
#[derive(Arbitrary, Debug, TridentAccounts)]
pub struct ExampleAccounts {
    #[account(storage = owner)]
    pub owner: TridentAccount,
}
```

!!! warning "PDA Storage Configuration"
    - For the first occurance of Program Derived Address (PDA) in the instruction, you need to insert it manually with `get_or_create` method in the `set_accounts` function.


### `address`

Specifies a fixed address for an account.

`This attribute is optional`

```rust
#[derive(Arbitrary, Debug, TridentAccounts)]
pub struct ExampleAccounts {
    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,
    pub user: TridentAccount,
}
```

### `skip_snapshot`

Marks an account to be excluded from state snapshot capture.

`This attribute is optional`

```rust
#[derive(Arbitrary, Debug, TridentAccounts)]
pub struct ExampleAccounts {
    #[account(skip_snapshot)]
    pub program: TridentAccount,
    pub user: TridentAccount,
}
```
