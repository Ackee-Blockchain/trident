use anchor_lang::{prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint,Token2022, TokenAccount, 
    },
};
use spl_token_2022::extension::non_transferable::NonTransferable;

pub fn handler(
    ctx: Context<CreateMintAccount>,
    //supply_elgamal_pubkey: solana_zk_sdk::encryption::pod::elgamal::PodElGamalPubkey,
    //decryptable_supply: solana_zk_sdk::encryption::pod::auth_encryption::PodAeCiphertext,
    //confidential_supply: solana_zk_sdk::encryption::pod::elgamal::PodElGamalCiphertext,
) -> Result<()> {
    msg!("CreateMintAccount handler called");
    eprintln!("CreateMintAccount handler called");

    
    Ok(())
}


#[derive(Accounts)]
#[instruction(
    //supply_elgamal_pubkey: solana_zk_sdk::encryption::pod::elgamal::PodElGamalPubkey,
    //decryptable_supply: solana_zk_sdk::encryption::pod::auth_encryption::PodAeCiphertext,
    //confidential_supply: solana_zk_sdk::encryption::pod::elgamal::PodElGamalCiphertext
)]
pub struct CreateMintAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    /// CHECK: can be any account
    pub authority: Signer<'info>,
    #[account()]
    /// CHECK: can be any account
    pub receiver: UncheckedAccount<'info>,
    #[account(
        //init_if_needed,
        //signer,
        //payer = payer,
        mint::token_program = token_program,
        mint::decimals = 9,
        mint::authority = authority,
        mint::freeze_authority = authority,

        extensions::metadata_pointer::authority = authority,
        //extensions::metadata_pointer::metadata_address = receiver,
        
        extensions::group_member_pointer::authority = authority,
        //extensions::group_member_pointer::member_address = payer,
        extensions::transfer_hook::authority = authority,
        //extensions::transfer_hook::program_id = crate::ID,
        //extensions::close_authority::authority = authority,
        //extensions::permanent_delegate::delegate = authority,
        extensions::group_pointer::authority = authority,
        //extensions::group_pointer::group_address = receiver,

        
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        //init,
        //payer = payer,
        token::mint = mint,
        token::authority = receiver,
        token::token_program = token_program,

        
        //extensions::transfer_hook::transferring = true,
        //extensions::transfer_hook::program_id = crate::ID,
    
    )]
    pub mint_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: This account is used to store extra metadata for the mint
    pub extra_metas_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

