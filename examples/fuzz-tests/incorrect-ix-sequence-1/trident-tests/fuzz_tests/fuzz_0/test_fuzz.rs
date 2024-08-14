use fuzz_instructions::incorrect_ix_sequence_1_fuzz_instructions::Initialize;
use incorrect_ix_sequence_1::entry as entry_incorrect_ix_sequence_1;
use incorrect_ix_sequence_1::ID as PROGRAM_ID_INCORRECT_IX_SEQUENCE_1;
const PROGRAM_NAME_INCORRECT_IX_SEQUENCE_1: &str = "incorrect_ix_sequence_1";
use fuzz_instructions::incorrect_ix_sequence_1_fuzz_instructions::FuzzInstruction as FuzzInstruction_incorrect_ix_sequence_1;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_incorrect_ix_sequence_1;

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

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_INCORRECT_IX_SEQUENCE_1,
                &PROGRAM_ID_INCORRECT_IX_SEQUENCE_1,
                processor!(convert_entry!(entry_incorrect_ix_sequence_1))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_INCORRECT_IX_SEQUENCE_1, &mut client);
        });
    }
}
