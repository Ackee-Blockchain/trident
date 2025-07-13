use trident_svm::trident_svm::TridentSVM;

use crate::fuzzing::{FuzzingStatistics, TridentRng};

pub trait FuzzTestGetters {
    #[doc(hidden)]
    /// Get Instruction discriminator
    fn get_client(&mut self) -> &mut TridentSVM;

    #[doc(hidden)]
    /// Get Instruction discriminator
    fn get_metrics(&mut self) -> &mut FuzzingStatistics;

    #[doc(hidden)]
    /// Get Instruction discriminator
    fn get_rng(&self) -> &TridentRng;
}
