# Specify instruction accounts

{{ config.site_name }} fuzzer generates random indexes of accounts to use in each instruction. Each created account is saved in the global `FuzzAccounts` structure which helps you to reuse already existing accounts across all instructions.

You are required to define, how these accounts should be created and which accounts should be passed to an instruction. It is done using the `IxOps` trait and its method `get_accounts`.

- Go to the `trdelnik-tests/fuzz_tests/<FUZZ_TEST_NAME>/fuzz_instructions.rs` file and complete the pre-generated `get_accounts` methods for each instruction such as:

```rust
fn get_accounts(
    &self,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut FuzzAccounts,
) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
    let author = fuzz_accounts.author.get_or_create_account(
        self.accounts.author,
        client,
        5000000000000,
    );
    let signers = vec![author.clone()];
    let state = fuzz_accounts
        .state
        .get_or_create_account(
            self.accounts.state,
            &[author.pubkey().as_ref(), STATE_SEED.as_ref()],
            &fuzz_example1::ID,
        )
        .ok_or(FuzzingError::CannotGetAccounts)?
        .pubkey();
    let acc_meta = fuzz_example1::accounts::EndRegistration {
        author: author.pubkey(),
        state,
    }
    .to_account_metas(None);
    Ok((signers, acc_meta))
}
```
Notice especially the helper method `fuzz_accounts.<account_name>.get_or_create_account` that is used to create a Keypair or retrieve the Public key of the already existing account.
