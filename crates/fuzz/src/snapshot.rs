#![allow(dead_code)] // The Snapshot is constructed in the FuzzTestExecutor macro and is generated automatically

use solana_sdk::account::{AccountSharedData, ReadableAccount};
use solana_sdk::clock::Epoch;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::fuzz_client::FuzzClient;

use crate::error::*;

/// A struct that represents an account in the snapshot.
/// It contains the address of the account and the account data (AccountSharedData).
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
    pub fn data_no_discriminator(&self) -> &[u8] {
        if self.account.data().len() > 8 {
            &self.account.data()[8..]
        } else {
            panic!("Account does not contain more than 8 bytes")
        }
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

/// A struct that represents the snapshot of a transaction.
/// It contains the accounts and the data of each instruction in the transaction.
#[derive(Default)]
pub struct TransactionSnapshot {
    accounts: Vec<Vec<SnapshotAccount>>,
    data: Vec<Vec<u8>>,
}

impl TransactionSnapshot {
    fn with_data(data: Vec<Vec<u8>>) -> Self {
        Self {
            accounts: vec![],
            data,
        }
    }
    fn add_accounts(&mut self, accounts: Vec<Vec<SnapshotAccount>>) {
        self.accounts = accounts;
    }
    pub fn get_accounts_at(&self, instruction_index: usize) -> &[SnapshotAccount] {
        if self.accounts.len() > instruction_index {
            &self.accounts[instruction_index]
        } else {
            panic!(
                "Transaction snapshot does not contain that many instructions {}",
                instruction_index
            );
        }
    }
    pub fn get_data_at(&self, instruction_index: usize) -> &[u8] {
        if self.data.len() > instruction_index {
            &self.data[instruction_index]
        } else {
            panic!(
                "Transaction snapshot does not contain that many instructions {}",
                instruction_index
            );
        }
    }
}

/// A struct that represents the snapshot of transaction before and after execution.
/// It is used internally by the fuzzer to capture the accounts and data of the transaction before and after execution.
pub(crate) struct Snapshot {
    before: TransactionSnapshot,
    after: TransactionSnapshot,
    metas: Vec<Vec<AccountMeta>>,
}

impl Snapshot {
    pub fn new(metas: &[Vec<AccountMeta>], data: Vec<Vec<u8>>) -> Snapshot {
        Self {
            before: TransactionSnapshot::with_data(data.clone()),
            after: TransactionSnapshot::with_data(data),
            metas: metas.to_vec(),
        }
    }
    pub fn capture_before(
        &mut self,
        client: &mut impl FuzzClient,
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        let accounts = self
            .capture(client)
            .map_err(|e| e.with_context(Context::Pre))?;

        self.before.add_accounts(accounts);

        Ok(())
    }

    pub fn capture_after(
        &mut self,
        client: &mut impl FuzzClient,
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        let accounts = self
            .capture(client)
            .map_err(|e| e.with_context(Context::Post))?;

        self.after.add_accounts(accounts);

        Ok(())
    }

    fn capture(
        &mut self,
        client: &mut impl FuzzClient,
    ) -> Result<Vec<Vec<SnapshotAccount>>, FuzzClientErrorWithOrigin> {
        let snapshot_accounts = self
            .metas
            .iter()
            .map(|instruction_metas| {
                instruction_metas
                    .iter()
                    .map(|meta| {
                        let account = client.get_account(&meta.pubkey);
                        SnapshotAccount {
                            address: meta.pubkey,
                            account,
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(snapshot_accounts)
    }

    pub fn get_before(&self) -> &TransactionSnapshot {
        &self.before
    }
    pub fn get_after(&self) -> &TransactionSnapshot {
        &self.after
    }

    pub fn get_snapshot(&self) -> (&TransactionSnapshot, &TransactionSnapshot) {
        (self.get_before(), self.get_after())
    }
}
