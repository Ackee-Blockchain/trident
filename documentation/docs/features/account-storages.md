# Account Storages


Trident allows developers to generate random accounts for fuzzing. However, the Accounts are not completely random, nor the Account addresses are not completely random. Instead, Trident generates random AccountIDs which are indexes to Account Storages. Each uniqe Account contained within the Anchor generated IDL has its own AccountStorage. The FuzzAccounts containing the Accounts Storages is global to all Instructions to use.


??? note

    **Details:**

    Always generating only random accounts would **in most cases lead to a situation where the fuzzer would be stuck because the accounts would be almost every time rejected by your Anchor program**. Therefore it is necessary to specify, what accounts should be used and also limit the number of newly created accounts to reduce the space complexity.

!!! important

    Currently, supported types of Account Storages:

    - Signer
    - PDA
    - Token Account
    - Program account

    Then use the corresponding AccountsStorage.

```rust
pub struct FuzzAccounts {
    signer: AccountsStorage<Keypair>,
    some_pda: AccountsStorage<PdaStore>,
    token_vault: AccountsStorage<TokenStore>,
    mint: AccountsStorage<MintStore>,
}
```

!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
