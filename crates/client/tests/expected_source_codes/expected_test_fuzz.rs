use trident_client::fuzzing::*;
mod fuzz_instructions;
use dummy_2::entry as entry_dummy_2;
use dummy_2::ID as PROGRAM_ID_dummy_2;
use dummy_example::entry as entry_dummy_example;
use dummy_example::ID as PROGRAM_ID_dummy_example;
use fuzz_instructions::FuzzInstruction;
const PROGRAM_NAME_dummy_2: &str = "dummy_2";
const PROGRAM_NAME_dummy_example: &str = "dummy_example";
struct InstructionsSequence;
/// Define instruction sequences for invocation.
/// `pre` runs at the start, `middle` in the middle, and `post` at the end.
/// For example, to call `InitializeFn`, `UpdateFn` and then `WithdrawFn` during
/// each fuzzing iteration:
/// ```
/// use fuzz_instructions::{InitializeFn, UpdateFn, WithdrawFn};
/// impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
///     pre_sequence!(InitializeFn,UpdateFn);
///     middle_sequence!(WithdrawFn);
///}
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/instructions-sequences/#instructions-sequences
impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {}
/// `fn fuzz_iteration` runs during every fuzzing iteration.
/// Modification is not required.
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_dummy_2 = FuzzingProgram::new(
        PROGRAM_NAME_dummy_2,
        &PROGRAM_ID_dummy_2,
        processor!(convert_entry!(entry_dummy_2)),
    );
    let fuzzing_program_dummy_example = FuzzingProgram::new(
        PROGRAM_NAME_dummy_example,
        &PROGRAM_ID_dummy_example,
        processor!(convert_entry!(entry_dummy_example)),
    );
    let mut client = ProgramTestClientBlocking::new(
        &[fuzzing_program_dummy_2, fuzzing_program_dummy_example],
        config,
    )
    .unwrap();
    let _ = fuzz_data.run_with_runtime(&mut client, config);
}
fn main() {
    let config = Config::new();
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence | { fuzz_iteration (fuzz_data , & config) ; });
}
