use hello_world_light::entry as entry_hello_world_light;
use hello_world_light::ID as PROGRAM_ID_HELLO_WORLD_LIGHT;
const PROGRAM_NAME_HELLO_WORLD_LIGHT: &str = "hello_world_light";
use fuzz_instructions::hello_world_light_fuzz_instructions::FuzzInstruction as FuzzInstruction_hello_world_light;
use trident_client::fuzzing::*;
mod fuzz_instructions;

// TODO: In case of using file extension for AccountsSnapshots
// uncomment the line below
// mod accounts_snapshots;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction_hello_world_light> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction_hello_world_light, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgramLight::new(
                PROGRAM_NAME_HELLO_WORLD_LIGHT,
                &PROGRAM_ID_HELLO_WORLD_LIGHT,
                Some(entry_hello_world_light)
            );

            let mut client =
                LightClient::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_HELLO_WORLD_LIGHT, &mut client);
        });
    }
}
