# Getting started

Here we provide a detailed guide on how to use SPL-Token 2022 in your tests.


## Account creation:

Trident provides a user-friendly interface/alias for loading or creating either Mint or Token Accounts.

### Creating/Loading Token-2022-Mint:

`.get_or_create_token_2022_mint` is the method used for either creating or loading your Mint accounts.

## Example

The following example demonstrates:
- Example syntax used to get-or-create a Token-2022 Mint account  

```rust
let token_2022_mint = fuzz_accounts.mint
    .get_or_create_token_2022_mint(
        account_id: self.accounts.mint.account_id,
        client: client,
        seeds: None,
        decimals: 9,
        mint_authority: &authority,
        freeze_authority: Some(authority),
        extensions: Some(extensions)
);
```
### Creating/Loading Token-2022-Account:

While Token-2022 Token Accounts uses the following method: `get_or_create_token2022_account`

```rust
let token_2022_account = fuzz_accounts.token_account 
    .get_or_create_token2022_account(  
        account_id: self.accounts.token_account.account_id,
        client: client,
        seeds: None,
        mint: self.accounts.mint.pubkey(),
        owner: receiver,  
        amount: 0,
        delegate: None,
        is_native: false,
        delegated_amount: 0,
        close_authority: None,
        extensions: Some(extensions),
    );
```

The alias method is mostly identical in usage to the regular methods used to interact/create regular SPL-Token Mint or Token Accounts, with the exception of an extra parameter: `extensions`.


