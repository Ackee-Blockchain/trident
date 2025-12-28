use trident_fuzz::fuzzing::*;

/// Storage for all account addresses used in fuzz testing.
///
/// This struct serves as a centralized repository for account addresses,
/// enabling their reuse across different instruction flows and test scenarios.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct AccountAddresses {
    pub compositeAccountNested: AddressStorage,

    pub someAccount: AddressStorage,

    pub nestedInner: AddressStorage,

    pub systemProgram: AddressStorage,

    pub deployer: AddressStorage,

    pub signer: AddressStorage,

    pub dataAccount1: AddressStorage,

    pub dataAccount2: AddressStorage,

    pub dataAccount3: AddressStorage,

    pub dataAccount4: AddressStorage,

    pub dataAccount5: AddressStorage,

    pub dataAccount6: AddressStorage,

    pub compositeAccount: AddressStorage,
}
