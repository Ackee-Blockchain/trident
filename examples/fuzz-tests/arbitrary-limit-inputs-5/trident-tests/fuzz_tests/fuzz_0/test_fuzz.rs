use arbitrary_limit_inputs_5::entry;
use arbitrary_limit_inputs_5::ID as PROGRAM_ID;
use fuzz_instructions::arbitrary_limit_inputs_5_fuzz_instructions::FuzzInstruction;
use fuzz_instructions::arbitrary_limit_inputs_5_fuzz_instructions::InitVesting;
use trident_client::fuzzing::*;
mod fuzz_instructions;

const PROGRAM_NAME: &str = "arbitrary_limit_inputs_5";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::InitVesting(InitVesting::arbitrary(u)?);

        Ok(vec![init_ix])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {
            let mut client =
                ProgramTestClientBlocking::new(PROGRAM_NAME, PROGRAM_ID, processor!(convert_entry!(entry)))
                    .unwrap();
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID, &mut client);
        });
    }
}
