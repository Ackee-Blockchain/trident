# Installation

Trident requires the following prerequisites:

!!! warning "Prerequisites"

    Before proceeding, ensure you have installed:

    - [Rust](https://www.rust-lang.org/tools/install) (stable version)
    - [Solana CLI](https://solana.com/docs/intro/installation)
    - [Anchor](https://www.anchor-lang.com/docs/installation)

  Check out [supported versions](#supported-versions) for version compatibility.

## Install Trident

```bash
cargo install trident-cli
```

You can also use the `version` flag to install a specific version:
```bash
cargo install trident-cli --version x.y.z
```

## Install cargo-llvm-cov

To enable code coverage tracking during fuzzing, install [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov):
```bash
cargo +stable install cargo-llvm-cov --locked
```


## Supported versions

!!! note "Version Table Legend"
    - :material-developer-board: = Develop branch
    - :material-tag: = Released version
    - `-` = Not required/supported

| **Version Type**                       | **Anchor**          | **Solana SDK**       | **Rust**  | **Honggfuzz** | **AFL**   |
| :------------------------------------- | :------------------ | :--------------- | :-------- | :------------ | :-------- |
| Development :material-developer-board: | `>=0.29.0`          | `2.3`            | `1.86`    | `-`           | `-`       |
| **Current (0.12.x)** :material-tag:    | `>=0.29.0 `         | `2.3`            | `1.86`    | `-`           | `-`       |
| **0.11.x** :material-tag:              | `>=0.29.0 `         | `>=1.17.3`       | `1.86`    | `-`           | `-`       |
| **0.10.x** :material-tag:              | `>=0.29.0 < 0.31.0` | `>=1.17.3 < 2.1` | `nightly` | `0.5.56`      | `0.15.11` |
| **0.9.x** :material-tag:               | `>=0.29.0 < 0.31.0` | `>=1.17.3 < 2.1` | `nightly` | `0.5.56`      | `0.15.11` |
| **0.8.x** :material-tag:               | `0.30.1`            | `^1.17.4`        | `nightly` | `0.5.56`      | `-`       |
| **0.7.x** :material-tag:               | `>=0.29.*`          | `^1.17.4`        | `nightly` | `0.5.56`      | `-`       |
| **0.6.x** :material-tag:               | `>=0.29.*`          | `^1.17`          | `nightly` | `0.5.55`      | `-`       |
| **0.5.x** :material-tag:               | `~0.28.*`           | `=1.16.6`        | `-`       | `-`           | `-`       |
| **0.4.x** :material-tag:               | `~0.27.*`           | `>=1.15`         | `-`       | `-`           | `-`       |
| **0.3.x** :material-tag:               | `~0.25.*`           | `>=1.10`         | `-`       | `-`           | `-`       |
| **0.2.x** :material-tag:               | `~0.24.*`           | `>=1.9`          | `-`       | `-`           | `-`       |
