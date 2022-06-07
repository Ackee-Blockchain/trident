<div align="center">
  <img height="250" width="250" src="./assets/Badge_Trdelnik.png"/>

# Trdelník

<a href="https://discord.gg/x7qXXnGCsa">
  <img src="https://discordapp.com/api/guilds/867746290678104064/widget.png?style=banner2" width="250" title="AckeeBlockchain/Trdelnik discord">
</a>

developed by [Ackee Blockchain](https://ackeeblockchain.com)

[![Crates.io](https://img.shields.io/crates/v/trdelnik-cli?label=CLI)](https://crates.io/crates/trdelnik-cli)
[![Crates.io](https://img.shields.io/crates/v/trdelnik-test?label=Test)](https://crates.io/crates/trdelnik-test)
[![Crates.io](https://img.shields.io/crates/v/trdelnik-client?label=Client)](https://crates.io/crates/trdelnik-client)
[![Crates.io](https://img.shields.io/crates/v/trdelnik-explorer?label=Explorer)](https://crates.io/crates/trdelnik-explorer)
<br />
[![lint](https://github.com/Ackee-Blockchain/trdelnik/actions/workflows/lint.yml/badge.svg)](https://github.com/Ackee-Blockchain/trdelnik/actions/workflows/lint.yml)
[![test](https://github.com/Ackee-Blockchain/trdelnik/actions/workflows/test.yml/badge.svg)](https://github.com/Ackee-Blockchain/trdelnik/actions/workflows/test.yml)

</div>

Trdelník is Rust based testing framework providing several convenient developer tools for testing Solana programs written in [Anchor](https://github.com/project-serum/anchor)

- **Trdelnik client** - build and deploy an Anchor program to a local cluster and run a test suite against it
- **Trdelnik console** - built-in console to give developers a command prompt for quick program interaction
- **Trdelnik fuzz** - property-based and stateful testing
- **Trdelnik explorer** - exploring a ledger changes

<div align="center">
  <img src="./assets/demo.svg" />
</div>

## Dependencies

- Install [Rust](https://www.rust-lang.org/tools/install) (`nightly` release)
- Install [Solana tool suite](https://docs.solana.com/cli/install-solana-cli-tools) (`stable` release)
- Install [Anchor](https://book.anchor-lang.com/chapter_2/installation.html)

## Installation

```shell
cargo install trdelnik-cli 

# or the specific version

cargo install --version <version> trdelnik-cli
```

## Usage

```shell
# navigate to your project root directory
trdelnik init
# it will generate `program_client` and `trdelnik-tests` directories with all the necessary files
trdelnik test
# want more?
trdelnik --help
```

### How to write tests?

```rust
// <my_project>/trdelnik-tests/tests/test.rs
// @todo: do not forget to add all necessary dependencies to the generated `trdelnik-tests/Cargo.toml`
use program_client::my_instruction;
use trdelnik_client::*;
use my_program;

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
  // create a test fixture
  let mut fixture = Fixture {
    client: Client::new(system_keypair(0)),
    program: program_keypair(1),
    state: keypair(42),
  };
  // deploy a tested program
  fixture.deploy().await?;
  // call instruction init
  my_instruction::initialize(
    &fixture.client,
    fixture.state.pubkey(),
    fixture.client.payer().pubkey(),
    System::id(),
    Some(fixture.state.clone()),
  ).await?;
  fixture
}

#[trdelnik_test]
async fn test_happy_path(#[future] init_fixture: Result<Fixture>) {
  let fixture = init_fixture.await?;
  // call the instruction
  my_instruction::do_something(
    &fixture.client,
    "dummy_string".to_owned(),
    fixture.state.pubkey(),
    None,
  ).await?;
  // check the test result
  let state = fixture.get_state().await?;
  assert_eq!(state.something_changed, "yes");
}
```

#### Skipping tests

- You can add the `#[ignore]` macro to skip the test

```rust
#[trdelnik_test]
#[ignore]
async fn test() {}
```

- The `trdelnik init` command generated a dummy test suite for you
- For more details, see the [complete test](examples/turnstile/programs/tests/) implementation.

### Supported versions

- We support `Anchor` and `Solana` versions specified in table bellow

| Trdelnik CLI |  Anchor  |  Solana |
|--------------|:--------:|--------:|
| `latest`     | `>=0.24` | `>=1.9` |

- _Please make sure you are using the correct versions_

## Roadmap

- [x] Q1/22 Trdelnik announcement at Prague Hacker House
  - [x] Trdelnik client available for testing
- [ ] Q2/22 Trdelnik explorer available
- [ ] Q3/22 Trdelnik fuzz available
- [ ] Q3/22 Trdelnik console available

## Contribution

Thank you for your interest in contributing to Trdelník! Please see the [CONTRIBUTING.md](./CONTRIBUTING.md) to learn how

## License

This project is licensed under the [MIT license](https://github.com/Ackee-Blockchain/trdelnik/blob/master/LICENSE)
