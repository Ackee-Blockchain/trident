

# New fuzz test template successfully added.

## To start fuzzing, follow these steps:

- Derive AccountsSnapshots for each account context in the program:

Include the following dependencies in the Cargo.toml of each program:
```rust
trident-derive-accounts-snapshots = "0.8.0"
trident-fuzz = { version = "0.8.0", optional = true }
```

- Add the fuzzing feature:
```toml
trident-fuzzing = ["dep:trident-fuzz"]
```

- Link Account Context Aliases in the `fuzz_instructions.rs` with desired Snapshots
For example:
```rust
use hello_world::trident_fuzz_initialize_context_snapshot::InitializeContextAlias;
type InitializeFnSnapshot<'info> = InitializeContextAlias<'info>;
```

- Implement the `todo!` placeholders in `fuzz_instructions.rs` based on the provided descriptions.

### For more details, refer to the Trident documentation: https://ackee.xyz/trident/docs/dev/
