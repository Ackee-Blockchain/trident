use fuzz_transactions::FuzzTransactions;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use callee::entry as entry_callee;
use caller::entry as entry_caller;
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
        let seq1 = sequence!([InitializeCallerTransaction], fuzzer_data);
        Ok(seq1)
    }
    fn middle_sequence(_fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        Ok(vec![])
    }
    fn ending_sequence(_fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        Ok(vec![])
    }
}
fn main() {
    let program_callee = ProgramEntrypoint::new(
        pubkey!("HJR1TK8bgrUWzysdpS1pBGBYKF7zi1tU9cS4qj8BW8ZL"),
        None,
        processor!(entry_callee),
    );
    let program_caller = ProgramEntrypoint::new(
        pubkey!("FWtSodrkUnovFPnNRCxneP6VWh6JH6jtQZ4PHoP8Ejuz"),
        None,
        processor!(entry_caller),
    );
    let config = TridentConfig::new();
    let mut client = TridentSVM::new_client(&[program_callee, program_caller], &config);
    fuzz_trident!(| fuzz_data : TransactionsSequence , client : TridentSVM , config : TridentConfig |);
}
