<p align="center">
  <a href="https://usetrident.xyz/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://abchprod.wpengine.com/wp-content/uploads/2025/09/Trident-Github-Updated.png?raw=true">
      <img alt="Trident Github" src="https://abchprod.wpengine.com/wp-content/uploads/2025/09/Trident-Github-Updated.png?raw=true" width="auto">
    </picture>
  </a>
</p>

# Trident

<p align="left">
  The first and only manually-guided fuzzing framework for Solana programs written in Rust, processing up to 12,000 tx/s. <br/>

  Granted by the Solana Foundation, securing Kamino.
  
  
<p>

<p align="left">
<a href="https://usetrident.xyz/" target="_blank" rel="noopener noreferrer">
   <picture>
     <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/website-usetrident.xyz-blue?colorA=21262d&colorB=0000FF&style=flat">
     <img src="https://img.shields.io/badge/website-usetrident.xyz-blue?colorA=f6f8fa&colorB=0000FF&style=flat" alt="Website">
   </picture>
 </a>
<a href="https://ackee.xyz/trident/docs/latest/" target="_blank" rel="noopener noreferrer">
   <picture>
     <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/docs-ackee.xyz-blue?colorA=21262d&colorB=0000FF&style=flat">
     <img src="https://img.shields.io/badge/docs-ackee.xyz-blue?colorA=f6f8fa&colorB=0000FF&style=flat" alt="Documentation">
   </picture>
 </a>
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
  <a href="https://github.com/Ackee-Blockchain/trident/actions/workflows/fuzz.yml" target="_blank" rel="noopener noreferrer">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/Ackee-Blockchain/trident/fuzz.yml?label=Test%20Fuzz%20Tests&colorA=21262d&style=flat">
      <img src="https://img.shields.io/github/actions/workflow/status/Ackee-Blockchain/trident/fuzz.yml?label=Test%20Fuzz%20Tests&colorA=f6f8fa&style=flat" alt="Test Escrow and Turnstile">
    </picture>
  </a>
</p>

<br />

## Why Trident?

- Executes **thousands of transactions per second** to stress your program at Solana speed.  
- Models **state changes and flows** that unit tests miss.  
- Surfaces **edge cases, overflows, and missing constraints** early in development.  
- Built and maintained by **[Ackee Blockchain Security](https://ackee.xyz)**, trusted auditors of Lido, Safe, and Axelar.  
- Supported by the **Solana Foundation**.  

<br />

## Features & benefits

- **Manually-guided fuzzer** – Define custom strategies to explore tricky code paths.  
- **Stateful fuzzing** – Inputs are generated based on critical account state changes.  
- **Anchor-like macros** – Write fuzz tests with familiar, clean syntax.  
- **TridentSVM client** – Execution using Anza’s Solana SVM API.  
- **Property-based testing** – Compare account states before and after execution.  
- **Flow-based sequence control** – Combine multiple instructions into realistic transaction patterns.  
- **Regression testing** – Compare fuzzing results between program versions.  

<br />

## Quick start

Install via Cargo:

```shell
cargo install trident-cli
```

Write your first fuzz test:

```rust
#[init]
fn start(&mut self) {
  // Build Initialize Transaction
  let mut tx = InitializeTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

  // Execute Initialize Transaction
  self.trident
        .execute_transaction(&mut tx, Some("Initialize"));
}

#[flow]
fn flow1(&mut self) {
    // Build MoveEast Transaction
    let mut tx = MoveEastTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

    // Execute MoveEast Transaction
    self.trident.execute_transaction(&mut tx, Some("MoveEast"));
}
#[flow]
fn flow2(&mut self) {
    // Build MoveSouth Transaction
    let mut tx = MoveSouthTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
    
    // Execute MoveSouth Transaction
    self.trident.execute_transaction(&mut tx, Some("MoveSouth"));
}
```

Run it:

```shell
trident fuzz run <fuzz_test>
```

For full examples and guides, see the [documentation](https://ackee.xyz/trident/docs/latest/trident-examples/trident-examples/).

<br />

## Installation

Latest release: **0.11.1**

```shell
cargo install trident-cli
```

<br />

## Use cases

- **Audit preparation** – Run fuzz campaigns before submitting your code for review.  
- **Continuous security** – Integrate Trident into CI for ongoing regression testing.  
- **Research & prototyping** – Generate complex attack sequences programmatically.  

<br />

## Documentation

- [Getting Started](https://ackee.xyz/trident/docs/latest/#getting-started)  
- [Advanced Customization](https://ackee.xyz/trident/docs/latest/trident-advanced/)  
- [Examples & Pipelines](https://ackee.xyz/trident/docs/latest/trident-examples/trident-examples/)  
- [API & Macro Reference](https://ackee.xyz/trident/docs/latest/trident-api-macro/)  

<br />

## Community

Check out the following places for more Trident-related content:

- Follow on [Twitter/X](https://twitter.com/TridentSolana) for updates
- Join the discussions on our Trident [discord channel](https://discord.gg/wyBW9Q23aJ)

## Grants

Solana Foundation             |  Marinade
:-------------------------:|:-------------------------:
[![](https://abchprod.wpengine.com/wp-content/uploads/2024/05/Solana-Foundation.png)](https://ackee.xyz/blog/introducing-trident-the-first-open-source-fuzzer-for-solana-programs/)  |  [![](https://abchprod.wpengine.com/wp-content/uploads/2024/05/Marinade.png)](https://solana.com/news/riptide-hackathon-winners-solana)

## Contribution

Thank you for your interest in contributing to Trident! Please see the [CONTRIBUTING.md](https://github.com/Ackee-Blockchain/trident/blob/master/CONTRIBUTING.md) to learn how.

## License

This project is licensed under the [MIT license](https://github.com/Ackee-Blockchain/trident/blob/master/LICENSE).

