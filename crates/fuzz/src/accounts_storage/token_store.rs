use solana_sdk::{
    account::AccountSharedData, program_option::COption, program_pack::Pack, pubkey::Pubkey,
    rent::Rent, signature::Keypair, signer::Signer,
};

use crate::{fuzz_client::FuzzClient, AccountId};

use super::AccountsStorage;

pub struct TokenStore {
    pub pubkey: Pubkey,
}

impl From<Pubkey> for TokenStore {
    fn from(pubkey: Pubkey) -> Self {
        TokenStore { pubkey }
    }
}

impl AccountsStorage<TokenStore> {
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Pubkey {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let token_account = Keypair::new();

            let delegate = match delegate {
                Some(a) => COption::Some(a),
                _ => COption::None,
            };

            let is_native = match is_native {
                Some(a) => COption::Some(a),
                _ => COption::None,
            };

            let close_authority = match close_authority {
                Some(a) => COption::Some(a),
                _ => COption::None,
            };

            let r = Rent::default();
            let lamports = r.minimum_balance(spl_token::state::Account::LEN);

            let mut account =
                AccountSharedData::new(lamports, spl_token::state::Account::LEN, &spl_token::id());

            let token_account_ = spl_token::state::Account {
                mint,
                owner,
                amount,
                delegate,
                state: spl_token::state::AccountState::Initialized,
                is_native,
                delegated_amount,
                close_authority,
            };

            let mut data = vec![0u8; spl_token::state::Account::LEN];
            spl_token::state::Account::pack(token_account_, &mut data[..]).unwrap();
            account.set_data_from_slice(&data);

            client.set_account_custom(&token_account.pubkey(), &account);

            TokenStore {
                pubkey: token_account.pubkey(),
            }
        });
        key.pubkey
    }
    pub fn get(&self, account_id: AccountId) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(v) => v.pubkey,
            None => Pubkey::default(),
        }
    }
}
