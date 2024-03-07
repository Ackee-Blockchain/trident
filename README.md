<div align="center">
  <img height="250" width="250" src="documentation/docs/images/Badge_Trdelnik.png" alt="Trident Logo"/>

  # Trident

  <a href="https://discord.gg/x7qXXnGCsa">
    <img src="https://discordapp.com/api/guilds/867746290678104064/widget.png?style=banner2" width="250" title="AckeeBlockchain/Trident discord" alt="Ackee Blockchain Discord invitation">
  </a>

  developed by [Ackee Blockchain](https://ackeeblockchain.com)

  [![Crates.io](https://img.shields.io/crates/v/trident-cli?label=CLI)](https://crates.io/crates/trident-cli)
  [![Crates.io](https://img.shields.io/crates/v/trident-test?label=Test)](https://crates.io/crates/trident-test)
  [![Crates.io](https://img.shields.io/crates/v/trident-client?label=Client)](https://crates.io/crates/trident-client)
  [![Crates.io](https://img.shields.io/crates/v/trident-explorer?label=Explorer)](https://crates.io/crates/trident-explorer)
  <br />
  [![lint](https://github.com/Ackee-Blockchain/trident/actions/workflows/lint.yml/badge.svg)](https://github.com/Ackee-Blockchain/trident/actions/workflows/lint.yml)
  [![Test Escrow and Turnstile](https://github.com/Ackee-Blockchain/trident/actions/workflows/run_examples.yml/badge.svg)](https://github.com/Ackee-Blockchain/trident/actions/workflows/run_examples.yml)
</div>

Trident is a Rust-based framework for Fuzz Tests and Integration Tests of Solana programs written in [Anchor](https://www.anchor-lang.com/), enabling automated generation of test templates and custom invariant checks to identify and prevent undesired behaviors using Rust's [Arbitrary crate](https://docs.rs/arbitrary/latest/arbitrary/) and [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs).


## Features



- **Automated Test Generation**: Simplifies the testing process by automatically creating templates for fuzz and integration tests for programs written using the Anchor Framework.
- **Dynamic Data Generation**: Increases coverage with random instruction data and pseudo-random accounts for unpredictable fuzz test scenarios.
- **Custom Instruction Sequences**: Provides the flexibility to design specific sequences of instructions to meet particular testing needs or to focus on particular aspects of program behavior during fuzz testing.
- **Invariant Checks**: Allows for custom pre- and post-execution invariants checks to spot vulnerabilities and unwanted behaviors.

## Prerequisites
Check [supported versions](#supported-versions) section for further details.
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Solana tool suite](https://docs.solana.com/cli/install-solana-cli-tools)
- Install [Anchor](https://www.anchor-lang.com/docs/installation)
- Install [Honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs#how-to-use-this-crate) for fuzz testing

## Installation

```shell
cargo install trident-cli

# or the specific version

cargo install --version <version> trident-cli
```

In order to install [Honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs#how-to-use-this-crate) run:
```shell
# installs hfuzz and honggfuzz subcommands in cargo
cargo install honggfuzz
```
## Quick Start
To initialize Trident in your Anchor-based Solana project, begin by executing the following command from the root folder of your project:
```bash
# will generate test templates for fuzz and integration tests
trident init
```
If you are interested in **specific test types**, such as **Fuzz Tests** or **Integration Tests**, run:
```bash
# generate fuzz tests template
trident init fuzz
```
```bash
# generate integration tests template
trident init poc
```
Next, enter `trident --help` to access basic information on usage.

## External Documentation
For more detailed information, visit our [documentation](http://127.0.0.1:8000/trident/docs/).



## Supported versions

- We support `Anchor` and `Solana` versions specified in the table below.

| Trident CLI |  Anchor   |   Solana  |          Rust          |
|--------------|:---------:|----------:|:-----------------------|
| `v0.6.0`     | `~0.29.*` | `<1.18 `  |  `nightly-2023-12-28`  |
| `v0.5.0`     | `~0.28.*` | `=1.16.6` |                        |
| `v0.4.0`     | `~0.27.*` | `>=1.15`  |                        |
| `v0.3.0`     | `~0.25.*` | `>=1.10`  |                        |
| `v0.2.0`     | `~0.24.*` |  `>=1.9`  |                        |



## Awards

**Marinade Community Prize** - winner of the [Marinade grant](https://solana.blog/riptide-hackathon-winners/) for the 2022 Solana Riptide Hackathon.

## Contribution

Thank you for your interest in contributing to Trident! Please see the [CONTRIBUTING.md](./CONTRIBUTING.md) to learn how.

## License

This project is licensed under the [MIT license](https://github.com/Ackee-Blockchain/trident/blob/master/LICENSE).

## University and investment partners

- [Czech technical university in Prague](https://www.cvut.cz/en)
- [Ackee](https://www.ackee.cz/)
- [Rockaway Blockchain Fund](https://rbf.capital/)
