#![allow(dead_code)] // The Snapshot is constructed in the FuzzTestExecutor macro and is generated automatically

use anchor_lang::solana_program::account_info::Account as Acc;
use anchor_lang::solana_program::account_info::AccountInfo;
use solana_sdk::{account::Account, instruction::AccountMeta};

use crate::fuzz_client::FuzzClient;

use crate::error::*;
use crate::ix_ops::IxOps;
pub struct Snapshot<'info, T> {
    before: Vec<Option<Account>>,
    before_acc_inf: Vec<Option<AccountInfo<'info>>>,
    after: Vec<Option<Account>>,
    after_acc_inf: Vec<Option<AccountInfo<'info>>>,
    metas: Vec<AccountMeta>,
    ix: &'info T,
}

impl<'info, T> Snapshot<'info, T>
where
    T: IxOps<'info>,
{
    pub fn new_empty(ix: &'info T) -> Snapshot<'info, T> {
        let capacity = 0;
        Self {
            before: Vec::with_capacity(capacity),
            before_acc_inf: Vec::with_capacity(capacity),
            after: Vec::with_capacity(capacity),
            after_acc_inf: Vec::with_capacity(capacity),
            metas: vec![],
            ix,
        }
    }
    pub fn add_metas(&mut self, metas: &[AccountMeta]) {
        let capacity = metas.len();
        self.before = Vec::with_capacity(capacity);
        self.before_acc_inf = Vec::with_capacity(capacity);
        self.after = Vec::with_capacity(capacity);
        self.after_acc_inf = Vec::with_capacity(capacity);
        self.metas = metas.to_vec();
    }
    pub fn new(metas: &[AccountMeta], ix: &'info T) -> Snapshot<'info, T> {
        let capacity = metas.len();
        Self {
            before: Vec::with_capacity(capacity),
            before_acc_inf: Vec::with_capacity(capacity),
            after: Vec::with_capacity(capacity),
            after_acc_inf: Vec::with_capacity(capacity),
            metas: metas.to_vec(),
            ix,
        }
    }

    pub fn capture_before(
        &mut self,
        client: &mut impl FuzzClient,
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        self.before = self
            .capture(client)
            .map_err(|e| e.with_context(Context::Pre))?;
        Ok(())
    }

    pub fn capture_after(
        &mut self,
        client: &mut impl FuzzClient,
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        self.after = self
            .capture(client)
            .map_err(|e| e.with_context(Context::Post))?;
        Ok(())
    }

    fn capture(
        &mut self,
        client: &mut impl FuzzClient,
    ) -> Result<Vec<Option<Account>>, FuzzClientErrorWithOrigin> {
        client.get_accounts(&self.metas)
    }

    fn calculate_account_info(
        accounts: &'info mut [Option<Account>],
        metas: &'info [AccountMeta],
    ) -> Vec<Option<AccountInfo<'info>>> {
        accounts
            .iter_mut()
            .zip(metas)
            .map(|(account, meta)| {
                if let Some(account) = account {
                    let (lamports, data, owner, executable, rent_epoch) = account.get();
                    Some(AccountInfo::new(
                        &meta.pubkey,
                        meta.is_signer,
                        meta.is_writable,
                        lamports,
                        data,
                        owner,
                        executable,
                        rent_epoch,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    fn set_missing_accounts_to_default(accounts: &mut [Option<Account>]) {
        for acc in accounts.iter_mut() {
            if acc.is_none() {
                *acc = Some(solana_sdk::account::Account::default());
            }
        }
    }

    pub fn get_raw_pre_ix_accounts(&'info mut self) -> &[Option<AccountInfo<'info>>] {
        Self::set_missing_accounts_to_default(&mut self.before);
        self.before_acc_inf = Self::calculate_account_info(&mut self.before, &self.metas);
        &self.before_acc_inf
    }

    pub fn get_snapshot(
        &'info mut self,
    ) -> Result<(T::IxSnapshot, T::IxSnapshot), FuzzingErrorWithOrigin> {
        // When user passes an account that is not initialized, the runtime will provide
        // a default empty account to the program. If the uninitialized account is of type
        // AccountInfo, Signer or UncheckedAccount, Anchor will not return an error. However
        // when we try to fetch "on-chain" accounts and an account is not initilized, this
        // account simply does not exist and the get_account() method returns None. To prevent
        // errors during deserialization due to missing accounts, we replace the missing accounts
        // with default values similar as the runtime does.
        Self::set_missing_accounts_to_default(&mut self.before);
        Self::set_missing_accounts_to_default(&mut self.after);

        self.before_acc_inf = Self::calculate_account_info(&mut self.before, &self.metas);
        self.after_acc_inf = Self::calculate_account_info(&mut self.after, &self.metas);

        let mut remaining_accounts_before: &[Option<AccountInfo<'info>>] = &self.before_acc_inf;
        let mut remaining_accounts_after: &[Option<AccountInfo<'info>>] = &self.after_acc_inf;

        let pre_ix = self
            .ix
            .deserialize_accounts(&mut remaining_accounts_before)
            .map_err(|e| e.with_context(Context::Pre))?;
        let post_ix = self
            .ix
            .deserialize_accounts(&mut remaining_accounts_after)
            .map_err(|e| e.with_context(Context::Post))?;
        Ok((pre_ix, post_ix))
    }
}
