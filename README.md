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

Trdelník is Rust based testing framework providing several convenient developer tools for testing Solana programs written in [Anchor](https://github.com/project-serum/anchor).

- **Trdelnik client** - build and deploy an Anchor program to a local cluster and run a test suite against it;
- **Trdelnik console** - built-in console to give developers a command prompt for quick program interaction;
- **Trdelnik fuzz** - property-based and stateful testing;
- **Trdelnik explorer** - exploring a ledger changes.

<div align="center">
  <img src="./assets/demo.svg" />
</div>

## Dependencies

- Install [Rust](https://www.rust-lang.org/tools/install) (`nightly` release)
- Install [Solana tool suite](https://docs.solana.com/cli/install-solana-cli-tools) (`stable` release)
- Install [Anchor](https://book.anchor-lang.com/chapter_2/installation.html)

## Installation

**Currently Trdelnik is only available as a [beta release](https://crates.io/crates/trdelnik-sandbox-cli), we are working hard toward the first official release coming within a few days.**

```shell
cargo install trdelnik-cli

# or the specific version

cargo install --version <version> trdelnik-cli
```

## Usage

```shell
# navigate to your project root directory
trdelnik init
# it will generate `.program_client` and `trdelnik-tests` directories with all the necessary files
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

- You can add the `#[ignore]` macro to skip the test.

```rust
#[trdelnik_test]
#[ignore]
async fn test() {}
```

#### Testing programs with associated token accounts

- `Trdelnik` does not export `anchor-spl` and `spl-associated-token-account`, so you have to add it manually.

```toml
# <my-project>/trdelnik-tests/Cargo.toml
# import the correct versions manually
anchor-spl = "0.24.2"
spl-associated-token-account = "1.0.3"
```

```rust
// <my-project>/trdelnik-tests/tests/test.rs
use anchor_spl::token::Token;
use spl_associated_token_account;

async fn init_fixture() -> Fixture {
  // ...
  let account = keypair(1);
  let mint = keypair(2);
  // constructs a token mint
  client
    .create_token_mint(&mint, mint.pubkey(), None, 0)
    .await?;
  // constructs associated token account
  let token_account = client
    .create_associated_token_account(&account, mint.pubkey())
    .await?;
  let associated_token_program = spl_associated_token_account::id();
  // derives the associated token account address for the given wallet and mint
  let associated_token_address = spl_associated_token_account::get_associated_token_address(&account.pubkey(), mint);
  Fixture {
    // ...
    token_program: Token::id(),
  }
}
```

- The `trdelnik init` command generated a dummy test suite for you.
- For more details, see the [complete test](examples/turnstile/programs/tests/) implementation.

### Supported versions

- We support `Anchor` and `Solana` versions specified in the table bellow.

| Trdelnik CLI |  Anchor  |  Solana |
| ------------ | :------: | ------: |
| `latest`     | `>=0.24` | `>=1.9` |

- _Please make sure you are using the correct versions._

## Roadmap

- [x] Q1/22 Trdelnik announcement at Solana Hacker House Prague
  - [x] Trdelnik client available for testing
- [x] Q2/22 Trdelnik explorer available
- [x] Q2/22 Trdelnik console, client and explorer introduced at Solana Hacker House Barcelona
- [ ] Q3/22 Trdelnik fuzz available
- [ ] Q3/22 Trdelnik console available

## Awards

**Marinade Community Prize** - winner of the [Marinade grant](https://solana.blog/riptide-hackathon-winners/) for the 2022 Solana Riptide Hackathon.

## Contribution

Thank you for your interest in contributing to Trdelník! Please see the [CONTRIBUTING.md](./CONTRIBUTING.md) to learn how.

## License

This project is licensed under the [MIT license](https://github.com/Ackee-Blockchain/trdelnik/blob/master/LICENSE).

## University and investment partners 
- [Czech technical university in Prague](https://www.cvut.cz/en)
- [Ackee](https://www.ackee.cz/)
- [Rockaway Blockchain Fund](https://rbf.capital/)
