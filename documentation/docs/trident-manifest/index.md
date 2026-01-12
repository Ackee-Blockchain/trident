# Trident Manifest

Trident supports its own Trident Manifest, called `Trident.toml`. This section describes all possible configuration fields.

---

## Metrics

Configure fuzzing statistics and reporting.

```toml
[fuzz.metrics]
enabled = true
dashboard = false
json = false
```

**Fields:**

- `enabled` - Show and save statistics after the fuzzing session.
- `dashboard` - Save a HTML dashboard after the fuzzing session.
- `json` - Save a JSON file after the fuzzing session.

---

## Regression

Configure regression testing to track account state changes across fuzzing sessions.

```toml
[fuzz.regression]
enabled = true
```

**Fields:**

- `enabled` - Save a JSON file with all monitored states after the fuzzing session.

!!! note

    The feature hashes contents of accounts. If the contents depend on time, the final hash might not match the one from the first fuzzing session.

---

## Coverage

Configure code coverage analysis during fuzzing sessions.

```toml
[fuzz.coverage]
enable = true
server_port = 58432
loopcount = 5
format = "json"
attach_extension = true
```

**Fields:**

- `enable` - Collect code coverage data and generate a report upon completion. Default: `false`.
- `server_port` - HTTP server port for communication with the [Solana VS Code extension](https://marketplace.visualstudio.com/items?itemName=AckeeBlockchain.solana). Default: `58432`.
- `loopcount` - Number of execution flows before generating intermediate coverage data. Set to `0` for end-of-session only. Default: `0`.
- `format` - Output format for coverage reports: `json` or `html`. Default: `json`.
- `attach_extension` - Enable real-time coverage visualization in VS Code. Requires `loopcount > 0` and `format = "json"`. Default: `false`.

!!! warning

    Coverage is only gathered for programs deployed through an entrypoint!

??? note "Entrypoint deployment guide"

    #### Entrypoint deployment guide

    1. Comment out the program in `Trident.toml`

        ```toml
        # [[fuzz.programs]]
        # address = "example_address"
        # program = "../target/deploy/example_program.so"
        ```

    2. Add the program dependency to `Cargo.toml`

        ```toml
        [dependencies.example_program]
        path = "../programs/example_program"
        ```

    3. Add `syscall-v1` or `syscall-v2` feature depending on the version of `solana-program`. Programs written for `solana-program` v1.x should use `syscall-v1`, programs written for `solana-program` v2.x should use `syscall-v2` feature.

        ```toml
        [dependencies.trident-fuzz]
        features = ["syscall-v2"]
        ```

    4. Import the programs entrypoint in `test_fuzz.rs`

        ```rust
        use example_program::entry as example_entrypoint;
        ```

    5. Deploy the entrypoint in `test_fuzz.rs`

        ```rust
        impl FuzzTest {
            fn new() -> Self {
                let mut trident = Trident::default();

                let program = TridentEntrypoint::new(
                    example_program::ID,
                    None,
                    processor!(example_entrypoint)
                );
                trident.get_client().deploy_entrypoint(program);

                Self {
                    trident,
                    fuzz_accounts: FuzzAccounts::default(),
                }
            }
        }
        ```

---

## Programs

Load dumped programs from a desired cluster during fuzzing.

```toml
[[fuzz.programs]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
upgrade_authority = "account address goes here"
program = "metaplex-program/metaplex-token-metadata.so"
```

**Fields:**

- `address` - The program's public key.
- `upgrade_authority` - (Optional) The upgrade authority address.
- `program` - Path to the program's `.so` file.

---

## Accounts

Load dumped accounts from a desired cluster during fuzzing.

```toml
[[fuzz.accounts]]
address = "6YG3J7PaxyMnnbU67ifyrgF3BzNzc7cD8hPkqK6ATweE"
filename = "tests/accounts/core_bridge_mainnet/guardian_set_5_mock.json"
```

**Fields:**

- `address` - The account's public key.
- `filename` - Path to the JSON file containing the account data.

---

## Fork

Fork accounts directly from Solana clusters (mainnet, devnet, testnet) for use during fuzzing.

```toml
[[fuzz.fork]]
address = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
cluster = "m"
overwrite = false
```

**Fields:**

- `address` - The public key of the account to fork.
- `cluster` - The cluster to fetch from:

    - `m` or `mainnet` - Mainnet Beta
    - `d` or `devnet` - Devnet
    - `t` or `testnet` - Testnet
    - Custom RPC URL (e.g., `https://my-rpc.com`)

- `overwrite` - (Optional) Re-fetch the account even if cached. Default: `false`.

Forked accounts are cached locally in `.fuzz-artifacts/fork-cache/`. For upgradeable programs (v3 loader), Trident automatically fetches both the program account and its associated program data account.

!!! note "Loader Support"

    Trident detects and reports the loader type for each forked program:

    - **v3 loader (upgradeable)** - Fully supported with automatic program data fetching
    - **v1 loader (deprecated)** - Loaded as regular account
    - **v2 loader (bpf_loader)** - Loaded as regular account
    - **v4 loader** - Loaded as regular account (not fully supported)

See [Forking](../trident-advanced/forking/index.md) for advanced usage.

---
