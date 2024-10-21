

# How To start fuzzing.

## To start fuzzing, follow these steps:

- Install ***Honggfuzz***

```bash
cargo install honggfuzz
```

For supported versions check https://ackee.xyz/trident/docs/latest/getting-started/getting-started/#supported-versions


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

- Link Account Context Aliases in the ***fuzz_instructions.rs*** with desired Snapshots

```rust
use hello_world::trident_fuzz_initialize_context_snapshot::InitializeContextAlias;
type InitializeFnSnapshot<'info> = InitializeContextAlias<'info>;
```

- Implement the ***todo!*** placeholders in ***fuzz_instructions.rs*** based on the provided descriptions.

- Run fuzzing with ***Honggfuzz***

```bash
trident fuzz run-hfuzz <FUZZ_TARGET>
```

### For more details, refer to the Trident documentation: https://ackee.xyz/trident/docs/dev/
