use super::TransactionCustomMethods;
use crate::traits::FuzzClient;

#[doc(hidden)]
pub trait TransactionSetters: TransactionCustomMethods {
    /// Set accounts before transaction
    fn set_snapshot_before(&mut self, client: &mut impl FuzzClient);

    /// Set accounts after transaction
    fn set_snapshot_after(&mut self, client: &mut impl FuzzClient);
}
