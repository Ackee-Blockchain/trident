use fuzz_instructions::InitVesting;
use trident_client::fuzzing::*;
mod fuzz_instructions;
use fuzz_instructions::FuzzInstruction;
use incorrect_integer_arithmetic_3::entry as entry_incorrect_integer_arithmetic_3;
use incorrect_integer_arithmetic_3::ID as PROGRAM_ID_INCORRECT_INTEGER_ARITHMETIC_3;
const PROGRAM_NAME_INCORRECT_INTEGER_ARITHMETIC_3: &str = "incorrect_integer_arithmetic_3";
struct InstructionsSequence;
/// Define instruction sequences for invocation.
/// `pre` runs at the start, `middle` in the middle, and `post` at the end.
/// For example, to call `InitializeFn`, `UpdateFn` and then `WithdrawFn` during
/// each fuzzing iteration:
/// ```
/// impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
///     pre_sequence!(InitializeFn,UpdateFn);
///     middle_sequence!(WithdrawFn);
///}
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/instructions-sequences/#instructions-sequences
impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
    pre_sequence!(InitVesting);
}
/// `fn fuzz_iteration` runs during every fuzzing iteration.
/// Modification is not required.
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_incorrect_integer_arithmetic_3 = FuzzingProgram::new(
        PROGRAM_NAME_INCORRECT_INTEGER_ARITHMETIC_3,
        &PROGRAM_ID_INCORRECT_INTEGER_ARITHMETIC_3,
        processor!(convert_entry!(entry_incorrect_integer_arithmetic_3)),
    );
    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_incorrect_integer_arithmetic_3], config)
            .unwrap();
    let _ = fuzz_data.run_with_runtime(&mut client, config);
}
fn main() {
    let config = Config::new();
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence | { fuzz_iteration (fuzz_data , & config) ; });
}
