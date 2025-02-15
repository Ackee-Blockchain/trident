# Fuzz Accounts

`FuzzAccounts` is a struct that contains `all the accounts that can be used in the fuzz test` i.e. `storage` for accounts.

By default, Trident generates the struct based on the program's idl, i.e. for each account Trident generates a field in the struct.

On demand, you can add your own accounts to the struct, these accounts are meant to be used within the `set_accounts`, `set_remaining_accounts` and potentially within the `set_data` methods, with the corresponding account indexes.


```rust
pub struct FuzzAccounts {
    pub account1: AccountsStorage,
    pub account2: AccountsStorage,
    pub account3: AccountsStorage,
    pub account4: AccountsStorage,
    pub account5: AccountsStorage,
    pub account6: AccountsStorage,
    /// ....
}
```
