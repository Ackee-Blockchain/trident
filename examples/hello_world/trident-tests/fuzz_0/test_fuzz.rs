use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use hello_world::entry as entry_hello_world;
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
            pubkey!("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit"),
            None,
            processor!(entry_hello_world),
        ));
    }

    #[flow]
    fn flow1(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        InitializeFnTransaction::build(fuzzer_data, &mut self.client, accounts)?.execute(
            &mut self.client,
            &self.config,
            accounts,
        )?;

        Ok(())
    }
}
fn main() {
    let config = TridentConfig::new();
    let client = TridentSVM::new_client(&[], &config);
    FuzzTest::new(client, config).fuzz();
}
