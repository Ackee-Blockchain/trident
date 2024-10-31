#![allow(dead_code)]
use std::collections::HashMap;

use solana_sdk::{
    account::AccountSharedData,
    clock::{Clock, Epoch},
    pubkey::Pubkey,
    signature::Keypair,
    stake::state::Lockup,
};

use crate::{fuzz_client::FuzzClient, AccountId};

pub struct PdaStore {
    pub pubkey: Pubkey,
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
    pub pubkey: Pubkey,
}
pub struct VoteAccountStore {
    pub pubkey: Pubkey,
}

pub struct StakeAccountStore {
    pub pubkey: Pubkey,
}

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
// TODO Add an easy way to limit the number of created accounts
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
    pub fn get(&self, account_id: AccountId) -> Keypair {
        match self.accounts.get(&account_id) {
            Some(v) => v.insecure_clone(),
            None => Keypair::new(),
        }
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
impl From<Pubkey> for TokenStore {
    fn from(pubkey: Pubkey) -> Self {
        TokenStore { pubkey }
    }
}
impl From<Pubkey> for MintStore {
    fn from(pubkey: Pubkey) -> Self {
        MintStore { pubkey }
    }
}

impl From<Pubkey> for ProgramStore {
    fn from(pubkey: Pubkey) -> Self {
        ProgramStore { pubkey }
    }
}

impl From<Pubkey> for VoteAccountStore {
    fn from(pubkey: Pubkey) -> Self {
        VoteAccountStore { pubkey }
    }
}

impl From<Pubkey> for StakeAccountStore {
    fn from(pubkey: Pubkey) -> Self {
        StakeAccountStore { pubkey }
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
            let key = client.set_token_account(
                mint,
                owner,
                amount,
                delegate,
                is_native,
                delegated_amount,
                close_authority,
            );
            TokenStore { pubkey: key }
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
            let key = client.set_mint_account(decimals, owner, freeze_authority);
            MintStore { pubkey: key }
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

impl AccountsStorage<PdaStore> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
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

impl AccountsStorage<VoteAccountStore> {
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
            let key = client.set_vote_account(
                node_pubkey,
                authorized_voter,
                authorized_withdrawer,
                commission,
                clock,
            );
            VoteAccountStore { pubkey: key }
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

impl AccountsStorage<StakeAccountStore> {
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_delegated_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        voter_pubkey: Pubkey,
        staker: Pubkey,
        withdrawer: Pubkey,
        stake: u64,
        activation_epoch: Epoch,
        deactivation_epoch: Option<Epoch>,
        lockup: Option<Lockup>,
    ) -> Pubkey {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let key = client.set_delegated_stake_account(
                voter_pubkey,
                staker,
                withdrawer,
                stake,
                activation_epoch,
                deactivation_epoch,
                lockup,
            );
            StakeAccountStore { pubkey: key }
        });
        key.pubkey
    }
    pub fn get_or_create_initialized_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        staker: Pubkey,
        withdrawer: Pubkey,
        lockup: Option<Lockup>,
    ) -> Pubkey {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let key = client.set_initialized_stake_account(staker, withdrawer, lockup);
            StakeAccountStore { pubkey: key }
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
