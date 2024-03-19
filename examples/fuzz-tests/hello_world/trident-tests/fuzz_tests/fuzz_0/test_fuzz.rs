use fuzz_instructions::hello_world_fuzz_instructions::FuzzInstruction;
use fuzz_instructions::hello_world_fuzz_instructions::Initialize;
use hello_world::entry;
use hello_world::ID as PROGRAM_ID;
use trident_client::{fuzz_trident, fuzzing::*};
mod accounts_snapshots;
mod fuzz_instructions;

const PROGRAM_NAME: &str = "hello_world";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        Ok(vec![init])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {
            let mut client =
                ProgramTestClientBlocking::new(PROGRAM_NAME, PROGRAM_ID, processor!(entry))
                    .unwrap();
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID, &mut client);
        });
    }
}
