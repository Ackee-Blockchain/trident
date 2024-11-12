use solana_sdk::{account::AccountSharedData, pubkey::Pubkey};

use crate::{fuzz_client::FuzzClient, AccountId};

use super::AccountsStorage;

pub struct PdaStore {
    pub pubkey: Pubkey,
    pub seeds: Vec<Vec<u8>>,
}
impl PdaStore {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }
}

impl From<Pubkey> for PdaStore {
    fn from(pubkey: Pubkey) -> Self {
        PdaStore {
            pubkey,
            seeds: Vec::new(), // Note: This creates empty seeds
        }
    }
}

impl AccountsStorage<PdaStore> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: &[&[u8]],
        program_id: &Pubkey,
    ) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(v) => v.pubkey,
            None => {
                if let Some((key, _)) = Pubkey::try_find_program_address(seeds, program_id) {
                    let seeds_vec: Vec<_> = seeds.iter().map(|&s| s.to_vec()).collect();
                    let pda_store = PdaStore {
                        pubkey: key,
                        seeds: seeds_vec,
                    };
                    self.accounts.insert(account_id, pda_store);
                    client.set_account_custom(
                        &key,
                        &AccountSharedData::new(0, 0, &solana_sdk::system_program::ID),
                    );
                    key
                } else {
                    Pubkey::new_unique()
                }
            }
        }
    }
    pub fn get(&self, account_id: AccountId) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(v) => v.pubkey,
            None => Pubkey::new_unique(),
        }
    }
}
