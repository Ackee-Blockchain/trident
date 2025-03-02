use solana_sdk::instruction::Instruction;

use super::TransactionHooks;
use super::TransactionMethods;

use crate::traits::FuzzClient;

/// Private trait that provides internal implementation details for transaction processing
///
/// This trait is not meant to be implemented directly by users.
/// It's implemented automatically for any type that implements TransactionMethods.
pub(crate) trait TransactionPrivateMethods: TransactionHooks + std::fmt::Debug {
    /// Creates a vector of Solana instructions from the transaction data
    ///
    /// This method assembles complete Solana instructions by combining:
    /// - Instruction discriminators (identifiers)
    /// - Program IDs that will process the instructions
    /// - Instruction-specific data/parameters
    /// - Account metadata needed for the instructions
    fn create_transaction(&mut self, client: &mut impl FuzzClient) -> Vec<Instruction>;
}

impl<T: TransactionMethods> TransactionPrivateMethods for T {
    fn create_transaction(&mut self, client: &mut impl FuzzClient) -> Vec<Instruction> {
        // Retrieve instruction discriminators (identifiers for different instruction types)
        let discriminators = self.get_instruction_discriminators();

        // Get the program IDs that will process these instructions
        let program_ids = self.get_instruction_program_ids();

        // Get the instruction-specific data/parameters
        let data = self.get_instruction_data(client);

        // Get the account metadata needed for the instructions
        let accounts = self.get_instruction_accounts(client);

        // Debug output when fuzzing_debug feature is enabled
        #[allow(unexpected_cfgs)]
        {
            if cfg!(fuzzing_debug) {
                println!(
                    "\x1b[96mCurrently processing transaction with instructions\x1b[0m: {:#?}",
                    self
                );
            }
        }

        // Combine all components to create a vector of Instructions
        // Each instruction consists of:
        // - program_id: The program that will process this instruction
        // - data: Combined discriminator and instruction-specific data
        // - accounts: The accounts involved in this instruction
        itertools::multizip((discriminators, program_ids, data, accounts))
            .map(|(discriminator, program_id, data, accounts)| {
                // Combine discriminator and instruction data into a single byte vector
                let mut ix_data = vec![];
                ix_data.extend(discriminator);
                ix_data.extend(data);

                // Create the complete Solana instruction
                Instruction {
                    program_id,
                    data: ix_data,
                    accounts,
                }
            })
            .collect()
    }
}
