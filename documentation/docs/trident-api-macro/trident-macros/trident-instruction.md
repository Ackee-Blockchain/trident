# TridentInstruction

The `TridentInstruction` macro is used to derive required methods for `Instructions`. This procedural macro automatically implements instruction-related functionality for structs that represent instructions.


!!! warning "Required Field Names"
    The macro parser strictly requires the struct to have an `accounts` field named exactly `accounts`. If a remaining accounts field is used, it must be named exactly `remaining_accounts`.

## Derived Traits

The macro implements the following traits:

- `InstructionGetters` - Methods to retrieve instruction data
- `InstructionSetters` - Methods to set up instruction state

!!! warning "Manual Implementation Note"
    There is no need to manually implement the getter, setter, or hook methods. The macro handles these implementations automatically based on the structure of your instruction.

## Instruction Getters

!!! warning "Internal Method"
    These methods are used internally by Trident and is not expected to use them manually.

### `get_discriminator`

Returns the instruction discriminator (identifier bytes) that uniquely identifies this instruction of the program.

```rust
fn get_discriminator(&self) -> Vec<u8>
```

---

### `get_program_id`

Returns the program ID that will process this instruction.

```rust
fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey
```

---

### `to_account_metas`

Converts all accounts to AccountMeta format for Solana instructions.

```rust
fn to_account_metas(&mut self) -> Vec<AccountMeta>
```

---

## Instruction Setters

!!! warning "Internal Method"
    These methods are used internally by Trident and is not expected to use them manually.

### `set_snapshot_before`

Captures the state of all accounts before instruction execution.

```rust
fn set_snapshot_before(&mut self, client: &mut impl FuzzClient)
```

---

### `set_snapshot_after`

Captures the state of all accounts after instruction execution.

```rust
fn set_snapshot_after(&mut self, client: &mut impl FuzzClient)
```

---

### `resolve_accounts`

Resolves all accounts needed for this instruction.

```rust
fn resolve_accounts(
    &mut self,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut Self::IxAccounts,
)
```

---


## Struct-Level Attributes

These attributes are applied to the struct definition itself.

### `program_id`

Specifies the program ID that will process this instruction. This can be provided as a string literal containing a base58-encoded public key.

`This attribute is mandatory`

```rust
#[derive(Arbitrary, Debug, TridentInstruction)]
#[program_id("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")]
pub struct ExampleInstruction {
    pub accounts: ExampleInstructionAccounts,
    pub data: ExampleInstructionData,
}
```

---

### `discriminator`

Specifies the instruction discriminator bytes that uniquely identify this instruction to the program. This is typically an 8-byte array.

`This attribute is mandatory`

```rust
#[derive(Arbitrary, Debug, TridentInstruction)]
#[discriminator([33u8, 132u8, 147u8, 228u8, 151u8, 192u8, 72u8, 89u8])]
pub struct ExampleInstruction {
    pub accounts: ExampleInstructionAccounts,
    pub data: ExampleInstructionData,
}
```
