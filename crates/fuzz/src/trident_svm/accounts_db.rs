use std::collections::HashMap;

use serde::de::DeserializeOwned;
use solana_sdk::{
    account::{AccountSharedData, ReadableAccount},
    pubkey::Pubkey,
    sysvar::SysvarId,
};

#[derive(Default)]
pub struct AccountsDB {
    accounts: HashMap<Pubkey, AccountSharedData>,
    permanent_accounts: HashMap<Pubkey, AccountSharedData>,
    programs: HashMap<Pubkey, AccountSharedData>,
    sysvars: HashMap<Pubkey, AccountSharedData>,
}

impl AccountsDB {
    pub fn get_account(&self, pubkey: &Pubkey) -> Option<AccountSharedData> {
        if let Some(account) = self.accounts.get(pubkey) {
            Some(account.to_owned())
        } else if let Some(permanent_account) = self.permanent_accounts.get(pubkey) {
            Some(permanent_account.to_owned())
        } else if let Some(program) = self.get_program(pubkey) {
            Some(program)
        } else {
            self.sysvars.get(pubkey).cloned()
        }
    }
    pub fn get_sysvar<S: SysvarId + DeserializeOwned>(&self) -> S {
        bincode::deserialize(self.accounts.get(&S::id()).unwrap().data()).unwrap()
    }
    fn get_program(&self, pubkey: &Pubkey) -> Option<AccountSharedData> {
        self.programs.get(pubkey).map(|acc| acc.to_owned())
    }
    pub fn add_account(&mut self, pubkey: &Pubkey, account: &AccountSharedData) {
        let _ = self.accounts.insert(pubkey.to_owned(), account.to_owned());
    }
    pub fn add_permanent_account(&mut self, pubkey: &Pubkey, account: &AccountSharedData) {
        let _ = self
            .permanent_accounts
            .insert(pubkey.to_owned(), account.to_owned());
    }
    pub fn add_sysvar(&mut self, pubkey: &Pubkey, account: &AccountSharedData) {
        let _ = self.sysvars.insert(pubkey.to_owned(), account.to_owned());
    }
    pub fn add_program(&mut self, pubkey: &Pubkey, account: &AccountSharedData) {
        let _ = self.programs.insert(pubkey.to_owned(), account.to_owned());
    }

    pub fn reset_temp(&mut self) {
        self.accounts = Default::default();
    }
}
