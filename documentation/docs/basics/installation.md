# Installation

Trident requires the following prerequisites:

!!! warning "Prerequisites"

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

!!! note "Version Table Legend"
    - :material-developer-board: = Develop branch
    - :material-tag: = Released version
    - `-` = Not required/supported

| **Version Type** | **Anchor** | **Solana** | **Rust** | **Honggfuzz** | **AFL** |
|:----------------|:-----------|:-----------|:---------|:--------------|:---------|
| Development :material-developer-board: | `>=0.29.0 < 0.31.0` | `>=1.17.3 < 2.1` | `nightly` | `0.5.56` | `0.15.11` |
| **Current (0.10.x)** :material-tag: | `>=0.29.0 < 0.31.0` | `>=1.17.3 < 2.1` | `nightly` | `0.5.56` | `0.15.11` |
| **0.9.x** :material-tag: | `>=0.29.0 < 0.31.0` | `>=1.17.3 < 2.1` | `nightly` | `0.5.56` | `0.15.11` |
| **0.8.x** :material-tag: | `0.30.1` | `^1.17.4` | `nightly` | `0.5.56` | `-` |
| **0.7.x** :material-tag: | `>=0.29.*` | `^1.17.4` | `nightly` | `0.5.56` | `-` |
| **0.6.x** :material-tag: | `>=0.29.*` | `^1.17` | `nightly` | `0.5.55` | `-` |
| **0.5.x** :material-tag: | `~0.28.*` | `=1.16.6` | `-` | `-` | `-` |
| **0.4.x** :material-tag: | `~0.27.*` | `>=1.15` | `-` | `-` | `-` |
| **0.3.x** :material-tag: | `~0.25.*` | `>=1.10` | `-` | `-` | `-` |
| **0.2.x** :material-tag: | `~0.24.*` | `>=1.9` | `-` | `-` | `-` |
