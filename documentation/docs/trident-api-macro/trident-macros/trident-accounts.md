# TridentAccounts

The `TridentAccounts` macro is used to derive required methods for `account structures`.



## Derived trait

The macro implements the `AccountsMethods` trait with the corresponding methods:

!!! warning "Manual Implementation Note"
    There is no need to specify any method of this trait manually.

### `resolve_accounts`

This method resolves accounts based on the specified `account` attributes.

```rust
fn resolve_accounts(
    &mut self,
    client: &mut impl FuzzClient,
    ix_accounts: &mut Self::IxAccounts,
);
```

---

### `to_account_meta`

Convert accounts to account metas

```rust
fn to_account_meta(&mut self) -> Vec<AccountMeta>;
```

---

### `capture_before`

Capture account state before transaction execution

```rust
fn capture_before(&mut self, client: &mut impl FuzzClient);
```

---

### `capture_after`

Capture account state after transaction execution

```rust
fn capture_after(&mut self, client: &mut impl FuzzClient);
```

## Attributes

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
