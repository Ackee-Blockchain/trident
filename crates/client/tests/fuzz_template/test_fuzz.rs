use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use additional_program::entry as entry_additional_program;
use idl_test::entry as entry_idl_test;
pub use transactions::*;
#[derive(Default, FuzzTestExecutor)]
struct FuzzTest {
    config: TridentConfig,
    client: TridentSVM,
}
#[flow_executor]
impl FuzzTest {
    #[init]
    fn start(&mut self) {
        self.client.deploy_native_program(ProgramEntrypoint::new(
            pubkey!("8bPSKGoWCdAW8Hu3S1hLHPpBv8BNwse4jDyaXNrj3jWB"),
            None,
            processor!(entry_additional_program),
        ));
        self.client.deploy_native_program(ProgramEntrypoint::new(
            pubkey!("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN"),
            None,
            processor!(entry_idl_test),
        ));
    }
}
fn main() {
    let config = TridentConfig::new();
    let client = TridentSVM::new_client(&[], &config);
    FuzzTest::new(client, config).fuzz();
}
