use solana_sdk::{
    account::AccountSharedData, program_option::COption, program_pack::Pack, pubkey::Pubkey,
    rent::Rent, signature::Keypair, signer::Signer,
};
use spl_token::state::Mint;

use crate::{fuzz_client::FuzzClient, AccountId};

use super::AccountsStorage;

pub struct MintStore {
    pub pubkey: Pubkey,
}

impl From<Pubkey> for MintStore {
    fn from(pubkey: Pubkey) -> Self {
        MintStore { pubkey }
    }
}

impl AccountsStorage<MintStore> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Pubkey {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let mint_account = Keypair::new();

            let authority = match freeze_authority {
                Some(a) => COption::Some(a),
                _ => COption::None,
            };

            let r = Rent::default();
            let lamports = r.minimum_balance(Mint::LEN);

            let mut account = AccountSharedData::new(lamports, Mint::LEN, &spl_token::id());

            let mint = Mint {
                is_initialized: true,
                mint_authority: COption::Some(*owner),
                freeze_authority: authority,
                decimals,
                ..Default::default()
            };

            let mut data = vec![0u8; Mint::LEN];
            Mint::pack(mint, &mut data[..]).unwrap();
            account.set_data_from_slice(&data);

            client.set_account_custom(&mint_account.pubkey(), &account);

            MintStore {
                pubkey: mint_account.pubkey(),
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
