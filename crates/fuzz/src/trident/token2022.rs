/// Placeholder for future support of token 2022
use crate::trident::Trident;
use solana_sdk::pubkey::Pubkey;

impl Trident {
    pub fn create_mint_2022(
        &mut self,
        mint_address: &Pubkey,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> solana_sdk::transaction::Result<()> {
        todo!()
    }
}
