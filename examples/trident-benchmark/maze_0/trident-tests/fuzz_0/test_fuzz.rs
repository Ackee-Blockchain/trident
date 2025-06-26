use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use maze0::entry as entry_maze0;
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
        let mut client = TridentSVM::new_client(&TridentConfig::new());

        client.deploy_entrypoint(TridentEntrypoint::new(
            pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq"),
            None,
            processor!(entry_maze0),
        ));

        Self {
            client,
            metrics: FuzzingStatistics::default(),
            rng: TridentRng::random(),
        }
    }
    #[init]
    fn start(&mut self, accounts: &mut FuzzAccounts) -> Result<(), FuzzingError> {
        InitializeTransaction::build(&mut self.client, accounts, &mut self.rng).execute(
            &mut self.client,
            &mut self.metrics,
            &self.rng,
        )
    }

    #[flow]
    fn flow1(&mut self, accounts: &mut FuzzAccounts) -> Result<(), FuzzingError> {
        MoveEastTransaction::build(&mut self.client, accounts, &mut self.rng).execute(
            &mut self.client,
            &mut self.metrics,
            &self.rng,
        )
    }
    #[flow]
    fn flow2(&mut self, accounts: &mut FuzzAccounts) -> Result<(), FuzzingError> {
        MoveSouthTransaction::build(&mut self.client, accounts, &mut self.rng).execute(
            &mut self.client,
            &mut self.metrics,
            &self.rng,
        )
    }
    #[flow]
    fn flow3(&mut self, accounts: &mut FuzzAccounts) -> Result<(), FuzzingError> {
        MoveNorthTransaction::build(&mut self.client, accounts, &mut self.rng).execute(
            &mut self.client,
            &mut self.metrics,
            &self.rng,
        )
    }
    #[flow]
    fn flow4(&mut self, accounts: &mut FuzzAccounts) -> Result<(), FuzzingError> {
        MoveWestTransaction::build(&mut self.client, accounts, &mut self.rng).execute(
            &mut self.client,
            &mut self.metrics,
            &self.rng,
        )
    }
}
fn main() {
    FuzzTest::fuzz_parallel(10000, 1000);
}
