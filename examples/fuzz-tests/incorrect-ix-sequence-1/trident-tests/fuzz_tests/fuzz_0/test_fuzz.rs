use fuzz_instructions::incorrect_ix_sequence_1_fuzz_instructions::FuzzInstruction;
use fuzz_instructions::incorrect_ix_sequence_1_fuzz_instructions::Initialize;
use incorrect_ix_sequence_1::entry;
use incorrect_ix_sequence_1::ID as PROGRAM_ID;
use trident_client::{fuzz_trident, fuzzing::*};
mod accounts_snapshots;
mod fuzz_instructions;

const PROGRAM_NAME: &str = "incorrect_ix_sequence_1";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        Ok(vec![init_ix])
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
