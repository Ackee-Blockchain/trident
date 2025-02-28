use solana_sdk::instruction::Instruction;

use super::TransactionCustomMethods;
use super::TransactionMethods;

use crate::traits::FuzzClient;

pub(crate) trait TransactionPrivateMethods:
    TransactionCustomMethods + std::fmt::Debug
{
    fn create_transaction(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) -> Vec<Instruction>;
}

impl<T: TransactionMethods> TransactionPrivateMethods for T {
    fn create_transaction(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) -> Vec<Instruction> {
        // Retrieve instruction identifiers used to distinguish between different instruction types
        let discriminators = self.get_instruction_discriminators();

        // Get the program IDs that will process these instructions
        let program_ids = self.get_instruction_program_ids();

        // Get the instruction-specific data/parameters
        let data = self.get_instruction_data(client, fuzz_accounts);

        // Get the account metadata needed for the instructions
        let accounts = self.get_instruction_accounts(client, fuzz_accounts);

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
        // Each instruction is created by combining its discriminator, program_id, data, and accounts
        itertools::multizip((discriminators, program_ids, data, accounts))
            .map(|(discriminator, program_id, data, accounts)| {
                let mut ix_data = vec![];
                ix_data.extend(discriminator);
                ix_data.extend(data);

                Instruction {
                    program_id,
                    data: ix_data,
                    accounts,
                }
            })
            .collect()
    }
}
