use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

use crate::trident::client::TransactionResult;
use crate::trident::Trident;

impl Trident {
    pub fn initialize_mint(
        &mut self,
        mint_address: &Pubkey,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> TransactionResult {
        let mut create_account_instructions = self.create_account(
            mint_address,
            owner,
            spl_token_interface::state::Mint::LEN,
            &spl_token_interface::ID,
        );

        let ix_2 = spl_token_interface::instruction::initialize_mint2(
            &spl_token_interface::ID,
            mint_address,
            owner,
            freeze_authority,
            decimals,
        )
        .unwrap();
        create_account_instructions.push(ix_2);

        self.process_transaction(&create_account_instructions, "Creating Mint Account")
    }

    pub fn initialize_token_account(
        &mut self,
        token_account_address: &Pubkey,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> TransactionResult {
        let mut create_account_instructions = self.create_account(
            token_account_address,
            owner,
            spl_token_interface::state::Account::LEN,
            &spl_token_interface::ID,
        );
        let ix = spl_token_interface::instruction::initialize_account3(
            &spl_token_interface::ID,
            token_account_address,
            mint,
            owner,
        )
        .unwrap();

        create_account_instructions.push(ix);

        self.process_transaction(&create_account_instructions, "Creating Token Account")
    }

    pub fn mint_to(
        &mut self,
        token_account_address: &Pubkey,
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        amount: u64,
    ) -> TransactionResult {
        let ix = spl_token_interface::instruction::mint_to(
            &spl_token_interface::ID,
            mint_address,
            token_account_address,
            mint_authority,
            &[],
            amount,
        )
        .unwrap();

        self.process_transaction(&[ix], "Minting to Token Account")
    }
    pub fn initialize_associated_token_account(
        &mut self,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> TransactionResult {
        let ix =
            spl_associated_token_account_interface::instruction::create_associated_token_account(
                owner,
                owner,
                mint,
                &spl_token_interface::ID,
            );

        self.process_transaction(&[ix], "Creating Associated Token Account")
    }
    pub fn get_associated_token_address(
        &self,
        mint: &Pubkey,
        owner: &Pubkey,
        program_id: &Pubkey,
    ) -> Pubkey {
        spl_associated_token_account_interface::address::get_associated_token_address_with_program_id(
            owner, mint, program_id,
        )
    }
}
