use trident_fuzz::fuzzing::*;
mod fuzz_instructions;
use fuzz_instructions::FuzzTransactions;
use fuzz_instructions::*;
use hello_world::entry as entry_hello_world;
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
///         let seq1 = transaction!([InitializeFn, UpdateFn], fuzzer_data);
///         Ok(seq1)
///     }
///     fn middle_sequence(fuzzer_data: &mut FuzzerData) ->
/// SequenceResult<FuzzTransactions> {
///         let seq1 = transaction!([WithdrawFn], fuzzer_data);
///         Ok(seq1)
///     }
///}
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/instructions-sequences/#instructions-sequences
impl FuzzSequenceBuilder<FuzzTransactions> for TransactionsSequence {
    fn starting_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        let seq = transaction!([InitializeFnTransaction], fuzzer_data);

        Ok(seq)
    }
    fn middle_sequence(_fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        Ok(vec![])
    }
    fn ending_sequence(_fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        Ok(vec![])
    }
}
fn main() {
    let program_hello_world = ProgramEntrypoint::new(
        pubkey!("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit"),
        None,
        processor!(entry_hello_world),
    );
    let config = TridentConfig::new();
    let mut client = TridentSVM::new_client(&[program_hello_world], &config);
    fuzz_trident ! (| fuzz_data : TransactionsSequence , client : TridentSVM , config : TridentConfig |);
}
