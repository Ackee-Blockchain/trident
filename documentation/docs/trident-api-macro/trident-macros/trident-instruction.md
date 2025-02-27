# TridentInstruction

The `TridentInstruction` macro is used to derive required methods for `Instructions`.


## Derived trait

The macro implements the `InstructionMethods` trait with the corresponding methods:

!!! warning "Manual Implementation Note"
    There is no need to specify any method of this trait manually.

### `get_discriminator`

Get instruction discriminator

```rust
fn get_discriminator(&self) -> Vec<u8>;
```

---

### `get_program_id`

Get instruction program id

```rust
fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey;
```

---

### `set_snapshot_before`

Set account snapshots before transaction

```rust
fn set_snapshot_before(&mut self, client: &mut impl FuzzClient);
```

---

### `set_snapshot_after`

Set account snapshots after transaction

```rust
fn set_snapshot_after(&mut self, client: &mut impl FuzzClient);
```

---

### `to_account_metas`

Convert accounts to account metas

```rust
fn to_account_metas(&mut self) -> Vec<AccountMeta>;
```

---

### `resolve_accounts`

Resolve accounts

```rust
fn resolve_accounts(
    &mut self,
    client: &mut impl FuzzClient,
    ix_accounts: &mut Self::IxAccounts,
);
```

## Attributes

This macro accepts the following attributes:


### `program_id`

The program to which the Instruction belongs.

`This attribute is mandatory`

```rust
#[derive(Arbitrary, Debug, TridentInstruction)]
#[program_id("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")]
pub struct ExampleInstruction {
    pub accounts: ExampleInstructionAccounts,
    pub data: ExampleInstructionData,
}
```

### `discriminator`

The discriminator of the Instruction.

`This attribute is mandatory`

```rust
#[derive(Arbitrary, Debug, TridentInstruction)]
#[discriminator([33u8, 132u8, 147u8, 228u8, 151u8, 192u8, 72u8, 89u8])]
pub struct ExampleInstruction {
    pub accounts: ExampleInstructionAccounts,
    pub data: ExampleInstructionData,
}
```
