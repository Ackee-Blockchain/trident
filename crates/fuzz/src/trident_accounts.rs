use arbitrary::Arbitrary;
use arbitrary::Result;
use arbitrary::Unstructured;

use solana_sdk::account::AccountSharedData;
use solana_sdk::account::ReadableAccount;
use solana_sdk::clock::Epoch;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::{fuzzing::FuzzClient, types::AccountId};

#[derive(Clone)]
pub struct TridentAccount {
    pub account_id: AccountId,
    account_meta: Option<AccountMeta>,
    snapshot_before: Option<SnapshotAccount>,
    snapshot_after: Option<SnapshotAccount>,
}

impl<'a> Arbitrary<'a> for TridentAccount {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let mut buf = [0; std::mem::size_of::<AccountId>()];
        u.fill_buffer(&mut buf)?;
        Ok(Self {
            account_id: AccountId::from_le_bytes(buf),
            account_meta: None,
            snapshot_before: None,
            snapshot_after: None,
        })
    }
    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        let n = std::mem::size_of::<AccountId>();
        (n, Some(n))
    }
}

impl TridentAccount {
    pub fn set_account_meta(&mut self, address: Pubkey, is_signer: bool, is_writable: bool) {
        if is_writable {
            self.account_meta = Some(AccountMeta::new(address, is_signer));
        } else {
            self.account_meta = Some(AccountMeta::new_readonly(address, is_signer));
        }
    }
    #[doc(hidden)]
    pub fn capture_before(&mut self, client: &mut impl FuzzClient) {
        if let Some(account_meta) = &self.account_meta {
            let account = client.get_account(&account_meta.pubkey);
            self.snapshot_before = Some(SnapshotAccount {
                address: account_meta.pubkey,
                account,
            });
        }
    }
    #[doc(hidden)]
    pub fn capture_after(&mut self, client: &mut impl FuzzClient) {
        if let Some(account_meta) = &self.account_meta {
            let account = client.get_account(&account_meta.pubkey);
            self.snapshot_after = Some(SnapshotAccount {
                address: account_meta.pubkey,
                account,
            });
        }
    }
    #[doc(hidden)]
    pub fn to_account_meta(&self) -> AccountMeta {
        match &self.account_meta {
            Some(account_meta) => account_meta.clone(),
            None => panic!("Account meta is not set"),
        }
    }
    #[doc(hidden)]
    pub fn is_account_meta_set(&self) -> bool {
        self.account_meta.is_some()
    }
    pub fn get_snapshot_before(&self) -> &SnapshotAccount {
        match &self.snapshot_before {
            Some(snapshot) => snapshot,
            None => panic!("Snapshot before is not set"),
        }
    }
    pub fn get_snapshot_after(&self) -> &SnapshotAccount {
        match &self.snapshot_after {
            Some(snapshot) => snapshot,
            None => panic!("Snapshot after is not set"),
        }
    }
    pub fn set_is_signer(&mut self) {
        match &mut self.account_meta {
            Some(account_meta) => account_meta.is_signer = true,
            None => self.account_meta = Some(AccountMeta::new_readonly(Pubkey::default(), true)),
        }
    }
    pub fn set_is_writable(&mut self) {
        match &mut self.account_meta {
            Some(account_meta) => account_meta.is_writable = true,
            None => self.account_meta = Some(AccountMeta::new(Pubkey::default(), false)),
        }
    }
    pub fn set_address(&mut self, address: Pubkey) {
        match &mut self.account_meta {
            Some(account_meta) => account_meta.pubkey = address,
            None => self.account_meta = Some(AccountMeta::new_readonly(address, false)),
        }
    }
    pub fn pubkey(&self) -> Pubkey {
        match &self.account_meta {
            Some(account_meta) => account_meta.pubkey,
            None => panic!("Account meta is not set"),
        }
    }
}

impl std::fmt::Debug for TridentAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TridentAccount {{ account_id: {}, account_meta: ",
            self.account_id
        )?;
        match &self.account_meta {
            Some(meta) => {
                write!(f, "{{ ")?;
                write!(f, "address: \x1b[93m{}\x1b[0m, ", meta.pubkey)?;
                write!(f, "is_signer: \x1b[33m{}\x1b[0m, ", meta.is_signer)?;
                write!(f, "is_writable: \x1b[94m{}\x1b[0m", meta.is_writable)?;
                write!(f, " }}")?;
            }
            None => write!(f, "none")?,
        }
        write!(f, " }}")
    }
}

/// A struct that represents an account in the snapshot.
/// It contains the address of the account and the account data (AccountSharedData).
#[derive(Debug, Clone, Default)]
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
