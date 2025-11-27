use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_stake_interface::state::Authorized;
use solana_stake_interface::state::Lockup;

use crate::trident::Trident;

impl Trident {
    /// Creates instructions to create and delegate a stake account
    ///
    /// Generates instructions to create a new stake account and immediately delegate it to the specified
    /// vote account, combining both operations.
    ///
    /// # Arguments
    /// * `from_pubkey` - The public key of the account funding the stake account creation
    /// * `stake_pubkey` - The public key of the stake account to create
    /// * `vote_pubkey` - The public key of the vote account to delegate to
    /// * `authorized` - The authorized staker and withdrawer authorities
    /// * `lockup` - The lockup configuration for the stake account
    /// * `lamports` - The number of lamports to transfer to the stake account
    ///
    /// # Returns
    /// A vector of instructions that need to be executed with `process_transaction`
    pub fn create_and_delegate_account(
        &mut self,
        from_pubkey: &Pubkey,
        stake_pubkey: &Pubkey,
        vote_pubkey: &Pubkey,
        authorized: &Authorized,
        lockup: Lockup,
        lamports: u64,
    ) -> Vec<Instruction> {
        solana_stake_interface::instruction::create_account_and_delegate_stake(
            from_pubkey,
            stake_pubkey,
            vote_pubkey,
            authorized,
            &lockup,
            lamports,
        )
    }

    /// Creates instructions to initialize a stake account without delegation
    ///
    /// Generates instructions to create a new stake account with the specified authorities and lockup
    /// configuration, but does not delegate it to any vote account.
    ///
    /// # Arguments
    /// * `from_pubkey` - The public key of the account funding the stake account creation
    /// * `stake_pubkey` - The public key of the stake account to create
    /// * `authorized` - The authorized staker and withdrawer authorities
    /// * `lockup` - The lockup configuration for the stake account
    /// * `lamports` - The number of lamports to transfer to the stake account
    ///
    /// # Returns
    /// A vector of instructions that need to be executed with `process_transaction`
    pub fn create_initialized_account(
        &mut self,
        from_pubkey: &Pubkey,
        stake_pubkey: &Pubkey,
        authorized: &Authorized,
        lockup: Lockup,
        lamports: u64,
    ) -> Vec<Instruction> {
        solana_stake_interface::instruction::create_account(
            from_pubkey,
            stake_pubkey,
            authorized,
            &lockup,
            lamports,
        )
    }
}
