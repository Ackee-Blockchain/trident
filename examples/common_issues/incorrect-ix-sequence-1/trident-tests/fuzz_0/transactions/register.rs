use crate::fuzz_transactions::FuzzAccounts;
use crate::instructions::*;
use crate::types::*;
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct RegisterTransaction {
    pub instruction: RegisterInstruction,
}
/// Custom Transaction Methods
///
/// Provides hooks for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
impl TransactionHooks for RegisterTransaction {
    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // This fuzz check will reveal that registrations can be performed
        // even though registration windows is not open.

        let state = State::deserialize(
            &mut self
                .instruction
                .accounts
                .state
                .get_snapshot_before()
                .data_no_discriminator(),
        );

        let project = Project::deserialize(
            &mut self
                .instruction
                .accounts
                .project
                .get_snapshot_after()
                .data_no_discriminator(),
        );

        if let Ok(_project) = project {
            if let Ok(state) = state {
                let registrations_round = state.registrations_round;
                if !registrations_round {
                    return Err(FuzzingError::Custom(1));
                }
            }
        }
        Ok(())
    }
}
