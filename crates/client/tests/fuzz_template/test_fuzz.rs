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
struct FuzzTest<'a> {
    config: TridentConfig,
    client: TridentSVM<'a>,
}
#[flow_executor]
impl<'a> FuzzTest {
    #[init]
    fn start(&mut self) {}
}
fn main() {
    let program_additional_program = ProgramEntrypoint::new(
        pubkey!("8bPSKGoWCdAW8Hu3S1hLHPpBv8BNwse4jDyaXNrj3jWB"),
        None,
        processor!(entry_additional_program),
    );
    let program_idl_test = ProgramEntrypoint::new(
        pubkey!("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN"),
        None,
        processor!(entry_idl_test),
    );
    let config = TridentConfig::new();
    let client = TridentSVM::new_client(&[program_additional_program, program_idl_test], &config);
    let mut fuzz_test = FuzzTest::new(client, config);
    fuzz_test.fuzz();
}
