use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use metaplex::entry as entry_metaplex;
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
            pubkey!("H2XPhu8mmGDZioamVp2C5bDWXSSKn6bDdhpiUqWqPmLS"),
            None,
            processor!(entry_metaplex),
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

        self.execute_transaction(&mut tx, Some("initialize"))?;

        Ok(())
    }
}
fn main() {
    FuzzTest::fuzz(1000, 50);
}
