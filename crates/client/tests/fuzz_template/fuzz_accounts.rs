use trident_fuzz::fuzzing::*;

/// Storage for all account addresses used in fuzz testing.
///
/// This struct serves as a centralized repository for account addresses,
/// enabling their reuse across different instruction flows and test scenarios.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct AccountAddresses {
    pub composite_account_nested: AddressStorage,

    pub some_account: AddressStorage,

    pub nested_inner: AddressStorage,

    pub system_program: AddressStorage,

    pub signer: AddressStorage,

    pub data_account_1: AddressStorage,

    pub data_account_2: AddressStorage,

    pub data_account_3: AddressStorage,

    pub data_account_4: AddressStorage,

    pub data_account_5: AddressStorage,

    pub data_account_6: AddressStorage,

    pub composite_account: AddressStorage,
}
