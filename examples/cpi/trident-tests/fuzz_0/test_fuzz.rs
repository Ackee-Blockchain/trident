use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use callee::entry as entry_callee;
use cpi::entry as entry_cpi;
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
            pubkey!("CWjKHxkHU7kqRKqNutPAbxogKg3K1crH61gwwzsHjpC4"),
            None,
            processor!(entry_callee),
        ));
        client.deploy_entrypoint(TridentEntrypoint::new(
            pubkey!("77skervubsozZaRdojomG7FK8T2QQppxtSqG8ag9D4qV"),
            None,
            processor!(entry_cpi),
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
        let mut tx = InitializeCallerTransaction::build(
            &mut self.client,
            &mut self.fuzz_accounts,
            &mut self.rng,
        );

        self.execute_transaction(&mut tx, Some("initialize_caller"))?;

        Ok(())
    }
}
fn main() {
    FuzzTest::fuzz(1000, 50);
}
