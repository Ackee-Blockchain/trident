use fuzz_instructions::hello_world_fuzz_instructions::Initialize;
use hello_world::entry as entry_hello_world;
use hello_world::ID as PROGRAM_ID_HELLO_WORLD;
const PROGRAM_NAME_HELLO_WORLD: &str = "hello_world";
use fuzz_instructions::hello_world_fuzz_instructions::FuzzInstruction as FuzzInstruction_hello_world;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_hello_world;

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

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_HELLO_WORLD,
                &PROGRAM_ID_HELLO_WORLD,
                processor!(convert_entry!(entry_hello_world))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_HELLO_WORLD, &mut client);
        });
    }
}
