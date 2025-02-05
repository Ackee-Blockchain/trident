use solana_sdk::instruction::AccountMeta;

use super::FuzzClient;

pub trait AccountsMethods {
    fn to_account_meta(&mut self) -> Vec<AccountMeta>;
    fn capture_before(&mut self, client: &mut impl FuzzClient);
    fn capture_after(&mut self, client: &mut impl FuzzClient);
}
