use trident_fuzz::fuzzing::*;

/// Storage for all account addresses used in fuzz testing.
///
/// This struct serves as a centralized repository for account addresses,
/// enabling their reuse across different instruction flows and test scenarios.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct AccountAddresses {
    pub author2022: AddressStorage,
    pub mint2022: AddressStorage,
    pub group_member_mint2022: AddressStorage,
    pub token_account2022: AddressStorage,

    pub author: AddressStorage,
    pub token_account: AddressStorage,
    pub mint: AddressStorage,

    // Just for paying fees and rent
    pub payer: AddressStorage,
}
