use fuzz_instructions::Initialize;
use fuzz_instructions::Update;
use trident_client::fuzzing::*;
mod fuzz_instructions;

use fuzz_instructions::FuzzInstruction;
use unchecked_arithmetic_0::entry as entry_unchecked_arithmetic_0;
use unchecked_arithmetic_0::ID as PROGRAM_ID_UNCHECKED_ARITHMETIC_0;

const PROGRAM_NAME_UNCHECKED_ARITHMETIC_0: &str = "unchecked_arithmetic_0";
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
    pre_sequence!(Initialize);
    middle_sequence!(Update);
}

fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_unchecked_arithmetic_0 = FuzzingProgram::new(
        PROGRAM_NAME_UNCHECKED_ARITHMETIC_0,
        &PROGRAM_ID_UNCHECKED_ARITHMETIC_0,
        processor!(convert_entry!(entry_unchecked_arithmetic_0)),
    );

    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_unchecked_arithmetic_0], config).unwrap();

    let _ = fuzz_data.run_with_runtime(&mut client, config);
}

fn main() {
    let config = Config::new();

    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence | { fuzz_iteration (fuzz_data , & config) ; });
}
