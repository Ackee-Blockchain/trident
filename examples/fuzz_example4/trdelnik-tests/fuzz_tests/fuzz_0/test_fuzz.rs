use fuzz_example4::entry;
use fuzz_example4::ID as PROGRAM_ID;
use fuzz_instructions::fuzz_example4_fuzz_instructions::FuzzInstruction;
use fuzz_instructions::fuzz_example4_fuzz_instructions::Initialize;
use fuzz_instructions::fuzz_example4_fuzz_instructions::Update;
use trdelnik_client::{fuzz_trd, fuzzing::*};
mod accounts_snapshots;
mod fuzz_instructions;

const PROGRAM_NAME: &str = "fuzz_example4";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        let update = FuzzInstruction::Update(Update::arbitrary(u)?);
        Ok(vec![init, update])
    }
}

fn main() {
    loop {
        fuzz_trd!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {
            let mut client =
                ProgramTestClientBlocking::new(PROGRAM_NAME, PROGRAM_ID, processor!(entry))
                    .unwrap();
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID, &mut client);
        });
    }
}
