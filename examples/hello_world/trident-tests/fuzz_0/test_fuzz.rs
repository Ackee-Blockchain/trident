use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// for transaction executions
    trident: Trident,
    /// for storing fuzzing accounts
    fuzz_accounts: FuzzAccounts,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: FuzzAccounts::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        let mut ix = InitializeFnTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident.execute_transaction(&mut ix, Some("Init"));
    }

    #[flow(weight = 5)]
    fn flow1(&mut self) {
        // This flow will be executed 60% of the time
        let mut ix = InitializeFnTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut ix, Some("Flow1"));
    }

    #[flow(weight = 5)]
    fn flow2(&mut self) {
        // This flow will be executed 40% of the time
        let mut ix = InitializeFnTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut ix, Some("Flow2"));
    }
    #[flow(weight = 90)]
    fn flow3(&mut self) {
        // This flow will be executed 40% of the time
        let mut ix = InitializeFnTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut ix, Some("Flow3"));
    }

    #[end]
    fn cleanup(&mut self) -> Result<(), FuzzingError> {
        // This method will be called after all flows have been executed
        Ok(())
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
