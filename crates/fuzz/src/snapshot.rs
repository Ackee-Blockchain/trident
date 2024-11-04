#![allow(dead_code)] // The Snapshot is constructed in the FuzzTestExecutor macro and is generated automatically

use solana_sdk::account::{AccountSharedData, ReadableAccount};
use solana_sdk::clock::Epoch;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::fuzz_client::FuzzClient;

use crate::error::*;

pub struct SnapshotAccount {
    address: Pubkey,
    account: AccountSharedData,
}

impl SnapshotAccount {
    pub fn get_account(&self) -> &AccountSharedData {
        &self.account
    }
    pub fn pubkey(&self) -> Pubkey {
        self.address
    }
    pub fn data(&self) -> &[u8] {
        self.account.data()
    }
    pub fn lamports(&self) -> u64 {
        self.account.lamports()
    }
    pub fn owner(&self) -> &Pubkey {
        self.account.owner()
    }
    pub fn executable(&self) -> bool {
        self.account.executable()
    }
    pub fn rent_epoch(&self) -> Epoch {
        self.account.rent_epoch()
    }
}
pub struct Snapshot {
    before: Vec<SnapshotAccount>,
    after: Vec<SnapshotAccount>,
    metas: Vec<AccountMeta>,
}

impl Snapshot {
    pub fn new(metas: &[AccountMeta]) -> Snapshot {
        Self {
            before: Default::default(),
            after: Default::default(),
            metas: metas.to_vec(),
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
    ) -> Result<Vec<SnapshotAccount>, FuzzClientErrorWithOrigin> {
        let accounts = client.get_accounts(&self.metas)?;
        let snapshot_accounts = accounts
            .into_iter()
            .zip(self.metas.iter())
            .map(|(account, meta)| SnapshotAccount {
                address: meta.pubkey,
                account,
            })
            .collect();

        Ok(snapshot_accounts)
    }

    pub fn get_before(&self) -> &[SnapshotAccount] {
        &self.before
    }
    pub fn get_after(&self) -> &[SnapshotAccount] {
        &self.after
    }

    pub fn get_snapshot(&self) -> (&[SnapshotAccount], &[SnapshotAccount]) {
        (self.get_before(), self.get_after())
    }
}
