# Specify accounts to reuse
{{ config.site_name }} fuzzer helps you to generate only a limited amount of pseudo-random accounts and reuse them in the instructions.

Always generating only random accounts would **in most cases lead to a situation where the fuzzer would be stuck because the accounts would be almost every time rejected by your Anchor program**. Therefore it is necessary to specify, what accounts should be used and also limit the number of newly created accounts to reduce the space complexity.

- Go to the `trident-tests/fuzz_tests/<FUZZ_TEST_NAME>/fuzz_instructions.rs` file and complete the pre-generated `FuzzAccounts` structure. It contains all accounts used in your program. You have to determine if the account is a:
    - Signer
    - PDA
    - Token Account
    - Program account

Then use the corresponding `AccountsStorage` types such as:

```rust
pub struct FuzzAccounts {
    signer: AccountsStorage<Keypair>,
    some_pda: AccountsStorage<PdaStore>,
    token_vault: AccountsStorage<TokenStore>,
    mint: AccountsStorage<MintStore>,
}
```
