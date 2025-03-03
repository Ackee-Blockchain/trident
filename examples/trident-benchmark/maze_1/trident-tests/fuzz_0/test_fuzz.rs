use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use maze1::entry as entry_maze1;
pub use transactions::*;
#[derive(Default)]
struct FuzzTest<C> {
    client: C,
}
#[flow_executor]
impl<C: FuzzClient + std::panic::RefUnwindSafe> FuzzTest<C> {
    fn new(client: C) -> Self {
        Self { client }
    }
    #[init]
    fn start(&mut self) {
        self.client.deploy_native_program(ProgramEntrypoint::new(
            pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq"),
            None,
            processor!(entry_maze1),
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
    #[shuffle]
    #[flow]
    fn flow2(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        MoveEastTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        Ok(())
    }
    #[shuffle]
    #[flow]
    fn flow3(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        MoveNorthTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        Ok(())
    }
    #[shuffle]
    #[flow]
    fn flow4(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        MoveSouthTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        Ok(())
    }
    #[shuffle]
    #[flow]
    fn flow5(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        MoveWestTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        Ok(())
    }
}
fn main() {
    let client = TridentSVM::new_client(&[], &TridentConfig::new());
    FuzzTest::new(client).fuzz();
}
