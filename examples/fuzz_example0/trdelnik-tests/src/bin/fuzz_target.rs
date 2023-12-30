use fuzzer::entry;
use program_client::fuzzer_instruction::*;
use trdelnik_tests::fuzz_instructions::fuzzer_fuzz_instructions::FuzzInstruction;
use trdelnik_client::{fuzz_trd, fuzzing::*};

const PROGRAM_NAME: &str = "fuzzer";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

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