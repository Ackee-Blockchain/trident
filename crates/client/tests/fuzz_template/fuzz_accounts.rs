use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub composite_account_nested: AccountsStorage,

    pub system_program: AccountsStorage,

    pub data_account_1: AccountsStorage,

    pub data_account_5: AccountsStorage,

    pub composite_account: AccountsStorage,

    pub nested_inner: AccountsStorage,

    pub data_account_3: AccountsStorage,

    pub some_account: AccountsStorage,

    pub signer: AccountsStorage,

    pub data_account_2: AccountsStorage,

    pub data_account_4: AccountsStorage,

    pub data_account_6: AccountsStorage,
}
