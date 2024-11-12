use solana_sdk::{account::AccountSharedData, pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::{fuzz_client::FuzzClient, AccountId};

use super::AccountsStorage;

pub struct KeypairStore {
    pub keypair: Keypair,
}

impl KeypairStore {
    pub fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }
}

// TODO Add an easy way to limit the number of created accounts
impl AccountsStorage<KeypairStore> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        lamports: u64,
    ) -> Keypair {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let keypair = Keypair::new();

            let empty_account =
                AccountSharedData::new(lamports, 0, &solana_sdk::system_program::ID);
            client.set_account_custom(&keypair.pubkey(), &empty_account);
            KeypairStore {
                keypair: keypair.insecure_clone(),
            }
        });
        key.keypair.insecure_clone()
    }
    pub fn get(&self, account_id: AccountId) -> Keypair {
        match self.accounts.get(&account_id) {
            Some(v) => v.keypair.insecure_clone(),
            None => Keypair::new(),
        }
    }
}
