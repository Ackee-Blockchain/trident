# AccountsSnapshots

Trident provides AccountsSnapshots macro that can be derived for each Instruction Context you have specified in your Solana Program for real-time updates of the Instruction Context.

!!! important

    AccountsSnapshots macro requires additional deps and source code additions in your Solana Program. On the other hand can provide real-time updates of Context modifications, instead of re-generating the `accounts_snapshots.rs`.



## Add required imports

To the Cargo.toml of the Solana Program add the following feature and deps.

```toml
[package]
name = "..."
version = "0.1.0"
description = "Created with Anchor"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]
name = "..."

[features]
default = []
cpi = ["no-entrypoint"]
no-entrypoint = []
no-idl = []
no-log-ix-name = []
idl-build = ["anchor-lang/idl-build", "anchor-spl/idl-build"]
# add the feature below
# ---
trident-fuzzing = ["dep:trident-fuzz"]
# ---


[dependencies]
# ---
# add the following Trident deps with the desired version
trident-derive-accounts-snapshots = "..."
trident-fuzz = { version = "...", optional = true }
# ---
# Your deps
anchor-lang = "0.30.1"
anchor-spl = "0.30.1"
...
```

## Derive the AccountsSnapshots

For each context in your program you can derive the AccountsSnapshots.

```rust
use trident_derive_accounts_snapshots::AccountsSnapshots;

// ...

#[derive(AccountsSnapshots, Accounts)]
pub struct TheContextName<'info> {
    #[account(...)]
    pub account1: ...<'info>,

    #[account(...)]
    pub account2: ...<'info, ...>,

    #[account(
        ...
     )]
    pub account3: ...,

    // additional accounts
}
```


## Use AccountsSnapshots in the Fuzz Tests

### Use defined feature

In the Cargo.toml within the `trident-tests/fuzz_tests`. Activate the new feature next to the program dependency.

```toml
[dependencies.<PROGRAM_NAME>]
path = "../../programs/<PROGRAM_NAME>"
features = ["trident-fuzzing"]
```

### Use the derived AccountsSnapshots within the Fuzz Test

Now you can use the derived AccountsSnapshots within the Fuzz Test instead of using generated `accounts_snapshots.rs`.

## Example

!!! tip

    Check the [Fuzz Example](https://github.com/Ackee-Blockchain/trident/tree/master/examples/fuzz-tests/arbitrary-limit-inputs-5) that uses the AccountsSnapshots macro.
