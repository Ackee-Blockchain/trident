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
/// Define the order in which the transactions are executed:
/// - `starting_sequence`
/// - `middle_sequence`
/// - `ending_sequence`
///
/// Docs: https://ackee.xyz/trident/docs/latest/features/trident-advanced/trident-transactions/trident-fuzzing-flows/
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
