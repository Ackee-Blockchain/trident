use arbitrary_custom_types_4::entry;
use arbitrary_custom_types_4::ID as PROGRAM_ID;
use fuzz_instructions::arbitrary_custom_types_4_fuzz_instructions::FuzzInstruction;
use fuzz_instructions::arbitrary_custom_types_4_fuzz_instructions::Initialize;
use fuzz_instructions::arbitrary_custom_types_4_fuzz_instructions::Update;
use trident_client::{fuzz_trident, fuzzing::*};
mod accounts_snapshots;
mod fuzz_instructions;

const PROGRAM_NAME: &str = "arbitrary_custom_types_4";

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
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {
            let mut client =
                ProgramTestClientBlocking::new(PROGRAM_NAME, PROGRAM_ID, processor!(entry))
                    .unwrap();
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID, &mut client);
        });
    }
}
