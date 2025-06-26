use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use callee::entry as entry_callee;
use cpi::entry as entry_cpi;
pub use transactions::*;

struct FuzzTest {
    /// for transaction executions
    client: TridentSVM,
    /// for storing fuzzing metrics
    metrics: FuzzingStatistics,
    /// for storing seed
    rng: TridentRng,
}
/// Use flows to specify custom sequences of behavior
/// #[init]
/// fn start(&mut self) {
///     // Initialization goes here
/// }
/// #[flow]
/// fn flow1(
///     &mut self,
///     fuzzer_data: &mut FuzzerData,
///     accounts: &mut FuzzAccounts,
/// ) -> Result<(), FuzzingError> {
///     // Flow logic goes here
///     Ok(())
/// }
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
        }
    }
    #[init]
    fn start(&mut self, accounts: &mut FuzzAccounts) -> Result<(), FuzzingError> {
        InitializeCallerTransaction::build(&mut self.client, accounts, &mut self.rng).execute(
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
