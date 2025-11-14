use solana_sdk::account::ReadableAccount;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::trident::Trident;

impl Trident {
    #[allow(dead_code)]
    pub(crate) fn create_account(
        &mut self,
        address: &Pubkey,
        from: &Pubkey,
        space: usize,
        owner: &Pubkey,
    ) -> Vec<Instruction> {
        let account = self.client.get_account(address).unwrap_or_default();

        let rent = solana_sdk::rent::Rent::default();
        if account.lamports() > 0 {
            let mut instructions = vec![];
            let lamports_required = rent.minimum_balance(space);

            let remaining_lamports = lamports_required.saturating_sub(account.lamports());

            if remaining_lamports > 0 {
                let transfer =
                    solana_sdk::system_instruction::transfer(from, address, remaining_lamports);
                instructions.push(transfer);
            }

            let allocate = solana_sdk::system_instruction::allocate(address, space as u64);
            instructions.push(allocate);

            let assign = solana_sdk::system_instruction::assign(address, owner);
            instructions.push(assign);

            instructions
        } else {
            let ix = solana_sdk::system_instruction::create_account(
                from,
                address,
                rent.minimum_balance(space),
                space as u64,
                owner,
            );
            vec![ix]
        }
    }
    /// Creates an instruction to transfer SOL from one account to another
    ///
    /// Generates a system program transfer instruction to move the specified amount
    /// of lamports from the source to destination account.
    ///
    /// # Arguments
    /// * `from` - The public key of the account to transfer from
    /// * `to` - The public key of the account to transfer to
    /// * `amount` - The number of lamports to transfer
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn transfer(&mut self, from: &Pubkey, to: &Pubkey, amount: u64) -> Instruction {
        solana_sdk::system_instruction::transfer(from, to, amount)
    }
}
