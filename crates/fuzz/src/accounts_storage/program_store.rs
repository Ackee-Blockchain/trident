use solana_sdk::pubkey::Pubkey;

use crate::{fuzz_client::FuzzClient, AccountId};

use super::AccountsStorage;

pub struct ProgramStore {
    pub pubkey: Pubkey,
}
impl From<Pubkey> for ProgramStore {
    fn from(pubkey: Pubkey) -> Self {
        ProgramStore { pubkey }
    }
}

impl AccountsStorage<ProgramStore> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        _client: &mut impl FuzzClient,
        program_id: Pubkey,
    ) -> Pubkey {
        let program_id = self
            .accounts
            .entry(account_id)
            .or_insert_with(|| ProgramStore { pubkey: program_id });
        program_id.pubkey
    }
    pub fn get(&self, account_id: AccountId) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(v) => v.pubkey,
            None => Pubkey::new_unique(),
        }
    }
}
