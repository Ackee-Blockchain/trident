#![allow(dead_code)] // The Snapshot is constructed in the FuzzTestExecutor macro and is generated automatically

use anchor_lang::solana_program::account_info::Account as Acc;
use anchor_lang::solana_program::account_info::AccountInfo;
use solana_sdk::{account::Account, instruction::AccountMeta};

use crate::fuzz_client::FuzzClient;
use crate::fuzz_deserialize::FuzzDeserialize;

use crate::error::*;
pub struct Snapshot<'info, T> {
    before: Vec<Option<Account>>,
    before_acc_inf: Vec<Option<AccountInfo<'info>>>,
    after: Vec<Option<Account>>,
    after_acc_inf: Vec<Option<AccountInfo<'info>>>,
    metas: &'info [AccountMeta],
    ix: &'info T,
}

impl<'info, T> Snapshot<'info, T>
where
    T: FuzzDeserialize<'info>,
{
    pub fn new(metas: &'info [AccountMeta], ix: &'info T) -> Snapshot<'info, T> {
        let capacity = metas.len();
        Self {
            before: Vec::with_capacity(capacity),
            before_acc_inf: Vec::with_capacity(capacity),
            after: Vec::with_capacity(capacity),
            after_acc_inf: Vec::with_capacity(capacity),
            metas,
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
        client.get_accounts(self.metas)
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

    pub fn get_raw_pre_ix_accounts(&'info mut self) -> Vec<Option<AccountInfo<'info>>> {
        Self::set_missing_accounts_to_default(&mut self.before);
        Self::calculate_account_info(&mut self.before, self.metas)
    }

    pub fn get_snapshot(
        &'info mut self,
        program_id: &solana_sdk::pubkey::Pubkey,
    ) -> Result<(T::Ix, T::Ix), FuzzingErrorWithOrigin> {
        // When user passes an account that is not initialized, the runtime will provide
        // a default empty account to the program. If the uninitialized account is of type
        // AccountInfo, Signer or UncheckedAccount, Anchor will not return an error. However
        // when we try to fetch "on-chain" accounts and an account is not initilized, this
        // account simply does not exist and the get_account() method returns None. To prevent
        // errors during deserialization due to missing accounts, we replace the missing accounts
        // with default values similar as the runtime does.
        Self::set_missing_accounts_to_default(&mut self.before);
        Self::set_missing_accounts_to_default(&mut self.after);

        self.before_acc_inf = Self::calculate_account_info(&mut self.before, self.metas);
        self.after_acc_inf = Self::calculate_account_info(&mut self.after, self.metas);

        let pre_ix = self
            .ix
            .deserialize_option(program_id, &mut self.before_acc_inf)
            .map_err(|e| e.with_context(Context::Pre))?;
        let post_ix = self
            .ix
            .deserialize_option(program_id, &mut self.after_acc_inf)
            .map_err(|e| e.with_context(Context::Post))?;
        Ok((pre_ix, post_ix))
    }
}
