use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

use crate::trident::Trident;

impl Trident {
    // #[cfg(feature = "token")]
    pub fn create_mint(
        &mut self,
        mint_address: &Pubkey,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> solana_sdk::transaction::Result<()> {
        let mut create_account_instructions = self.create_account(
            mint_address,
            owner,
            spl_token_interface::state::Mint::LEN,
            &spl_token::ID,
        );

        let ix_2 = spl_token::instruction::initialize_mint2(
            &spl_token::ID,
            mint_address,
            owner,
            freeze_authority,
            decimals,
        )
        .unwrap();
        create_account_instructions.push(ix_2);

        self.execute(&create_account_instructions, "Creating Mint Account")
    }

    // #[cfg(feature = "token")]
    pub fn create_token_account(
        &mut self,
        token_account_address: &Pubkey,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> solana_sdk::transaction::Result<()> {
        let mut create_account_instructions = self.create_account(
            token_account_address,
            owner,
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
        let ix = spl_token::instruction::initialize_account3(
            &spl_token::ID,
            token_account_address,
            mint,
            owner,
        )
        .unwrap();

        create_account_instructions.push(ix);

        self.execute(&create_account_instructions, "Creating Token Account")
    }

    // #[cfg(feature = "token")]
    pub fn mint_to(
        &mut self,
        token_account_address: &Pubkey,
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        amount: u64,
    ) -> solana_sdk::transaction::Result<()> {
        let ix = spl_token::instruction::mint_to(
            &spl_token::ID,
            mint_address,
            token_account_address,
            mint_authority,
            &[],
            amount,
        )
        .unwrap();

        self.execute(&[ix], "Minting to Token Account")
    }
}
