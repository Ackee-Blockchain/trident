<p align="center">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://abchprod.wpengine.com/wp-content/uploads/2024/05/Trident-Github.png?raw=true">
      <img alt="Trident Github" src="https://abchprod.wpengine.com/wp-content/uploads/2024/05/Trident-Github.png?raw=true" width="auto">
    </picture>
  </a>
</p>

<p align="left">
  <img height="100" width="100" src="https://abchprod.wpengine.com/wp-content/uploads/2024/05/Trident-Color.png" alt="Trident"/>

# Trident

<p align="left">
  Rust-based framework to Fuzz and Integration test Solana programs to help you ship secure code.
<p>

<p align="left">
<a href="https://discord.gg/JhTVXUvaEr" target="_blank" rel="noopener noreferrer">
   <picture>
     <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/discord/867746290678104064?colorA=21262d&colorB=0000FF&style=flat">
     <img src="https://img.shields.io/discord/867746290678104064?colorA=f6f8fa&colorB=0000FF&style=flat" alt="Chat">
   </picture>
 </a>
 <a href="https://crates.io/crates/trident-cli" target="_blank" rel="noopener noreferrer">
   <picture>
     <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/crates/v/trident-cli?colorA=21262d&colorB=21262d&style=flat">
     <img src="https://img.shields.io/crates/v/trident-cli?colorA=f6f8fa&colorB=f6f8fa&style=flat" alt="Version">
   </picture>
 </a>
 <a href="https://github.com/Ackee-Blockchain/trident/blob/master/LICENSE" target="_blank" rel="noopener noreferrer">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/npm/l/@coinbase/onchainkit?colorA=21262d&colorB=21262d&style=flat">
      <img src="https://img.shields.io/npm/l/@coinbase/onchainkit?colorA=f6f8fa&colorB=f6f8fa&style=flat" alt="MIT License">
    </picture>
  </a>
  <a href="https://github.com/Ackee-Blockchain/trident/actions/workflows/lint.yml" target="_blank" rel="noopener noreferrer">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/Ackee-Blockchain/trident/lint.yml?label=Lint&colorA=21262d&style=flat">
      <img src="https://img.shields.io/github/actions/workflow/status/Ackee-Blockchain/trident/lint.yml?label=Lint&colorA=f6f8fa&style=flat" alt="Lint">
    </picture>
  </a>
  <a href="https://github.com/Ackee-Blockchain/trident/actions/workflows/run_examples.yml" target="_blank" rel="noopener noreferrer">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/Ackee-Blockchain/trident/run_examples.yml?label=Test%20Escrow%20and%20Turnstile&colorA=21262d&style=flat">
      <img src="https://img.shields.io/github/actions/workflow/status/Ackee-Blockchain/trident/run_examples.yml?label=Test%20Escrow%20and%20Turnstile&colorA=f6f8fa&style=flat" alt="Test Escrow and Turnstile">
    </picture>
  </a>
</p>

<br />

## Documentation

For documentation and guides, visit [ackee.xyz/trident/docs](https://ackee.xyz/trident/docs/).

## Prerequisites
Check [Supported versions](https://ackee.xyz/trident/docs/home/home-installation/#supported-versions) section for further details.
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Solana tool suite](https://docs.solana.com/cli/install-solana-cli-tools)
- Install [Anchor](https://www.anchor-lang.com/docs/installation)
- Install [Honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs#how-to-use-this-crate) for fuzz testing

## Installation

```shell
cargo install trident-cli
```
</p>

<br />

## Features

### Automated Test Generation
Simplifies the testing process by automatically creating templates for fuzz and integration tests for programs written using the [Anchor Framework](https://www.anchor-lang.com/).

### Dynamic Data Generation
Increases test coverage with random instruction data and pseudo-random accounts for unpredictable fuzz test scenarios.

### Custom Instruction Sequences
Provides the flexibility to design specific sequences of instructions to meet particular testing needs or to focus on particular aspects of program behavior during fuzz testing.

### Invariant Checks
Allows for custom invariants checks to spot vulnerabilities and unwanted behaviors.
</p>

<br />

## Community

Check out the following places for more Trident-related content:

- Follow on [Twitter/X](https://twitter.com/TridentSolana) & [Warpcast](https://warpcast.com/~/channel/trident) for project updates
- Join the discussions on our Trident [warpcast channel](https://warpcast.com/~/channel/trident) or our [discord channel](https://discord.gg/wyBW9Q23aJ)

## Grants

Solana Foundation             |  Marinade
:-------------------------:|:-------------------------:
[![](https://abchprod.wpengine.com/wp-content/uploads/2024/05/Solana-Foundation.png)](https://ackee.xyz/blog/introducing-trident-the-first-open-source-fuzzer-for-solana-programs/)  |  [![](https://abchprod.wpengine.com/wp-content/uploads/2024/05/Marinade.png)](https://solana.blog/riptide-hackathon-winners/)

## Contribution

Thank you for your interest in contributing to Trident! Please see the [CONTRIBUTING.md](./CONTRIBUTING.md) to learn how.

## License

This project is licensed under the [MIT license](https://github.com/Ackee-Blockchain/trident/blob/master/LICENSE).
