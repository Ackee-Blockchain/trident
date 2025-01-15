use trident_fuzz::fuzzing::*;
mod fuzz_instructions;
use fuzz_instructions::FuzzInstruction;
use fuzz_instructions::*;

use unchecked_arithmetic_0::entry;

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
impl FuzzDataBuilder<FuzzInstruction> for InstructionsSequence {}
fn main() {
    let program = ProgramEntrypoint::new(
        pubkey!("BM8vocQeC2VuDf1KhbHLsZxTh7owzDNTAkKyZoTxFiUs"),
        None,
        processor!(entry),
    );

    let config = TridentConfig::new();
    let mut client = TridentSVM::new_client(&[program], &config);
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence , client : TridentSVM , config : TridentConfig |);
}
