use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use callee::entry as entry_callee;
use cpi::entry as entry_cpi;
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
        self.client.deploy_entrypoint(TridentEntrypoint::new(
            pubkey!("CWjKHxkHU7kqRKqNutPAbxogKg3K1crH61gwwzsHjpC4"),
            None,
            processor!(entry_callee),
        ));
        self.client.deploy_entrypoint(TridentEntrypoint::new(
            pubkey!("77skervubsozZaRdojomG7FK8T2QQppxtSqG8ag9D4qV"),
            None,
            processor!(entry_cpi),
        ));
    }
    #[flow]
    fn flow1(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        InitializeCallerTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        Ok(())
    }
}
fn main() {
    let client = TridentSVM::new_client(&TridentConfig::new());
    FuzzTest::new(client).fuzz();
}
