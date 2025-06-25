use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use hello_world::entry as entry_hello_world;
pub use transactions::*;

use rand::{rngs::SmallRng, Rng, SeedableRng};

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
    fn start(&mut self) {
        self.client.deploy_entrypoint(TridentEntrypoint::new(
            pubkey!("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit"),
            None,
            processor!(entry_hello_world),
        ));
    }
    #[flow]
    fn flow1(&mut self, accounts: &mut FuzzAccounts) -> Result<(), FuzzingError> {
        InitializeFnTransaction::build(&mut self.client, accounts, &mut self.rng)
            .execute(&mut self.client, &mut self.metrics)
            .unwrap();

        Ok(())
    }
}
fn main() {
    FuzzTest::fuzz_parallel(10000);
}
