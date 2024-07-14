use incorrect_integer_arithmetic_3::entry as entry_incorrect_integer_arithmetic_3;
use incorrect_integer_arithmetic_3::ID as PROGRAM_ID_INCORRECT_INTEGER_ARITHMETIC_3;
const PROGRAM_NAME_INCORRECT_INTEGER_ARITHMETIC_3: &str = "incorrect_integer_arithmetic_3";
use fuzz_instructions::incorrect_integer_arithmetic_3_fuzz_instructions::FuzzInstruction as FuzzInstruction_incorrect_integer_arithmetic_3;
use trident_client::fuzzing::*;
mod fuzz_instructions;

// TODO: In case of using file extension for AccountsSnapshots
// uncomment the line below
mod accounts_snapshots;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction_incorrect_integer_arithmetic_3> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction_incorrect_integer_arithmetic_3, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_INCORRECT_INTEGER_ARITHMETIC_3,
                &PROGRAM_ID_INCORRECT_INTEGER_ARITHMETIC_3,
                processor!(convert_entry!(entry_incorrect_integer_arithmetic_3))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_INCORRECT_INTEGER_ARITHMETIC_3, &mut client);
        });
    }
}
