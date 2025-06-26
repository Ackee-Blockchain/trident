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
        let mut client = TridentSVM::new_client(&TridentConfig::new());

        client.deploy_entrypoint(TridentEntrypoint::new(
            pubkey!("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit"),
            None,
            processor!(entry_hello_world),
        ));
        Self {
            client,
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

        self.execute_transaction(&mut ix, Some("Init"))?;
        Ok(())
    }

    #[flow(weight = 5)]
    fn flow1(&mut self) -> Result<(), FuzzingError> {
        // This flow will be executed 60% of the time
        let mut ix = InitializeFnTransaction::build(
            &mut self.client,
            &mut self.fuzz_accounts,
            &mut self.rng,
        );
        self.execute_transaction(&mut ix, Some("Flow1"))?;
        Ok(())
    }

    #[flow(weight = 5)]
    fn flow2(&mut self) -> Result<(), FuzzingError> {
        // This flow will be executed 40% of the time
        let mut ix = InitializeFnTransaction::build(
            &mut self.client,
            &mut self.fuzz_accounts,
            &mut self.rng,
        );
        self.execute_transaction(&mut ix, Some("Flow2"))?;
        Ok(())
    }
    #[flow(weight = 90)]
    fn flow3(&mut self) -> Result<(), FuzzingError> {
        // This flow will be executed 40% of the time
        let mut ix = InitializeFnTransaction::build(
            &mut self.client,
            &mut self.fuzz_accounts,
            &mut self.rng,
        );
        self.execute_transaction(&mut ix, Some("Flow3"))?;
        Ok(())
    }

    #[end]
    fn cleanup(&mut self) -> Result<(), FuzzingError> {
        // This method will be called after all flows have been executed
        Ok(())
    }
}
fn main() {
    FuzzTest::fuzz(1000, 50);
}
