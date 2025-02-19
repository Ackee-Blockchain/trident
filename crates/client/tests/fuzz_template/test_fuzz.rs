use fuzz_transactions::FuzzTransactions;
use fuzz_transactions::*;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use additional_program::entry as entry_additional_program;
use idl_test::entry as entry_idl_test;
pub use transactions::*;
struct TransactionsSequence;
/// Define transaction sequences for execution.
/// `starting_sequence` runs at the start, `middle` in the middle, and `ending`
/// at the end.
/// For example, to call `InitializeFn`, `UpdateFn` and then `WithdrawFn` during
/// each fuzzing iteration:
/// ```
/// impl FuzzDataBuilder<FuzzTransactions> for InstructionsSequence {
///     fn starting_sequence(fuzzer_data: &mut FuzzerData) ->
/// SequenceResult<FuzzTransactions> {
///         let seq1 = sequence!([InitializeFn, UpdateFn], fuzzer_data);
///         Ok(seq1)
///     }
///     fn middle_sequence(fuzzer_data: &mut FuzzerData) ->
/// SequenceResult<FuzzTransactions> {
///         let seq1 = sequence!([WithdrawFn], fuzzer_data);
///         Ok(seq1)
///     }
///}
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/instructions-sequences/#instructions-sequences
impl FuzzSequenceBuilder<FuzzTransactions> for TransactionsSequence {}
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
    fuzz_trident ! (| fuzz_data : TransactionsSequence , client : TridentSVM , config : TridentConfig |);
}
