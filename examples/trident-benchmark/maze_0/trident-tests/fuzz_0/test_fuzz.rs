use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use maze0::entry as entry_maze0;
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
            pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq"),
            None,
            processor!(entry_maze0),
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
        let mut tx =
            InitializeTransaction::build(&mut self.client, &mut self.fuzz_accounts, &mut self.rng);

        self.execute_transaction(&mut tx, Some("Initialize"))
    }

    #[flow]
    fn flow1(&mut self) -> Result<(), FuzzingError> {
        let mut tx =
            MoveEastTransaction::build(&mut self.client, &mut self.fuzz_accounts, &mut self.rng);
        self.execute_transaction(&mut tx, Some("MoveEast"))
    }
    #[flow]
    fn flow2(&mut self) -> Result<(), FuzzingError> {
        let mut tx =
            MoveSouthTransaction::build(&mut self.client, &mut self.fuzz_accounts, &mut self.rng);
        self.execute_transaction(&mut tx, Some("MoveSouth"))
    }
    #[flow]
    fn flow3(&mut self) -> Result<(), FuzzingError> {
        let mut tx =
            MoveNorthTransaction::build(&mut self.client, &mut self.fuzz_accounts, &mut self.rng);
        self.execute_transaction(&mut tx, Some("MoveNorth"))
    }
    #[flow]
    fn flow4(&mut self) -> Result<(), FuzzingError> {
        let mut tx =
            MoveWestTransaction::build(&mut self.client, &mut self.fuzz_accounts, &mut self.rng);
        self.execute_transaction(&mut tx, Some("MoveWest"))
    }
}
fn main() {
    FuzzTest::fuzz(10000, 1000);
}
