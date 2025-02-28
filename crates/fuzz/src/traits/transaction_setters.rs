use super::TransactionCustomMethods;
use crate::traits::FuzzClient;

#[doc(hidden)]
pub trait TransactionSetters: TransactionCustomMethods {
    /// Set accounts before transaction
    fn set_snapshot_before(&mut self, client: &mut impl FuzzClient);

    /// Set accounts after transaction
    fn set_snapshot_after(&mut self, client: &mut impl FuzzClient);

    /// Set data for the transaction
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts);

    /// Set accounts for the transaction
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts);
}
