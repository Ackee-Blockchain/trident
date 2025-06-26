use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

struct FuzzTest {
    /// for transaction executions
    client: TridentSVM,
    /// for storing fuzzing metrics
    metrics: FuzzingStatistics,
    /// for storing seed
    rng: TridentRng,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            client: TridentSVM::new_client(&TridentConfig::new()),
            metrics: FuzzingStatistics::default(),
            rng: TridentRng::random(),
        }
    }
    #[init]
    fn start(&mut self, accounts: &mut FuzzAccounts) -> Result<(), FuzzingError> {
        InitializeFnTransaction::build(&mut self.client, accounts, &mut self.rng).execute(
            &mut self.client,
            &mut self.metrics,
            &self.rng,
        )?;
        Ok(())
    }
}
fn main() {
    FuzzTest::fuzz_parallel(1000, 50);
}
