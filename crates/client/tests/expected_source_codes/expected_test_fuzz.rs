use trident_client::fuzzing::*;

mod fuzz_instructions;

use dummy_2::entry as entry_dummy_2;
use dummy_2::ID as PROGRAM_ID_DUMMY_2;
use dummy_example::entry as entry_dummy_example;
use dummy_example::ID as PROGRAM_ID_DUMMY_EXAMPLE;
use fuzz_instructions::FuzzInstruction;

const PROGRAM_NAME_DUMMY_2: &str = "dummy_2";

const PROGRAM_NAME_DUMMY_EXAMPLE: &str = "dummy_example";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {

    let fuzzing_program_dummy_2 = FuzzingProgram::new(
        PROGRAM_NAME_DUMMY_2,
        &PROGRAM_ID_DUMMY_2,
        processor!(convert_entry!(entry_dummy_2)),
    );

    let fuzzing_program_dummy_example = FuzzingProgram::new(
        PROGRAM_NAME_DUMMY_EXAMPLE,
        &PROGRAM_ID_DUMMY_EXAMPLE,
        processor!(convert_entry!(entry_dummy_example)),
    );

    let mut client = ProgramTestClientBlocking::new(
        &[fuzzing_program_dummy_2, fuzzing_program_dummy_example],
        &[],
    )
    .unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client, config);
}

fn main() {

    let config = Config::new();

    loop {

        fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data , & config) ; });
    }
}
