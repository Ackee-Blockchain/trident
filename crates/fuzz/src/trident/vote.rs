use solana_sdk::pubkey::Pubkey;

use crate::trident::Trident;

use solana_sdk::account::AccountSharedData;
use solana_sdk::account::WritableAccount;
use solana_sdk::clock::Clock;

impl Trident {
    // #[cfg(feature = "vote")]
    pub fn create_vote_account(
        &mut self,
        address: Pubkey,
        node_pubkey: &Pubkey,
        authorized_voter: &Pubkey,
        authorized_withdrawer: &Pubkey,
        commission: u8,
        clock: &Clock,
    ) {
        use solana_sdk::rent::Rent;
        use solana_sdk::vote::state::VoteInit;
        use solana_sdk::vote::state::VoteState;
        use solana_sdk::vote::state::VoteStateVersions;

        let rent = Rent::default();
        let lamports = rent.minimum_balance(VoteState::size_of());
        let mut account = AccountSharedData::new(
            lamports,
            VoteState::size_of(),
            &solana_sdk::vote::program::ID,
        );

        let vote_state = VoteState::new(
            &VoteInit {
                node_pubkey: *node_pubkey,
                authorized_voter: *authorized_voter,
                authorized_withdrawer: *authorized_withdrawer,
                commission,
            },
            clock,
        );

        VoteState::serialize(
            &VoteStateVersions::Current(Box::new(vote_state)),
            account.data_as_mut_slice(),
        )
        .unwrap();

        self.set_account_custom(&address, &account);
    }
}
