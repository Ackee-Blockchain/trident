use fuzz_instructions::Initialize;
use trident_client::fuzzing::*;

mod fuzz_instructions;

use fuzz_instructions::FuzzInstruction;
use incorrect_ix_sequence_1::entry as entry_incorrect_ix_sequence_1;
use incorrect_ix_sequence_1::ID as PROGRAM_ID_INCORRECT_IX_SEQUENCE_1;

const PROGRAM_NAME_INCORRECT_IX_SEQUENCE_1: &str = "incorrect_ix_sequence_1";

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        Ok(vec![init_ix])
    }
}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(fuzz_data: FuzzData<T, U>) {
    let fuzzing_program_incorrect_ix_sequence_1 = FuzzingProgram::new(
        PROGRAM_NAME_INCORRECT_IX_SEQUENCE_1,
        &PROGRAM_ID_INCORRECT_IX_SEQUENCE_1,
        processor!(convert_entry!(entry_incorrect_ix_sequence_1)),
    );

    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_incorrect_ix_sequence_1], &[]).unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client);
}

fn main() {
    loop {
        fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data) ; });
    }
}
