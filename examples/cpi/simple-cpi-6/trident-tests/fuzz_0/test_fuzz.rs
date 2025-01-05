use trident_fuzz::fuzzing::*;
mod fuzz_instructions;
use callee::entry as entry_callee;
use caller::entry as entry_caller;
use fuzz_instructions::FuzzInstruction;
use fuzz_instructions::*;

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
    pre_sequence!(InitializeCaller);
    middle_sequence!();
    post_sequence!();
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
    let program_callee = ProgramEntrypoint {
        program_id: pubkey!("HJR1TK8bgrUWzysdpS1pBGBYKF7zi1tU9cS4qj8BW8ZL"),
        authority: None,
        entry: processor!(entry_callee),
    };
    let program_caller = ProgramEntrypoint {
        program_id: pubkey!("FWtSodrkUnovFPnNRCxneP6VWh6JH6jtQZ4PHoP8Ejuz"),
        authority: None,
        entry: processor!(entry_caller),
    };
    let config = Config::new();
    let mut client = TridentSVM::new_client(&[program_callee, program_caller], &config);
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence | { fuzz_iteration (fuzz_data , & config , & mut client) ; });
}
