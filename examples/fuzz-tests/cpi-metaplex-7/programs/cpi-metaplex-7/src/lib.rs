use anchor_lang::prelude::*;
use anchor_spl::{token::Mint, token_interface::TokenInterface};
use mpl_token_metadata::{
    instructions::{
        CreateMetadataAccountV3Cpi, CreateMetadataAccountV3CpiAccounts,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::DataV2,
    ID as MPL_METADATA_PROGRAM,
};

use trident_derive_accounts_snapshots::AccountsSnapshots;

declare_id!("3XtULmXDGS867VbBXiPkjYr4EMjytGW8X12F6BS23Zcw");

#[program]
pub mod cpi_metaplex_7 {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        input: u8,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.create_metadata(name, symbol, uri)?;

        if input > 15 {
            panic!("This number is magic");
        }

        Ok(())
    }
}

#[derive(AccountsSnapshots, Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = signer,
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: Will be initialized
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    pub mpl_token_metadata: Program<'info, MplTokenMetadataProgram>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Debug, Clone)]
pub struct MplTokenMetadataProgram;

impl anchor_lang::Id for MplTokenMetadataProgram {
    fn id() -> Pubkey {
        MPL_METADATA_PROGRAM
    }
}

impl<'info> Initialize<'info> {
    pub fn create_metadata(&self, name: String, symbol: String, uri: String) -> Result<()> {
        let mpl_metadata_program = &self.mpl_token_metadata.to_account_info();
        let metadata = &self.metadata_account.to_account_info();
        let mint = &self.mint.to_account_info();
        let mint_authority = &self.signer.to_account_info();
        let payer = &self.signer.to_account_info();
        let system_program = &self.system_program.to_account_info();

        let cpi_context = CreateMetadataAccountV3Cpi::new(
            mpl_metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority,
                payer,
                update_authority: (system_program, false), // second value sets if the account is also signer
                system_program,
                rent: None,
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name,
                    symbol,
                    uri,
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                },
                is_mutable: false,
                collection_details: None,
            },
        );

        cpi_context.invoke()?;

        Ok(())
    }
}
