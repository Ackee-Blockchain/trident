use solana_sdk::pubkey::Pubkey;
use solana_stake_interface::state::Authorized;
use solana_stake_interface::state::Lockup;

use crate::trident::transaction_result::TransactionResult;
use crate::trident::Trident;

impl Trident {
    /// Creates and delegates a stake account in a single transaction
    ///
    /// This method creates a new stake account and immediately delegates it to the specified
    /// vote account, combining both operations into a single transaction.
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
    /// A `TransactionResult` indicating success or failure of the account creation and delegation
    pub fn create_and_delegate_account(
        &mut self,
        from_pubkey: &Pubkey,
        stake_pubkey: &Pubkey,
        vote_pubkey: &Pubkey,
        authorized: &Authorized,
        lockup: Lockup,
        lamports: u64,
    ) -> TransactionResult {
        let create_and_delegate =
            solana_stake_interface::instruction::create_account_and_delegate_stake(
                from_pubkey,
                stake_pubkey,
                vote_pubkey,
                authorized,
                &lockup,
                lamports,
            );
        self.process_transaction(&create_and_delegate, "Creating and Delegating Account")
    }

    /// Creates and initializes a stake account without delegation
    ///
    /// This method creates a new stake account with the specified authorities and lockup
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
    /// A `TransactionResult` indicating success or failure of the account creation
    pub fn create_initialized_account(
        &mut self,
        from_pubkey: &Pubkey,
        stake_pubkey: &Pubkey,
        authorized: &Authorized,
        lockup: Lockup,
        lamports: u64,
    ) -> TransactionResult {
        let create_account = solana_stake_interface::instruction::create_account(
            from_pubkey,
            stake_pubkey,
            authorized,
            &lockup,
            lamports,
        );

        self.process_transaction(&create_account, "Creating Initialized Account")
    }
}
