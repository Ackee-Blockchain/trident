---
hide:
  - navigation
---

# Installation

!!! important

    **Prerequisite**

    It is expected that you have installed:

    - Rust ([Install Rust](https://www.rust-lang.org/tools/install))
    - Solana CLI ([Install Solana CLI](https://docs.solanalabs.com/cli/install))
    - Anchor Framework ([Install Anchor](https://www.anchor-lang.com/docs/installation))

    For supported versions check the [Supported Versions](#supported-versions)

## Install System Dependencies

Update your package list and install the required packages:

```bash
sudo apt-get update
sudo apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    npm \
    vim \
    nano \
    wget \
    binutils-dev \
    libunwind-dev \
    lldb
```

## Install Hongfuzz

Install honggfuzz and AFL

```bash
cargo install honggfuzz

```
```bash
cargo install cargo-afl
```


## Install Trident

```bash
cargo install trident-cli

```

## Supported versions

| ***{{ config.site_name }} CLI*** | ***Anchor*** | ***Solana*** | ***Rust*** | ***Honggfuzz*** | ***AFL*** |
|-:|-:|-:|-:|-:|-:|
| :material-developer-board: ***`develop`*** | `>=0.29.0` | `>=1.17.3` | `nightly` | `0.5.56` | `0.15.11` |
| :material-tag: ***`0.8.*`*** | `>=0.29.0` | `>=1.17.3` | `nightly` | `0.5.56` | - |
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
