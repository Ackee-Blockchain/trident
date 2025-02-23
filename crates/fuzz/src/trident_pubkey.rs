use arbitrary::Arbitrary;
use arbitrary::Result;
use arbitrary::Unstructured;

use solana_sdk::pubkey::Pubkey;

use crate::types::AccountId;

#[derive(Debug, Clone, Copy)]
pub struct TridentPubkey {
    pub account_id: AccountId,
    pubkey: Pubkey,
}

impl TridentPubkey {
    pub fn set_pubkey(&mut self, pubkey: Pubkey) {
        self.pubkey = pubkey;
    }
    pub fn get_pubkey(&self) -> Pubkey {
        self.pubkey
    }
}

impl<'a> Arbitrary<'a> for TridentPubkey {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let mut buf = [0; std::mem::size_of::<AccountId>()];
        u.fill_buffer(&mut buf)?;
        Ok(Self {
            account_id: AccountId::from_le_bytes(buf),
            pubkey: Pubkey::default(),
        })
    }
    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        let n = std::mem::size_of::<AccountId>();
        (n, Some(n))
    }
}

impl From<AccountId> for TridentPubkey {
    fn from(account_id: AccountId) -> Self {
        Self {
            account_id,
            pubkey: Pubkey::default(),
        }
    }
}
impl borsh::BorshSerialize for TridentPubkey {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.pubkey.serialize(writer)
    }
}

impl borsh::BorshDeserialize for TridentPubkey {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let pubkey = Pubkey::deserialize_reader(reader)?;
        Ok(Self {
            account_id: AccountId::default(),
            pubkey,
        })
    }
}

impl serde::Serialize for TridentPubkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.pubkey.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for TridentPubkey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let pubkey = Pubkey::deserialize(deserializer)?;
        Ok(Self {
            account_id: AccountId::default(),
            pubkey,
        })
    }
}
