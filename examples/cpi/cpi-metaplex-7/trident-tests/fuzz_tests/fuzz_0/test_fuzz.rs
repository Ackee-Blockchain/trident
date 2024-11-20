use fuzz_instructions::Initialize;
use trident_client::fuzzing::*;
mod fuzz_instructions;
use cpi_metaplex_7::entry as entry_cpi_metaplex_7;
use cpi_metaplex_7::ID as PROGRAM_ID_CPI_METAPLEX_7;
use fuzz_instructions::FuzzInstruction;
const PROGRAM_NAME_CPI_METAPLEX_7: &str = "cpi_metaplex_7";
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
    middle_sequence!();
    post_sequence!();
}
/// `fn fuzz_iteration` runs during every fuzzing iteration.
/// Modification is not required.
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_cpi_metaplex_7 = FuzzingProgram::new(
        PROGRAM_NAME_CPI_METAPLEX_7,
        &PROGRAM_ID_CPI_METAPLEX_7,
        processor!(convert_entry!(entry_cpi_metaplex_7)),
    );
    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_cpi_metaplex_7], config).unwrap();
    let _ = fuzz_data.run_with_runtime(&mut client, config);
}
fn main() {
    let config = Config::new();
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence | { fuzz_iteration (fuzz_data , & config) ; });
}
