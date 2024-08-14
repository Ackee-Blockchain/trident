use arbitrary_limit_inputs_5::entry as entry_arbitrary_limit_inputs_5;
use arbitrary_limit_inputs_5::ID as PROGRAM_ID_ARBITRARY_LIMIT_INPUTS_5;
const PROGRAM_NAME_ARBITRARY_LIMIT_INPUTS_5: &str = "arbitrary_limit_inputs_5";
use fuzz_instructions::arbitrary_limit_inputs_5_fuzz_instructions::FuzzInstruction as FuzzInstruction_arbitrary_limit_inputs_5;
use fuzz_instructions::arbitrary_limit_inputs_5_fuzz_instructions::InitVesting;
use trident_client::fuzzing::*;
mod fuzz_instructions;

struct MyFuzzData;

pub type FuzzInstruction = FuzzInstruction_arbitrary_limit_inputs_5;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::InitVesting(InitVesting::arbitrary(u)?);

        Ok(vec![init_ix])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_ARBITRARY_LIMIT_INPUTS_5,
                &PROGRAM_ID_ARBITRARY_LIMIT_INPUTS_5,
                processor!(convert_entry!(entry_arbitrary_limit_inputs_5))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_ARBITRARY_LIMIT_INPUTS_5, &mut client);
        });
    }
}
