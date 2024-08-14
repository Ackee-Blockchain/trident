use fuzz_instructions::unauthorized_access_2_fuzz_instructions::Initialize;
use unauthorized_access_2::entry as entry_unauthorized_access_2;
use unauthorized_access_2::ID as PROGRAM_ID_UNAUTHORIZED_ACCESS_2;
const PROGRAM_NAME_UNAUTHORIZED_ACCESS_2: &str = "unauthorized_access_2";
use fuzz_instructions::unauthorized_access_2_fuzz_instructions::FuzzInstruction as FuzzInstruction_unauthorized_access_2;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_unauthorized_access_2;

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
                PROGRAM_NAME_UNAUTHORIZED_ACCESS_2,
                &PROGRAM_ID_UNAUTHORIZED_ACCESS_2,
                processor!(convert_entry!(entry_unauthorized_access_2))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_UNAUTHORIZED_ACCESS_2, &mut client);
        });
    }
}
