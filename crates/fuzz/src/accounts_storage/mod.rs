#![allow(dead_code)]
use std::collections::HashMap;

use crate::AccountId;

pub mod keypair_store;
pub mod mint_store;
pub mod pda_store;
pub mod program_store;
pub mod stake_store;
pub mod token_store;
pub mod vote_store;

use crate::fuzz_client::FuzzClient;
pub use keypair_store::KeypairStore;
pub use mint_store::MintStore;
pub use pda_store::PdaStore;
pub use program_store::ProgramStore;
use solana_sdk::account::AccountSharedData;
use solana_sdk::pubkey::Pubkey;
pub use stake_store::StakeStore;
pub use token_store::TokenStore;
pub use vote_store::VoteStore;

pub struct AccountsStorage<T> {
    accounts: HashMap<AccountId, T>,
    _max_accounts: u8,
}

impl<T> AccountsStorage<T> {
    pub fn new(max_accounts: u8) -> Self {
        let accounts: HashMap<AccountId, T> = HashMap::new();
        Self {
            accounts,
            _max_accounts: max_accounts,
        }
    }

    pub fn set_custom(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        address: Pubkey,
        account: AccountSharedData,
    ) where
        T: From<Pubkey>,
    {
        client.set_account_custom(&address, &account);
        self.accounts.insert(account_id, T::from(address));
    }
}
impl<T> Default for AccountsStorage<T> {
    fn default() -> Self {
        Self::new(2)
    }
}
