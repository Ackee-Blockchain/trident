use callee::entry as entry_callee;
use callee::ID as PROGRAM_ID_CALLEE;
use caller::ID as PROGRAM_ID_CALLER;
const PROGRAM_NAME_CALLEE: &str = "callee";
const PROGRAM_NAME_CALLER: &str = "caller";
use fuzz_instructions::caller_fuzz_instructions::FuzzInstruction as FuzzInstruction_caller;
use trident_client::fuzzing::*;
mod fuzz_instructions;

// TODO: In case of using file extension for AccountsSnapshots
// uncomment the line below
// mod accounts_snapshots;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction_caller> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction_caller, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_CALLER,
                &PROGRAM_ID_CALLER,
                None
            );

            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program2 = FuzzingProgram::new(
                PROGRAM_NAME_CALLEE,
                &PROGRAM_ID_CALLEE,
                processor!(convert_entry!(entry_callee))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1,fuzzing_program2])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_CALLER, &mut client);
        });
    }
}
