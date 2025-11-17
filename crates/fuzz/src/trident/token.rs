use solana_sdk::account::ReadableAccount;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_error::ProgramError;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

use crate::trident::token2022::MintWithExtensions;
use crate::trident::token2022::TokenAccountWithExtensions;
use crate::trident::Trident;

impl Trident {
    /// Creates instructions to initialize a new SPL Token mint
    ///
    /// Generates instructions to create and initialize a new token mint with the specified parameters.
    /// This involves creating the account and then initializing it as a mint.
    ///
    /// # Arguments
    /// * `payer` - The payer covering the rent
    /// * `mint_address` - The public key where the mint will be created
    /// * `decimals` - Number of decimal places for the token (0-9)
    /// * `owner` - The mint authority who can mint new tokens
    /// * `freeze_authority` - Optional authority who can freeze token accounts
    ///
    /// # Returns
    /// A vector of instructions that need to be executed with `process_transaction`
    pub fn initialize_mint(
        &mut self,
        payer: &Pubkey,
        mint_address: &Pubkey,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> Vec<Instruction> {
        let mut create_account_instructions = self.create_account_internal(
            mint_address,
            payer,
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

        create_account_instructions
    }

    /// Creates instructions to initialize a new SPL Token account
    ///
    /// Generates instructions to create and initialize a new token account that can hold tokens
    /// of the specified mint type.
    ///
    /// # Arguments
    /// * `payer` - The payer covering the rent
    /// * `token_account_address` - The public key where the token account will be created
    /// * `mint` - The mint that this account will hold tokens for
    /// * `owner` - The owner of the token account
    ///
    /// # Returns
    /// A vector of instructions that need to be executed with `process_transaction`
    pub fn initialize_token_account(
        &mut self,
        payer: &Pubkey,
        token_account_address: &Pubkey,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> Vec<Instruction> {
        let mut create_account_instructions = self.create_account_internal(
            token_account_address,
            payer,
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

        create_account_instructions
    }

    /// Creates an instruction to mint tokens to a specified token account
    ///
    /// Generates an instruction to create new tokens and add them to the specified token account.
    /// The mint authority must sign this transaction.
    ///
    /// # Arguments
    /// * `token_account_address` - The token account to mint tokens to
    /// * `mint_address` - The mint to create tokens from
    /// * `mint_authority` - The authority allowed to mint tokens
    /// * `amount` - The number of tokens to mint (in base units)
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn mint_to(
        &mut self,
        token_account_address: &Pubkey,
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        amount: u64,
    ) -> Instruction {
        spl_token_interface::instruction::mint_to(
            &spl_token_interface::ID,
            mint_address,
            token_account_address,
            mint_authority,
            &[],
            amount,
        )
        .unwrap()
    }
    /// Creates an instruction to initialize an Associated Token Account (ATA)
    ///
    /// Generates an instruction to create an associated token account for the given mint and owner.
    /// The ATA address is deterministically derived from the mint and owner.
    ///
    /// # Arguments
    /// * `payer` - The payer covering the rent
    /// * `mint` - The mint that the ATA will hold tokens for
    /// * `owner` - The owner of the ATA
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn initialize_associated_token_account(
        &mut self,
        payer: &Pubkey,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> Instruction {
        spl_associated_token_account_interface::instruction::create_associated_token_account(
            payer,
            owner,
            mint,
            &spl_token_interface::ID,
        )
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

    /// Deserializes a token account (SPL Token or Token 2022) with all its extensions
    ///
    /// Works with both SPL Token and Token 2022 accounts. For Token 2022 accounts,
    /// all extensions are deserialized and included in the result.
    ///
    /// # Arguments
    ///
    /// * `account` - The public key of the token account to deserialize
    ///
    /// # Returns
    ///
    /// Returns a `TokenAccountWithExtensions` containing the account data and all extensions,
    /// or an error if deserialization fails or the account is not owned by a token program.
    pub fn get_token_account(
        &mut self,
        account: Pubkey,
    ) -> Result<TokenAccountWithExtensions, ProgramError> {
        let account_data = self.get_account(&account);
        if account_data.owner() != &spl_token_2022_interface::ID
            && account_data.owner() != &spl_token_interface::ID
        {
            Err(ProgramError::InvalidAccountOwner)
        } else {
            self.get_token_account_2022(account_data.data())
        }
    }

    /// Deserializes a mint account (SPL Token or Token 2022) with all its extensions
    ///
    /// Works with both SPL Token and Token 2022 mints. For Token 2022 mints,
    /// all extensions are deserialized and included in the result.
    ///
    /// # Arguments
    ///
    /// * `account` - The public key of the mint account to deserialize
    ///
    /// # Returns
    ///
    /// Returns a `MintWithExtensions` containing the mint data and all extensions,
    /// or an error if deserialization fails or the account is not owned by a token program.
    pub fn get_mint(&mut self, account: Pubkey) -> Result<MintWithExtensions, ProgramError> {
        let account_data = self.get_account(&account);
        if account_data.owner() != &spl_token_2022_interface::ID
            && account_data.owner() != &spl_token_interface::ID
        {
            Err(ProgramError::InvalidAccountOwner)
        } else {
            self.get_mint_2022(account_data.data())
        }
    }
}
