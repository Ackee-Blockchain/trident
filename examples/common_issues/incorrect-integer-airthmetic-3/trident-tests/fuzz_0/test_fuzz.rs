use fuzz_transactions::FuzzTransactions;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use incorrect_integer_arithmetic_3::entry as entry_incorrect_integer_arithmetic_3;
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
impl FuzzSequenceBuilder<FuzzTransactions> for TransactionsSequence {
    fn starting_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        let seq1 = sequence!([InitVestingTransaction], fuzzer_data);
        Ok(seq1)
    }
    fn middle_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        let seq1 = sequence!([WithdrawUnlockedTransaction], fuzzer_data);
        Ok(seq1)
    }
}
fn main() {
    let program_incorrect_integer_arithmetic_3 = ProgramEntrypoint::new(
        pubkey!("4VS6YQzYbdEVbt9iw1eoUnpNzqC5TJPoSeJH9qFfgm4m"),
        None,
        processor!(entry_incorrect_integer_arithmetic_3),
    );
    let config = TridentConfig::new();
    let mut client = TridentSVM::new_client(&[program_incorrect_integer_arithmetic_3], &config);
    fuzz_trident!(| fuzz_data : TransactionsSequence , client : TridentSVM , config : TridentConfig |);
}
