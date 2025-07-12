use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;

use additional_program::entry as entry_additional_program;

use idl_test::entry as entry_idl_test;

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
            pubkey!("8bPSKGoWCdAW8Hu3S1hLHPpBv8BNwse4jDyaXNrj3jWB"),
            None,
            processor!(entry_additional_program),
        ));

        client.deploy_entrypoint(TridentEntrypoint::new(
            pubkey!("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN"),
            None,
            processor!(entry_idl_test),
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
        // perform any initialization here, this method will be executed
        // at start of each iteration
        Ok(())
    }

    #[flow]
    fn flow1(&mut self) -> Result<(), FuzzingError> {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows
        Ok(())
    }

    #[flow]
    fn flow2(&mut self) -> Result<(), FuzzingError> {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows
        Ok(())
    }

    #[end]
    fn end(&mut self) -> Result<(), FuzzingError> {
        // perform any cleaning here, this method will be executed
        // at the end of each iteration
        Ok(())
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
