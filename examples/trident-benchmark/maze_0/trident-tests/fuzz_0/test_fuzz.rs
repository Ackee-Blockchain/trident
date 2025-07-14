use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
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
        let client = TridentSVM::new_client(&TridentConfig::new());

        Self {
            client,
            metrics: FuzzingStatistics::default(),
            rng: TridentRng::random(),
            fuzz_accounts: FuzzAccounts::default(),
        }
    }

    #[init]
    fn start(&mut self) -> Result<(), FuzzingError> {
        let mut tx =
            InitializeTransaction::build(&mut self.client, &mut self.fuzz_accounts, &mut self.rng);

        let _res = self.execute_transaction(&mut tx, Some("Initialize"));
        Ok(())
    }

    #[flow]
    fn flow1(&mut self) -> Result<(), FuzzingError> {
        let mut tx =
            MoveEastTransaction::build(&mut self.client, &mut self.fuzz_accounts, &mut self.rng);
        let _res = self.execute_transaction(&mut tx, Some("MoveEast"));
        Ok(())
    }
    #[flow]
    fn flow2(&mut self) -> Result<(), FuzzingError> {
        let mut tx =
            MoveSouthTransaction::build(&mut self.client, &mut self.fuzz_accounts, &mut self.rng);
        let _res = self.execute_transaction(&mut tx, Some("MoveSouth"));
        Ok(())
    }
    #[flow]
    fn flow3(&mut self) -> Result<(), FuzzingError> {
        let mut tx =
            MoveNorthTransaction::build(&mut self.client, &mut self.fuzz_accounts, &mut self.rng);
        let _res = self.execute_transaction(&mut tx, Some("MoveNorth"));
        Ok(())
    }
    #[flow]
    fn flow4(&mut self) -> Result<(), FuzzingError> {
        let mut tx =
            MoveWestTransaction::build(&mut self.client, &mut self.fuzz_accounts, &mut self.rng);
        let _res = self.execute_transaction(&mut tx, Some("MoveWest"));
        Ok(())
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
