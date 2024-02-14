#![allow(dead_code)] // The Snapshot is constructed in the FuzzTestExecutor macro and is generated automatically

use solana_sdk::{account::Account, instruction::AccountMeta};

use crate::data_builder::{FuzzClient, FuzzClientError, FuzzDeserialize, FuzzingError};

pub struct Snapshot<'info, T> {
    before: Vec<Option<Account>>,
    after: Vec<Option<Account>>,
    metas: &'info [AccountMeta],
    ix: &'info T,
}

impl<'info, T> Snapshot<'info, T>
where
    T: FuzzDeserialize<'info>,
{
    pub fn new(metas: &'info [AccountMeta], ix: &'info T) -> Snapshot<'info, T> {
        Self {
            before: Vec::new(),
            after: Vec::new(),
            metas,
            ix,
        }
    }

    pub fn capture_before(&mut self, client: &mut impl FuzzClient) -> Result<(), FuzzClientError> {
        self.before = self.capture(client)?;
        Ok(())
    }

    pub fn capture_after(&mut self, client: &mut impl FuzzClient) -> Result<(), FuzzClientError> {
        self.after = self.capture(client)?;
        Ok(())
    }

    fn capture(
        &mut self,
        client: &mut impl FuzzClient,
    ) -> Result<Vec<Option<Account>>, FuzzClientError> {
        let accounts;
        {
            accounts = client.get_accounts(self.metas)?;
        }
        Ok(accounts)
    }

    fn set_missing_accounts_to_default(accounts: &mut [Option<Account>]) {
        for acc in accounts.iter_mut() {
            if acc.is_none() {
                *acc = Some(solana_sdk::account::Account::default());
            }
        }
    }

    pub fn get_snapshot(&'info mut self) -> Result<(T::Ix, T::Ix), FuzzingError> {
        // When user passes an account that is not initialized, the runtime will provide
        // a default empty account to the program. If the uninitialized account is of type
        // AccountInfo, Signer or UncheckedAccount, Anchor will not return an error. However
        // when we try to fetch "on-chain" accounts and an account is not initilized, this
        // account simply does not exist and the get_account() method returns None. To prevent
        // errors during deserialization due to missing accounts, we replace the missing accounts
        // with default values similar as the runtime does.
        Self::set_missing_accounts_to_default(&mut self.before);
        Self::set_missing_accounts_to_default(&mut self.after);

        let pre_ix = self.ix.deserialize_option(self.metas, &mut self.before)?;
        let post_ix = self.ix.deserialize_option(self.metas, &mut self.after)?;
        Ok((pre_ix, post_ix))
    }
}
