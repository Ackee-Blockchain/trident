use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    clock::Clock,
    pubkey::Pubkey,
    rent::Rent,
    signature::Keypair,
    signer::Signer,
};
use solana_vote_program::vote_state::{VoteInit, VoteState, VoteStateVersions};

use crate::{fuzz_client::FuzzClient, AccountId};

use super::AccountsStorage;

pub struct VoteStore {
    pub pubkey: Pubkey,
}

impl From<Pubkey> for VoteStore {
    fn from(pubkey: Pubkey) -> Self {
        VoteStore { pubkey }
    }
}

impl AccountsStorage<VoteStore> {
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        node_pubkey: &Pubkey,
        authorized_voter: &Pubkey,
        authorized_withdrawer: &Pubkey,
        commission: u8,
        clock: &Clock,
    ) -> Pubkey {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let vote_account = Keypair::new();

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

            client.set_account_custom(&vote_account.pubkey(), &account);

            VoteStore {
                pubkey: vote_account.pubkey(),
            }
        });
        key.pubkey
    }
    pub fn get(&self, account_id: AccountId) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(v) => v.pubkey,
            None => Pubkey::new_unique(),
        }
    }
}
