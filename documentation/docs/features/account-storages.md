# Account Storages


Trident allows developers to generate random accounts for fuzzing.

However, the Accounts are not completely random, and neither are the Account addresses.

Instead, Trident generates random **AccountIDs** which are indexes to **Account Storages**. Each unique Account contained within the Anchor generated IDL has its own AccountStorage. The FuzzAccounts containing the Accounts Storages is global to all Instructions to use.


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
    // ...
}
```

!!! tip

    Keep in mind:

    - You do not need to specify every `AccountStorage`, some accounts do not necessarily need to be stored in their corresponding storage.
        - For example `System Program` does not need to be stored, rather can be used from the `solana_sdk`.
    - If you are about to Initialize `Mint` or `Token Account` in your Solana Program.
        - use `Keypair` or `PdaStore` (not `MintStore` or `TokenStore`).
    - If you are going to initialize `Associated Token Account` in your Solana Program.
        - use `PdaStore`.
    - You can rename fields of `FuzzAccounts` to whatever you want. The default names are generated based on the Program's `IDL`.


!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
