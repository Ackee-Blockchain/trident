# Installation

## Dependencies

Check [supported versions](#supported-versions) section for further details.

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Solana tool suite](https://docs.solana.com/cli/install-solana-cli-tools)
- Install [Anchor](https://www.anchor-lang.com/docs/installation)
- Install [Honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs#how-to-use-this-crate) for fuzz testing

## Installation

```bash
cargo install trident-cli

# or the specific version

cargo install --version <version> trident-cli
```

### Supported versions

- We support `Anchor` and `Solana` versions specified in the table below.

| {{ config.site_name }} CLI | Anchor | Solana | Rust |
|--------------|:---------:|----------:|:-----------------------|
| `v0.7.0`     | `>=0.29.*`<sup>1</sup> | `^1.17.4`  | `nightly` |
| `v0.6.0`     | `>=0.29.*`<sup>1</sup> | `^1.17`  | `nightly` |
| `v0.5.0`     | `~0.28.*` | `=1.16.6` |                        |
| `v0.4.0`     | `~0.27.*` | `>=1.15`  |                        |
| `v0.3.0`     | `~0.25.*` | `>=1.10`  |                        |
| `v0.2.0`     | `~0.24.*` |  `>=1.9`  |                        |

1. To use Trident with Anchor 0.29.0, run the following commands from your project's root directory after Trident initialization:
```bash
cargo update anchor-client@0.30.0 --precise 0.29.0
cargo update anchor-spl@0.30.0 --precise 0.29.0
```
