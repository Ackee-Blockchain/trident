use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

use maze0::entry as maze0;

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
        let mut trident = Trident::default();

        // Deploy through the entrypoint
        let program = TridentEntrypoint::new(maze0::ID, None, processor!(maze0));
        trident.get_client().deploy_entrypoint(program);

        Self {
            trident,
            fuzz_accounts: FuzzAccounts::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        let mut tx = InitializeTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident
            .execute_transaction(&mut tx, Some("Initialize"));
    }

    #[flow]
    fn flow1(&mut self) {
        let mut tx = MoveEastTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut tx, Some("MoveEast"));
    }
    #[flow]
    fn flow2(&mut self) {
        let mut tx = MoveSouthTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut tx, Some("MoveSouth"));
    }
    #[flow]
    fn flow3(&mut self) {
        let mut tx = MoveNorthTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut tx, Some("MoveNorth"));
    }
    #[flow]
    fn flow4(&mut self) {
        let mut tx = MoveWestTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut tx, Some("MoveWest"));
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
