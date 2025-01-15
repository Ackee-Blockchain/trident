use trident_fuzz::fuzzing::*;
mod fuzz_instructions;
use fuzz_instructions::FuzzInstruction;
use fuzz_instructions::*;
use maze3::entry as entry_maze3;
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
fn main() {
    let program_maze3 = ProgramEntrypoint::new(
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq"),
        None,
        processor!(entry_maze3),
    );
    let config = TridentConfig::new();
    let mut client = TridentSVM::new_client(&[program_maze3], &config);
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence , client : TridentSVM , config : TridentConfig |);
}
