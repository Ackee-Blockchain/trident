use trident_fuzz::fuzzing::*;
mod fuzz_instructions;
use cpi_metaplex_7::entry;
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
    pre_sequence!(Initialize);
    middle_sequence!();
    post_sequence!();
}
fn main() {
    let program = ProgramEntrypoint::new(
        pubkey!("3XtULmXDGS867VbBXiPkjYr4EMjytGW8X12F6BS23Zcw"),
        None,
        processor!(entry),
    );

    let config = Config::new();
    let mut client = TridentSVM::new_client(&[program], &config);
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence , client : TridentSVM , config : Config |);
}
