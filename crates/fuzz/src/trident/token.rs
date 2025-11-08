use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

use crate::trident::transaction_result::TransactionResult;
use crate::trident::Trident;

impl Trident {
    /// Initializes a new SPL Token mint
    ///
    /// Creates and initializes a new token mint with the specified parameters.
    /// This involves creating the account and then initializing it as a mint.
    ///
    /// # Arguments
    /// * `mint_address` - The public key where the mint will be created
    /// * `decimals` - Number of decimal places for the token (0-9)
    /// * `owner` - The mint authority who can mint new tokens
    /// * `freeze_authority` - Optional authority who can freeze token accounts
    ///
    /// # Returns
    /// A `TransactionResult` indicating success or failure of the mint creation
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

    /// Initializes a new SPL Token account
    ///
    /// Creates and initializes a new token account that can hold tokens
    /// of the specified mint type.
    ///
    /// # Arguments
    /// * `token_account_address` - The public key where the token account will be created
    /// * `mint` - The mint that this account will hold tokens for
    /// * `owner` - The owner of the token account
    ///
    /// # Returns
    /// A `TransactionResult` indicating success or failure of the account creation
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

    /// Mints tokens to a specified token account
    ///
    /// Creates new tokens and adds them to the specified token account.
    /// The mint authority must sign this transaction.
    ///
    /// # Arguments
    /// * `token_account_address` - The token account to mint tokens to
    /// * `mint_address` - The mint to create tokens from
    /// * `mint_authority` - The authority allowed to mint tokens
    /// * `amount` - The number of tokens to mint (in base units)
    ///
    /// # Returns
    /// A `TransactionResult` indicating success or failure of the mint operation
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
    /// Creates an Associated Token Account (ATA)
    ///
    /// Creates an associated token account for the given mint and owner.
    /// The ATA address is deterministically derived from the mint and owner.
    ///
    /// # Arguments
    /// * `mint` - The mint that the ATA will hold tokens for
    /// * `owner` - The owner of the ATA
    ///
    /// # Returns
    /// A `TransactionResult` indicating success or failure of the ATA creation
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
    /// Gets the Associated Token Account address for a mint and owner
    ///
    /// Calculates the deterministic address of an Associated Token Account
    /// without creating it. This is useful for finding existing ATAs.
    ///
    /// # Arguments
    /// * `mint` - The mint public key
    /// * `owner` - The owner public key
    /// * `program_id` - The token program ID (usually spl_token::ID)
    ///
    /// # Returns
    /// The public key of the associated token account
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
