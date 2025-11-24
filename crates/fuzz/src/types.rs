use solana_sdk::account::AccountSharedData;
use solana_sdk::pubkey::Pubkey;

pub struct TridentAccountSharedData {
    pub address: Pubkey,
    pub account: AccountSharedData,
}
impl TridentAccountSharedData {
    pub fn new(address: Pubkey, account: AccountSharedData) -> TridentAccountSharedData {
        Self { address, account }
    }
}


pub type AccountId = u8;
pub type FuzzerData<'a> = arbitrary::Unstructured<'a>;
