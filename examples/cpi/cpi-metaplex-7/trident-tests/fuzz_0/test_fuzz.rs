use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use cpi_metaplex_7::entry as entry_cpi_metaplex_7;
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
            pubkey!("3XtULmXDGS867VbBXiPkjYr4EMjytGW8X12F6BS23Zcw"),
            None,
            processor!(entry_cpi_metaplex_7),
        ));
    }
}
fn main() {
    let client = TridentSVM::new_client(&[], &TridentConfig::new());
    FuzzTest::new(client).fuzz();
}
