
# Installation

!!! important "Prerequisites"

    Before proceeding, ensure you have installed:

    - [Rust](https://www.rust-lang.org/tools/install) (stable version)
    - [Solana CLI](https://solana.com/docs/intro/installation)
    - [Anchor](https://www.anchor-lang.com/docs/installation)

  Check out [supported versions](#supported-versions) for version compatibility.

## Install system dependencies

Update your package list:

```bash
sudo apt update
sudo apt upgrade
```
Install the required packages:
```bash
sudo apt install build-essential
sudo apt-get install binutils-dev
sudo apt-get install libunwind-dev
```

## Install Trident

```bash
cargo install trident-cli
```

You can also use the `version` flag to install a specific version:
```bash
cargo install trident-cli --version x.y.z
```

## Install Hongfuzz and AFL

```bash
cargo install honggfuzz
cargo install cargo-afl
```
To install a specific version use the following commands:
```bash
cargo install honggfuzz --version x.y.z
cargo install cargo-afl --version x.y.z
```



## Supported versions

| ***{{ config.site_name }} CLI*** | ***Anchor*** | ***Solana*** | ***Rust*** | ***Honggfuzz*** | ***AFL*** |
|-:|-:|-:|-:|-:|-:|
| :material-developer-board: ***`develop`*** | `>=0.29.0` | `>=1.17.3` | `nightly` | `0.5.56` | `0.15.11` |
| :material-tag: ***`0.9.0`*** | `>=0.29.0` | `>=1.17.3` | `nightly` | `0.5.56` | `0.15.11` |
| :material-tag: ***`0.8.*`*** | `0.30.1` | `^1.17.4` | `nightly` | `0.5.56` | - |
| :material-tag: ***`0.7.0`*** | `>=0.29.*` | `^1.17.4` | `nightly` | `0.5.56` | - |
| :material-tag: ***`0.6.0`*** | `>=0.29.*` | `^1.17` | `nightly` | `0.5.55` | - |
| :material-tag: ***`0.5.0`*** | `~0.28.*` | `=1.16.6` | - | - | - |
| :material-tag: ***`0.4.0`*** | `~0.27.*` | `>=1.15`  | - | - | - |
| :material-tag: ***`0.3.0`*** | `~0.25.*` | `>=1.10`  | - | - | - |
| :material-tag: ***`0.2.0`*** | `~0.24.*` |  `>=1.9`  | - | - | - |

<!-- 1. To use Trident with Anchor 0.29.0, run the following commands from your project's root directory after Trident initialization:
```bash
cargo update anchor-client@0.30.0 --precise 0.29.0
cargo update anchor-spl@0.30.0 --precise 0.29.0
``` -->
