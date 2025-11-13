use solana_sdk::account::ReadableAccount;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::trident::Trident;

impl Trident {
    #[allow(dead_code)]
    pub(crate) fn create_account_internal(
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
    /// Creates a new account
    ///
    /// Generates a system program create_account instruction to allocate space
    /// and assign ownership of a new account.
    ///
    /// # Arguments
    /// * `from_pubkey` - The public key of the account funding the new account
    /// * `to_pubkey` - The public key of the new account to create
    /// * `lamports` - The number of lamports to transfer to the new account
    /// * `space` - The number of bytes to allocate for the account data
    /// * `owner` - The program that will own the new account
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn create_account(
        &mut self,
        from_pubkey: &Pubkey,
        to_pubkey: &Pubkey,
        lamports: u64,
        space: u64,
        owner: &Pubkey,
    ) -> Instruction {
        solana_sdk::system_instruction::create_account(
            from_pubkey,
            to_pubkey,
            lamports,
            space,
            owner,
        )
    }

    /// Allocates space for an account
    ///
    /// Generates a system program allocate instruction to allocate the specified
    /// number of bytes for an account's data.
    ///
    /// # Arguments
    /// * `address` - The public key of the account to allocate space for
    /// * `space` - The number of bytes to allocate
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    ///
    /// # Note
    /// This will succeed on PDAs in Trident, but would fail on-chain outside of a program invocation
    /// since PDAs cannot sign transactions
    pub fn allocate(&mut self, address: &Pubkey, space: u64) -> Instruction {
        solana_sdk::system_instruction::allocate(address, space)
    }

    /// Assigns an account to a program
    ///
    /// Generates a system program assign instruction to change the owner
    /// of an account to the specified program.
    ///
    /// # Arguments
    /// * `address` - The public key of the account to assign
    /// * `owner` - The public key of the program that will own the account
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    ///
    /// # Note
    /// This will succeed on PDAs in Trident, but would fail on-chain outside of a program invocation
    /// since PDAs cannot sign transactions
    pub fn assign(&mut self, address: &Pubkey, owner: &Pubkey) -> Instruction {
        solana_sdk::system_instruction::assign(address, owner)
    }

    /// Transfers SOL from one account to another
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
