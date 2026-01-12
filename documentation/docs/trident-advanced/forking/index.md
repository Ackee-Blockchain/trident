# Forking

Forking allows you to fetch accounts directly from Solana clusters (mainnet, devnet, testnet) and use them during fuzzing. This is useful for testing your program against real-world state, such as existing programs, token mints, or any on-chain accounts.

## Configuration

Add fork entries to your `Trident.toml`:

```toml
[[fuzz.fork]]
address = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
cluster = "m"

[[fuzz.fork]]
address = "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD"
cluster = "mainnet"
overwrite = true
```

**Fields:**

- `address` - The public key of the account to fork
- `cluster` - The cluster to fetch from:

    - `m` or `mainnet` - Mainnet Beta
    - `d` or `devnet` - Devnet
    - `t` or `testnet` - Testnet
    - Custom RPC URL (e.g., `https://my-rpc.com`)

- `overwrite` - (Optional) Force re-fetch even if cached

## How It Works

When you run `trident fuzz`, Trident processes forks before fuzzing begins.

**Key behaviors:**

1. **Caching** - Accounts are cached in `.fuzz-artifacts/fork-cache/` as JSON files. Subsequent runs use the cache unless `overwrite = true`.

2. **Program handling** - For v3 upgradeable programs, Trident automatically fetches both the program account and its program data account.

3. **Thread safety** - All RPC calls happen once in the main thread before parallel fuzzing begins. Worker threads load from cache.

## Loader Detection

Trident detects and reports the loader type for each forked program:

| Loader | Detection Message | Support |
|--------|-------------------|---------|
| v1 (deprecated) | `Detected v1 loader (deprecated)` | Loaded as regular account |
| v2 (bpf_loader) | `Detected v2 loader (bpf_loader)` | Loaded as regular account |
| v3 (upgradeable) | `Detected v3 loader (upgradeable)` | Full support with program data |
| v4 | `Detected v4 loader (not fully supported)` | Loaded as regular account |


