use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::trident::Trident;

impl Trident {
    /// Creates instructions to initialize a vote account with the specified configuration
    ///
    /// Generates instructions to create and initialize a vote account with the provided
    /// validator configuration and authorities.
    ///
    /// # Arguments
    /// * `from_pubkey` - The public key of the account to create the vote account from
    /// * `vote_pubkey` - The public key of the vote account to create
    /// * `node_pubkey` - The public key of the validator's node
    /// * `authorized_voter` - The public key of the authority allowed to submit votes
    /// * `authorized_withdrawer` - The public key of the authority allowed to withdraw from the account
    /// * `commission` - The commission percentage (0-100)
    /// * `lamports` - The number of lamports to transfer to the vote account
    ///
    /// # Returns
    /// A vector of instructions that need to be executed with `process_transaction`
    #[allow(clippy::too_many_arguments)]
    pub fn initialize_vote_account(
        &mut self,
        from_pubkey: &Pubkey,
        vote_pubkey: &Pubkey,
        node_pubkey: &Pubkey,
        authorized_voter: &Pubkey,
        authorized_withdrawer: &Pubkey,
        commission: u8,
        lamports: u64,
    ) -> Vec<Instruction> {
        let config = solana_vote_interface::instruction::CreateVoteAccountConfig::default();

        let vote_init = solana_vote_interface::state::VoteInit {
            node_pubkey: *node_pubkey,
            authorized_voter: *authorized_voter,
            authorized_withdrawer: *authorized_withdrawer,
            commission,
        };
        solana_vote_interface::instruction::create_account_with_config(
            from_pubkey,
            vote_pubkey,
            &vote_init,
            lamports,
            config,
        )
    }
}
