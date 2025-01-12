# Account Storages

Trident allows developers to generate random accounts for fuzzing.

However, the fuzzer cannot generate account addresses completely randomly, as the address has 32 bytes; this means that the fuzzer will, in most cases, generate incorrect addresses.

Thus, Trident generates random **AccountIDs** which are indexes to **Account Storages**. Each unique account contained within the Anchor-generated IDL has its own AccountStorage. The FuzzAccounts containing the Account Storages are global to all instructions.

!!! important

    There are two types of Account Storages:

    - KeypairStore (dedicated for Keypairs)
    - PdaStore (dedicated for Program Derived Addresses)

```rust
pub struct FuzzAccounts {
    signer: AccountsStorage<KeypairStore>,
    some_pda: AccountsStorage<PdaStore>,
    // ...
}
```

!!! tip

    Keep in mind:

    - You do not need to specify every `AccountStorage`; some accounts do not necessarily need to be stored in their corresponding storage.
        - For example, the `System Program` does not need to be stored and can be used from the `solana_sdk` as a constant account address.
    - If you are going to initialize an `Associated Token Account` in your Solana Program, use `PdaStore`.
    - You can rename fields of `FuzzAccounts` to whatever you want. The default names are generated based on the Program's `IDL`.

## Account Storage Methods

There are multiple methods to interact with Account Storages.

### `get_or_create_account()`

Retrieves a record from AccountsStorage based on the entered `account_id`. If no record exists for the `account_id`, a new **empty** account is created.

### `get()`

Retrieves a record from AccountsStorage based on the entered `account_id`. If no record exists for the `account_id`, a random public key is returned.

### `get_or_create_token_account()`

Retrieves a record from AccountsStorage based on the entered `account_id`. If no record exists for the `account_id`, a new Token account is created.

### `get_or_create_mint_account()`

Retrieves a record from AccountsStorage based on the entered `account_id`. If no record exists for the `account_id`, a new Mint account is created.

### `get_or_create_delegated_account()`

Retrieves a record from AccountsStorage based on the entered `account_id`. If no record exists for the `account_id`, a new Delegated account is created.

### `get_or_create_initialized_account()`

Retrieves a record from AccountsStorage based on the entered `account_id`. If no record exists for the `account_id`, a new Initialized account is created.

### `get_or_create_vote_account()`

Retrieves a record from AccountsStorage based on the entered `account_id`. If no record exists for the `account_id`, a new Vote account is created.

!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
