use trident_client::fuzzing::*;

mod fuzz_instructions;

use callee::entry as entry_callee;
use callee::ID as PROGRAM_ID_CALLEE;
use caller::entry as entry_caller;
use caller::ID as PROGRAM_ID_CALLER;

const PROGRAM_NAME_CALLEE: &str = "callee";

const PROGRAM_NAME_CALLER: &str = "caller";

use fuzz_instructions::caller_fuzz_instructions::FuzzInstruction as fuzz_instruction_caller;

pub type FuzzInstruction = fuzz_instruction_caller;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(fuzz_data: FuzzData<T, U>) {
    let fuzzing_program_callee = FuzzingProgram::new(
        PROGRAM_NAME_CALLEE,
        &PROGRAM_ID_CALLEE,
        processor!(convert_entry!(entry_callee)),
    );

    let fuzzing_program_caller = FuzzingProgram::new(
        PROGRAM_NAME_CALLER,
        &PROGRAM_ID_CALLER,
        processor!(convert_entry!(entry_caller)),
    );

    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_callee, fuzzing_program_caller], &[])
            .unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client);
}

fn main() {
    loop {
        fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data) ; });
    }
}
