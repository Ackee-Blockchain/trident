use trident_fuzz::fuzzing::*;
mod fuzz_instructions;
use additional_program::entry as entry_additional_program;
use fuzz_instructions::FuzzInstruction;
use fuzz_instructions::*;
use idl_test::entry as entry_idl_test;
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
    let program_additional_program = ProgramEntrypoint::new(
        pubkey!("8bPSKGoWCdAW8Hu3S1hLHPpBv8BNwse4jDyaXNrj3jWB"),
        None,
        processor!(entry_additional_program),
    );
    let program_idl_test = ProgramEntrypoint::new(
        pubkey!("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN"),
        None,
        processor!(entry_idl_test),
    );
    let config = TridentConfig::new();
    let mut client =
        TridentSVM::new_client(&[program_additional_program, program_idl_test], &config);
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : InstructionsSequence , client : TridentSVM , config : TridentConfig |);
}
