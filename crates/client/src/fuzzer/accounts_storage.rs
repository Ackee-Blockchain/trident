use std::collections::HashMap;

use solana_sdk::{pubkey::Pubkey, signature::Keypair};

use crate::{data_builder::FuzzClient, AccountId};


pub struct PdaStore {
    pubkey: Pubkey,
    pub seeds: Vec<Vec<u8>>,
}
impl PdaStore {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }
}

pub struct TokenStore {
    pub pubkey: Pubkey,
}

pub struct MintStore {
    pub pubkey: Pubkey,
}

pub struct ProgramStore {
    pub pubkey: u8,
}

pub struct AccountsStorage<T> {
    accounts: HashMap<AccountId, T>,
    pub max_accounts: u8,
}

impl<T> AccountsStorage<T> {
    pub fn new(max_accounts: u8) -> Self {
        let accounts: HashMap<AccountId, T> = HashMap::new();
        Self {
            accounts,
            max_accounts,
        }
    }

    pub fn get(&self, account_id: AccountId) -> Option<&T> {
        self.accounts.get(&account_id)
    }
}

impl<T> Default for AccountsStorage<T> {
    fn default() -> Self {
        Self::new(2)
    }
}

impl AccountsStorage<Keypair> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        lamports: u64,
    ) -> Keypair {
        let key = self
            .accounts
            .entry(account_id)
            .or_insert_with(|| client.set_account(lamports));
        key.insecure_clone()
    }
}

impl AccountsStorage<TokenStore> {
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
    ) -> Option<Pubkey> {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let key = client.set_token_account(mint, owner, amount, delegate, is_native, delegated_amount, close_authority);
            TokenStore { pubkey: key }
        });
        Some(key.pubkey)
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
    ) -> Option<Pubkey> {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let key = client.set_mint_account(decimals, owner, freeze_authority);
            MintStore { pubkey: key }
        });
        Some(key.pubkey)
    }
}

impl AccountsStorage<PdaStore> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        seeds: &[&[u8]],
        program_id: &Pubkey,
    ) -> Option<&PdaStore> {
        let key = self.accounts.entry(account_id).or_insert(
            if let Some((key, _)) = Pubkey::try_find_program_address(seeds, program_id) {
                let seeds_vec: Vec<_> = seeds.iter().map(|&s| s.to_vec()).collect();
                PdaStore {
                    pubkey: key,
                    seeds: seeds_vec,
                }
            } else {
                return None;
            },
        );
        Some(key)
    }
}
