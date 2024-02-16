use trdelnik_client::anchor_lang::solana_program::instruction::AccountMeta;
use trdelnik_client::anchor_lang::{self, prelude::*};
use trdelnik_client::fuzzing::{get_account_infos_option, FuzzingError};
pub struct InitVestingSnapshot<'info> {
    pub sender: Signer<'info>,
    pub sender_token_account: Account<'info, TokenAccount>,
    pub escrow: Option<Account<'info, Escrow>>,
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
pub struct WithdrawUnlockedSnapshot<'info> {
    pub recipient: Signer<'info>,
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub escrow: Option<Account<'info, Escrow>>,
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub escrow_pda_authority: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitVestingSnapshot<'info> {
    pub fn deserialize_option(
        metas: &'info [AccountMeta],
        accounts: &'info mut [Option<trdelnik_client::solana_sdk::account::Account>],
    ) -> core::result::Result<Self, FuzzingError> {
        let accounts = get_account_infos_option(accounts, metas)
            .map_err(|_| FuzzingError::NotAbleToObtainAccountInfos)?;
        let mut accounts_iter = accounts.into_iter();
        let sender: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("sender".to_string()))?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .ok_or(FuzzingError::AccountNotFound("sender".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("sender".to_string()))?;
        let sender_token_account: anchor_lang::accounts::account::Account<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "sender_token_account".to_string(),
                ))?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .ok_or(FuzzingError::AccountNotFound(
                    "sender_token_account".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("sender_token_account".to_string())
                })?;
        let escrow: Option<anchor_lang::accounts::account::Account<Escrow>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("escrow".to_string()))?
            .map(|acc| {
                if acc.key() != PROGRAM_ID {
                    anchor_lang::accounts::account::Account::try_from(&acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("escrow".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "escrow".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let escrow_token_account: anchor_lang::accounts::account::Account<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "escrow_token_account".to_string(),
                ))?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .ok_or(FuzzingError::AccountNotFound(
                    "escrow_token_account".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("escrow_token_account".to_string())
                })?;
        let mint: anchor_lang::accounts::account::Account<Mint> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("mint".to_string()))?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .ok_or(FuzzingError::AccountNotFound("mint".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("mint".to_string()))?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            sender,
            sender_token_account,
            escrow,
            escrow_token_account,
            mint,
            token_program,
            system_program,
        })
    }
}
impl<'info> WithdrawUnlockedSnapshot<'info> {
    pub fn deserialize_option(
        metas: &'info [AccountMeta],
        accounts: &'info mut [Option<trdelnik_client::solana_sdk::account::Account>],
    ) -> core::result::Result<Self, FuzzingError> {
        let accounts = get_account_infos_option(accounts, metas)
            .map_err(|_| FuzzingError::NotAbleToObtainAccountInfos)?;
        let mut accounts_iter = accounts.into_iter();
        let recipient: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("recipient".to_string()))?
            .map(|acc| anchor_lang::accounts::signer::Signer::try_from(&acc))
            .ok_or(FuzzingError::AccountNotFound("recipient".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("recipient".to_string()))?;
        let recipient_token_account: anchor_lang::accounts::account::Account<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "recipient_token_account".to_string(),
                ))?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .ok_or(FuzzingError::AccountNotFound(
                    "recipient_token_account".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("recipient_token_account".to_string())
                })?;
        let escrow: Option<anchor_lang::accounts::account::Account<Escrow>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("escrow".to_string()))?
            .map(|acc| {
                if acc.key() != PROGRAM_ID {
                    anchor_lang::accounts::account::Account::try_from(&acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("escrow".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "escrow".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let escrow_token_account: anchor_lang::accounts::account::Account<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "escrow_token_account".to_string(),
                ))?
                .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
                .ok_or(FuzzingError::AccountNotFound(
                    "escrow_token_account".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("escrow_token_account".to_string())
                })?;
        let escrow_pda_authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "escrow_pda_authority".to_string(),
            ))?
            .ok_or(FuzzingError::AccountNotFound(
                "escrow_pda_authority".to_string(),
            ))?;
        let mint: anchor_lang::accounts::account::Account<Mint> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("mint".to_string()))?
            .map(|acc| anchor_lang::accounts::account::Account::try_from(&acc))
            .ok_or(FuzzingError::AccountNotFound("mint".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("mint".to_string()))?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .map(|acc| anchor_lang::accounts::program::Program::try_from(&acc))
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            recipient,
            recipient_token_account,
            escrow,
            escrow_token_account,
            escrow_pda_authority,
            mint,
            token_program,
            system_program,
        })
    }
}
