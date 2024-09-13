use trident_client::fuzzing::*;

mod fuzz_instructions;

use dummy_example::entry as entry_dummy_example;
use dummy_example::ID as PROGRAM_ID_DUMMY_EXAMPLE;

const PROGRAM_NAME_DUMMY_EXAMPLE: &str = "dummy_example";

use fuzz_instructions::dummy_example_fuzz_instructions::FuzzInstruction as fuzz_instruction_dummy_example;

pub type FuzzInstruction = fuzz_instruction_dummy_example;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(fuzz_data: FuzzData<T, U>) {

    let fuzzing_program_dummy_example = FuzzingProgram::new(
        PROGRAM_NAME_DUMMY_EXAMPLE,
        &PROGRAM_ID_DUMMY_EXAMPLE,
        processor!(convert_entry!(entry_dummy_example)),
    );

    let mut client = ProgramTestClientBlocking::new(&[fuzzing_program_dummy_example], &[]).unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client);
}

fn main() {

    loop {

        fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data) ; });
    }
}
