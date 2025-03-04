use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use additional_program::entry as entry_additional_program;
use idl_test::entry as entry_idl_test;
pub use transactions::*;
#[derive(Default)]
struct FuzzTest<C> {
    client: C,
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
impl<C: FuzzClient + std::panic::RefUnwindSafe> FuzzTest<C> {
    fn new(client: C) -> Self {
        Self { client }
    }
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
    let client = TridentSVM::new_client(&[], &TridentConfig::new());
    FuzzTest::new(client).fuzz();
}
