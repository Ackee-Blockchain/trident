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

    pub fn get_snapshot(&'info mut self) -> Result<(T::Ix, T::Ix), FuzzingError> {
        let pre_ix = self.ix.deserialize_option(self.metas, &mut self.before)?;
        let post_ix = self.ix.deserialize_option(self.metas, &mut self.after)?;
        Ok((pre_ix, post_ix))
    }
}
