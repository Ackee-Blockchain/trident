use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use arbitrary_limit_inputs_5::entry as entry_arbitrary_limit_inputs_5;
pub use transactions::*;
#[derive(Default, FuzzTestExecutor)]
struct FuzzTest {
    client: TridentSVM,
}
#[flow_executor]
impl FuzzTest {
    #[init]
    fn start(&mut self) {
        self.client.deploy_native_program(ProgramEntrypoint::new(
            pubkey!("AGpdCBtXUyLWKutvMCVDeTywkxgvQVjJk54btLQNLMiZ"),
            None,
            processor!(entry_arbitrary_limit_inputs_5),
        ));
    }
    #[flow]
    fn flow1(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        InitVestingTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        Ok(())
    }

    #[flow]
    fn flow2(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        WithdrawUnlockedTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        Ok(())
    }
}
fn main() {
    let client = TridentSVM::new_client(&[], &TridentConfig::new());
    FuzzTest::new(client).fuzz();
}
