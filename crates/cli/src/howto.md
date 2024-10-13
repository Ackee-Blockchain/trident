

# How To start fuzzing.

## To start fuzzing, follow these steps:

- Initialize ***Trident*** using

```bash
trident init
```

- Derive ***AccountsSnapshots*** for each account context in the program:

```rust
use trident_derive_accounts_snapshots::AccountsSnapshots;

#[derive(AccountsSnapshots, Accounts)]
pub struct InitializeContext<'info> {
// ...
}

```

- Apply ***instr_data*** for each castom type that is used as Instruction input:

```rust
use trident_instr_data::instr_data;

#[instr_data]
#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InputUpdatePrameters {
    pub input1: u8,
    pub input2: u8,
}

```

- Link Account Context Aliases in the ***fuzz_instructions.rs*** with desired Snapshots

```rust
use hello_world::trident_fuzz_initialize_context_snapshot::InitializeContextAlias;
type InitializeFnSnapshot<'info> = InitializeContextAlias<'info>;
```

- Implement the ***todo!*** placeholders in ***fuzz_instructions.rs*** based on the provided descriptions.

- Run fuzzing with ***Honggfuzz*** or ***AFL***

```bash
trident fuzz run-hfuzz
```

```bash
trident fuzz run-afl
```

### For more details, refer to the Trident documentation: https://ackee.xyz/trident/docs/dev/
