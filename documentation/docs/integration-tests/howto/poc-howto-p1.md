# Testing programs with associated token accounts

- `Trident` does not export `anchor-spl` and `spl-associated-token-account`, so you have to add it manually.

```toml
# <my-project>/trident-tests/poc_tests/Cargo.toml
# import the correct versions manually
anchor-spl = "0.29.0"
spl-associated-token-account = "2.0.0"
```

```rust
// <my-project>/trident-tests/poc_tests/tests/test.rs
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
