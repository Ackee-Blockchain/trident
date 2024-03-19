use fuzz_instructions::unchecked_arithmetic_0_fuzz_instructions::FuzzInstruction;
use fuzz_instructions::unchecked_arithmetic_0_fuzz_instructions::Initialize;
use trident_client::{fuzz_trident, fuzzing::*};
use unchecked_arithmetic_0::entry;
use unchecked_arithmetic_0::ID as PROGRAM_ID;
mod accounts_snapshots;
mod fuzz_instructions;

const PROGRAM_NAME: &str = "unchecked_arithmetic_0";

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
