use trident_fuzz::fuzzing::*;
mod fuzz_instructions;
use fuzz_instructions::FuzzInstruction;
use fuzz_instructions::*;
use incorrect_ix_sequence_1::entry as entry_incorrect_ix_sequence_1;
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
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/instructions-sequences/#instructions-sequences
impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {
    pre_sequence!(Initialize);
}
/// `fn fuzz_iteration` runs during every fuzzing iteration.
/// Modification is not required.
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
    client: &mut impl FuzzClient,
) {
    let _ = fuzz_data.run_with_runtime(client, config);
}
fn main() {
    let program_incorrect_ix_sequence_1 = ProgramEntrypoint::new(
        pubkey!("dk5VmuCSjrG6iRVXRycKZ6mS4rDCyvBrYJvcfyqWGcU"),
        None,
        processor!(entry_incorrect_ix_sequence_1),
    );
    let config = Config::new();
    let mut client = TridentSVM::new_client(&[program_incorrect_ix_sequence_1], &config);
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence | { fuzz_iteration (fuzz_data , & config , & mut client) ; });
}
