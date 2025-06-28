use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// for transaction executions
    client: TridentSVM,
    /// for storing fuzzing metrics
    metrics: FuzzingStatistics,
    /// for storing seed
    rng: TridentRng,
    /// for storing fuzzing accounts
    fuzz_accounts: FuzzAccounts,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            client: TridentSVM::new_client(&TridentConfig::new()),
            metrics: FuzzingStatistics::default(),
            rng: TridentRng::random(),
            fuzz_accounts: FuzzAccounts::default(),
        }
    }
    #[init]
    fn start(&mut self) -> Result<(), FuzzingError> {
        let mut ix = InitializeFnTransaction::build(
            &mut self.client,
            &mut self.fuzz_accounts,
            &mut self.rng,
        );

        self.execute_transaction(&mut ix, None)?;
        Ok(())
    }

    #[flow]
    fn flow1(&mut self) -> Result<(), FuzzingError> {
        Ok(())
    }

    #[end]
    fn cleanup(&mut self) -> Result<(), FuzzingError> {
        // This method will be called after all flows have been executed
        println!("Cleaning up after fuzzing session");
        Ok(())
    }
}
fn main() {
    FuzzTest::fuzz_parallel(1000, 50);
}
