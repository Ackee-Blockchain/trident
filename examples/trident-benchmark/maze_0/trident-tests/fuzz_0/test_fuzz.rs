use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use maze0::entry as entry_maze0;
pub use transactions::*;
#[derive(Default)]
struct FuzzTest<C> {
    client: C,
}
#[flow_executor(random_tail = true)]
impl<C: FuzzClient + std::panic::RefUnwindSafe> FuzzTest<C> {
    fn new(client: C) -> Self {
        Self { client }
    }
    #[init]
    fn start(&mut self) {
        self.client.deploy_native_program(ProgramEntrypoint::new(
            pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq"),
            None,
            processor!(entry_maze0),
        ));
    }

    #[flow]
    fn flow1(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        InitializeTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        Ok(())
    }
}
fn main() {
    let client = TridentSVM::new_client(&[], &TridentConfig::new());
    FuzzTest::new(client).fuzz();
}
