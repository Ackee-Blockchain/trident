use solana_sdk::instruction::AccountMeta;

use super::FuzzClient;

pub trait AccountsMethods {
    type IxAccounts;

    #[allow(unused_variables)]
    fn resolve_accounts(
        &mut self,
        client: &mut impl FuzzClient,
        ix_accounts: &mut Self::IxAccounts,
    ) {
    }
    fn to_account_meta(&mut self) -> Vec<AccountMeta>;
    fn capture_before(&mut self, client: &mut impl FuzzClient);
    fn capture_after(&mut self, client: &mut impl FuzzClient);
}
