use solana_sdk::instruction::AccountMeta;

use super::FuzzClient;

pub trait RemainingAccountsMethods {
    #[doc(hidden)]
    fn to_account_meta(&mut self) -> Vec<AccountMeta>;

    #[doc(hidden)]
    fn capture_before(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    fn capture_after(&mut self, client: &mut impl FuzzClient);
}
