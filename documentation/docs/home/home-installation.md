## Dependencies

Check [supported versions](#supported-versions) section for further details.

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Solana tool suite](https://docs.solana.com/cli/install-solana-cli-tools)
- Install [Anchor](https://www.anchor-lang.com/docs/installation)
- Install [Honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs#how-to-use-this-crate) for fuzz testing

## Installation

```bash
cargo install trdelnik-cli

# or the specific version

cargo install --version <version> trdelnik-cli
```

### Supported versions

- We support `Anchor` and `Solana` versions specified in the table below.

| {{ config.site_name }} CLI |  Anchor   |   Solana  |          Rust          |
|--------------|:---------:|----------:|:-----------------------|
| `v0.6.0`     | `~0.29.*` | `<1.18 `  |  `nightly-2023-12-28`  |
| `v0.5.0`     | `~0.28.*` | `=1.16.6` |                        |
| `v0.4.0`     | `~0.27.*` | `>=1.15`  |                        |
| `v0.3.0`     | `~0.25.*` | `>=1.10`  |                        |
| `v0.2.0`     | `~0.24.*` |  `>=1.9`  |                        |
