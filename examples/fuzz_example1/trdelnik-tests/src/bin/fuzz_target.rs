use fuzz_example1::entry;
use program_client::fuzz_example1_instruction::*;
use trdelnik_client::{fuzz_trd, fuzzing::*};
use trdelnik_tests::fuzz_instructions::fuzz_example1_fuzz_instructions::{
    FuzzInstruction, Initialize,
};

const PROGRAM_NAME: &str = "fuzz_example1";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

// impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
//     fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
//         let init = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);

//         Ok(vec![init])
//     }
// }

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
