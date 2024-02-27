#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;

mod error {

    use anchor_lang::prelude::*;
    #[repr(u32)]
    pub enum VestingError {
        InvalidAmount,
        InvalidTimeRange,
        InvalidInterval,
        Overflow,
        Underflow,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for VestingError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f,
                match self {
                    VestingError::InvalidAmount => "InvalidAmount",
                    VestingError::InvalidTimeRange => "InvalidTimeRange",
                    VestingError::InvalidInterval => "InvalidInterval",
                    VestingError::Overflow => "Overflow",
                    VestingError::Underflow => "Underflow",
                })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for VestingError {
        #[inline]
        fn clone(&self) -> VestingError { *self }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for VestingError { }
    impl VestingError {
        #[doc = r" Gets the name of this [#enum_name]."]
        pub fn name(&self) -> String {
            match self {
                VestingError::InvalidAmount => "InvalidAmount".to_string(),
                VestingError::InvalidTimeRange =>
                    "InvalidTimeRange".to_string(),
                VestingError::InvalidInterval =>
                    "InvalidInterval".to_string(),
                VestingError::Overflow => "Overflow".to_string(),
                VestingError::Underflow => "Underflow".to_string(),
            }
        }
    }
    impl From<VestingError> for u32 {
        fn from(e: VestingError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<VestingError> for anchor_lang::error::Error {
        fn from(error_code: VestingError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: error_code.name(),
                    error_code_number: error_code.into(),
                    error_msg: error_code.to_string(),
                    error_origin: None,
                    compared_values: None,
                })
        }
    }
    impl std::fmt::Display for VestingError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>)
            -> std::result::Result<(), std::fmt::Error> {
            match self {
                VestingError::InvalidAmount =>
                    <Self as std::fmt::Debug>::fmt(self, fmt),
                VestingError::InvalidTimeRange =>
                    <Self as std::fmt::Debug>::fmt(self, fmt),
                VestingError::InvalidInterval =>
                    <Self as std::fmt::Debug>::fmt(self, fmt),
                VestingError::Overflow =>
                    <Self as std::fmt::Debug>::fmt(self, fmt),
                VestingError::Underflow =>
                    <Self as std::fmt::Debug>::fmt(self, fmt),
            }
        }
    }
}
mod instructions {
    pub mod initialize {
        use anchor_lang::prelude::*;
        use anchor_spl::token::{
            set_authority, transfer, Mint, SetAuthority, Token, TokenAccount,
            Transfer,
        };
        use crate::state::Escrow;
        use crate::VestingError;
        pub fn _init_vesting(ctx: Context<InitVesting>, recipient: Pubkey,
            amount: u64, start_at: u64, end_at: u64, interval: u64)
            -> Result<()> {
            let escrow = &mut ctx.accounts.escrow;
            if !(amount > 0) {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                    error_name: VestingError::InvalidAmount.name(),
                                    error_code_number: VestingError::InvalidAmount.into(),
                                    error_msg: VestingError::InvalidAmount.to_string(),
                                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                filename: "programs/fuzz_example3/src/instructions/initialize.rs",
                                                line: 18u32,
                                            })),
                                    compared_values: None,
                                }));
                };
            if !(end_at > start_at) {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                    error_name: VestingError::InvalidTimeRange.name(),
                                    error_code_number: VestingError::InvalidTimeRange.into(),
                                    error_msg: VestingError::InvalidTimeRange.to_string(),
                                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                filename: "programs/fuzz_example3/src/instructions/initialize.rs",
                                                line: 20u32,
                                            })),
                                    compared_values: None,
                                }));
                };
            if !(end_at - start_at > interval) {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                    error_name: VestingError::InvalidInterval.name(),
                                    error_code_number: VestingError::InvalidInterval.into(),
                                    error_msg: VestingError::InvalidInterval.to_string(),
                                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                filename: "programs/fuzz_example3/src/instructions/initialize.rs",
                                                line: 22u32,
                                            })),
                                    compared_values: None,
                                }));
                };
            if !(interval > 0) {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                    error_name: VestingError::InvalidInterval.name(),
                                    error_code_number: VestingError::InvalidInterval.into(),
                                    error_msg: VestingError::InvalidInterval.to_string(),
                                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                filename: "programs/fuzz_example3/src/instructions/initialize.rs",
                                                line: 23u32,
                                            })),
                                    compared_values: None,
                                }));
                };
            escrow.amount = amount;
            escrow.start_time = start_at;
            escrow.end_time = end_at;
            escrow.interval = interval;
            escrow.recipient = recipient;
            escrow.bump = ctx.bumps.escrow;
            let (escrow_pda_authority, _) =
                Pubkey::find_program_address(&[b"ESCROW_PDA_AUTHORITY"],
                    ctx.program_id);
            set_authority(CpiContext::new(ctx.accounts.token_program.to_account_info(),
                        SetAuthority {
                            account_or_mint: ctx.accounts.escrow_token_account.to_account_info(),
                            current_authority: ctx.accounts.sender.to_account_info(),
                        }),
                    anchor_spl::token::spl_token::instruction::AuthorityType::AccountOwner,
                    Some(escrow_pda_authority))?;
            transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(),
                        Transfer {
                            from: ctx.accounts.sender_token_account.to_account_info(),
                            to: ctx.accounts.escrow_token_account.to_account_info(),
                            authority: ctx.accounts.sender.to_account_info(),
                        }), amount)?;
            Ok(())
        }
        #[instruction(recipient: Pubkey)]
        pub struct InitVesting<'info> {
            #[account(mut)]
            pub sender: Signer<'info>,
            #[account(mut, token::authority = sender, token::mint = mint)]
            pub sender_token_account: Account<'info, TokenAccount>,
            #[account(init, payer = sender, space = 8 + 1 + 32 + 5*8, seeds =
            [recipient.as_ref(),b"ESCROW_SEED"], bump)]
            pub escrow: Account<'info, Escrow>,
            #[account(mut, token::mint = mint)]
            pub escrow_token_account: Account<'info, TokenAccount>,
            pub mint: Account<'info, Mint>,
            pub token_program: Program<'info, Token>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, InitVestingBumps> for
            InitVesting<'info> where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8], __bumps: &mut InitVestingBumps,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let mut __ix_data = __ix_data;
                struct __Args {
                    recipient: Pubkey,
                }
                impl borsh::ser::BorshSerialize for __Args where
                    Pubkey: borsh::ser::BorshSerialize {
                    fn serialize<W: borsh::maybestd::io::Write>(&self,
                        writer: &mut W)
                        -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                        borsh::BorshSerialize::serialize(&self.recipient, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args where
                    Pubkey: borsh::BorshDeserialize {
                    fn deserialize_reader<R: borsh::maybestd::io::Read>(reader:
                            &mut R)
                        ->
                            ::core::result::Result<Self, borsh::maybestd::io::Error> {
                        Ok(Self {
                                recipient: borsh::BorshDeserialize::deserialize_reader(reader)?,
                            })
                    }
                }
                let __Args { recipient } =
                    __Args::deserialize(&mut __ix_data).map_err(|_|
                                anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
                let sender: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("sender"))?;
                let sender_token_account:
                        anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("sender_token_account"))?;
                if __accounts.is_empty() {
                        return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
                    }
                let escrow = &__accounts[0];
                *__accounts = &__accounts[1..];
                let escrow_token_account:
                        anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("escrow_token_account"))?;
                let mint: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("mint"))?;
                let token_program:
                        anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("token_program"))?;
                let system_program:
                        anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) =
                    Pubkey::find_program_address(&[recipient.as_ref(),
                                    b"ESCROW_SEED"], __program_id);
                __bumps.escrow = __bump;
                if escrow.key() != __pda_address {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds).with_account_name("escrow").with_pubkeys((escrow.key(),
                                        __pda_address)));
                    }
                let escrow =
                    {
                        let actual_field = AsRef::<AccountInfo>::as_ref(&escrow);
                        let actual_owner = actual_field.owner;
                        let space = 8 + 1 + 32 + 5 * 8;
                        let pa: anchor_lang::accounts::account::Account<Escrow> =
                            if !false ||
                                        actual_owner ==
                                            &anchor_lang::solana_program::system_program::ID {
                                    let __current_lamports = escrow.lamports();
                                    if __current_lamports == 0 {
                                            let space = space;
                                            let lamports = __anchor_rent.minimum_balance(space);
                                            let cpi_accounts =
                                                anchor_lang::system_program::CreateAccount {
                                                    from: sender.to_account_info(),
                                                    to: escrow.to_account_info(),
                                                };
                                            let cpi_context =
                                                anchor_lang::context::CpiContext::new(system_program.to_account_info(),
                                                    cpi_accounts);
                                            anchor_lang::system_program::create_account(cpi_context.with_signer(&[&[recipient.as_ref(),
                                                                                    b"ESCROW_SEED", &[__bump][..]][..]]), lamports,
                                                    space as u64, __program_id)?;
                                        } else {
                                           if sender.key() == escrow.key() {
                                                   return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                                       error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount.name(),
                                                                       error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount.into(),
                                                                       error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount.to_string(),
                                                                       error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                                                   filename: "programs/fuzz_example3/src/instructions/initialize.rs",
                                                                                   line: 64u32,
                                                                               })),
                                                                       compared_values: None,
                                                                   }).with_pubkeys((sender.key(), escrow.key())));
                                               };
                                           let required_lamports =
                                               __anchor_rent.minimum_balance(space).max(1).saturating_sub(__current_lamports);
                                           if required_lamports > 0 {
                                                   let cpi_accounts =
                                                       anchor_lang::system_program::Transfer {
                                                           from: sender.to_account_info(),
                                                           to: escrow.to_account_info(),
                                                       };
                                                   let cpi_context =
                                                       anchor_lang::context::CpiContext::new(system_program.to_account_info(),
                                                           cpi_accounts);
                                                   anchor_lang::system_program::transfer(cpi_context,
                                                           required_lamports)?;
                                               }
                                           let cpi_accounts =
                                               anchor_lang::system_program::Allocate {
                                                   account_to_allocate: escrow.to_account_info(),
                                               };
                                           let cpi_context =
                                               anchor_lang::context::CpiContext::new(system_program.to_account_info(),
                                                   cpi_accounts);
                                           anchor_lang::system_program::allocate(cpi_context.with_signer(&[&[recipient.as_ref(),
                                                                                   b"ESCROW_SEED", &[__bump][..]][..]]), space as u64)?;
                                           let cpi_accounts =
                                               anchor_lang::system_program::Assign {
                                                   account_to_assign: escrow.to_account_info(),
                                               };
                                           let cpi_context =
                                               anchor_lang::context::CpiContext::new(system_program.to_account_info(),
                                                   cpi_accounts);
                                           anchor_lang::system_program::assign(cpi_context.with_signer(&[&[recipient.as_ref(),
                                                                                   b"ESCROW_SEED", &[__bump][..]][..]]), __program_id)?;
                                       }
                                    match anchor_lang::accounts::account::Account::try_from_unchecked(&escrow)
                                        {
                                        Ok(val) => val,
                                        Err(e) => return Err(e.with_account_name("escrow")),
                                    }
                                } else {
                                   match anchor_lang::accounts::account::Account::try_from(&escrow)
                                       {
                                       Ok(val) => val,
                                       Err(e) => return Err(e.with_account_name("escrow")),
                                   }
                               };
                        if false {
                                if space != actual_field.data_len() {
                                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSpace).with_account_name("escrow").with_values((space,
                                                        actual_field.data_len())));
                                    }
                                if actual_owner != __program_id {
                                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintOwner).with_account_name("escrow").with_pubkeys((*actual_owner,
                                                        *__program_id)));
                                    }
                                {
                                    let required_lamports =
                                        __anchor_rent.minimum_balance(space);
                                    if pa.to_account_info().lamports() < required_lamports {
                                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRentExempt).with_account_name("escrow"));
                                        }
                                }
                            }
                        pa
                    };
                if !AsRef::<AccountInfo>::as_ref(&escrow).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("escrow"));
                    }
                if !__anchor_rent.is_exempt(escrow.to_account_info().lamports(),
                                escrow.to_account_info().try_data_len()?) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRentExempt).with_account_name("escrow"));
                    }
                if !AsRef::<AccountInfo>::as_ref(&sender).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("sender"));
                    }
                if !AsRef::<AccountInfo>::as_ref(&sender_token_account).is_writable
                        {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("sender_token_account"));
                    }
                {
                    if sender_token_account.owner != sender.key() {
                            return Err(anchor_lang::error::ErrorCode::ConstraintTokenOwner.into());
                        }
                    if sender_token_account.mint != mint.key() {
                            return Err(anchor_lang::error::ErrorCode::ConstraintTokenMint.into());
                        }
                }
                if !AsRef::<AccountInfo>::as_ref(&escrow_token_account).is_writable
                        {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("escrow_token_account"));
                    }
                {
                    if escrow_token_account.mint != mint.key() {
                            return Err(anchor_lang::error::ErrorCode::ConstraintTokenMint.into());
                        }
                }
                Ok(InitVesting {
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
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitVesting<'info>
            where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.sender.to_account_infos());
                account_infos.extend(self.sender_token_account.to_account_infos());
                account_infos.extend(self.escrow.to_account_infos());
                account_infos.extend(self.escrow_token_account.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitVesting<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.sender.to_account_metas(None));
                account_metas.extend(self.sender_token_account.to_account_metas(None));
                account_metas.extend(self.escrow.to_account_metas(None));
                account_metas.extend(self.escrow_token_account.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitVesting<'info>
            where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.sender,
                            program_id).map_err(|e| e.with_account_name("sender"))?;
                anchor_lang::AccountsExit::exit(&self.sender_token_account,
                            program_id).map_err(|e|
                            e.with_account_name("sender_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.escrow,
                            program_id).map_err(|e| e.with_account_name("escrow"))?;
                anchor_lang::AccountsExit::exit(&self.escrow_token_account,
                            program_id).map_err(|e|
                            e.with_account_name("escrow_token_account"))?;
                Ok(())
            }
        }
        pub struct InitVestingBumps {
            pub escrow: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InitVestingBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f,
                    "InitVestingBumps", "escrow", &&self.escrow)
            }
        }
        impl Default for InitVestingBumps {
            fn default() -> Self { InitVestingBumps { escrow: u8::MAX } }
        }
        impl<'info> anchor_lang::Bumps for InitVesting<'info> where
            'info: 'info {
            type Bumps = InitVestingBumps;
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_init_vesting {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`InitVesting`]."]
            pub struct InitVesting {
                pub sender: Pubkey,
                pub sender_token_account: Pubkey,
                pub escrow: Pubkey,
                pub escrow_token_account: Pubkey,
                pub mint: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitVesting where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.sender, writer)?;
                    borsh::BorshSerialize::serialize(&self.sender_token_account,
                            writer)?;
                    borsh::BorshSerialize::serialize(&self.escrow, writer)?;
                    borsh::BorshSerialize::serialize(&self.escrow_token_account,
                            writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program,
                            writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program,
                            writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitVesting {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.sender,
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.sender_token_account,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.escrow,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.escrow_token_account,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.mint,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.token_program,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.system_program,
                            false));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_init_vesting {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`InitVesting`]."]
            pub struct InitVesting<'info> {
                pub sender: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub sender_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub escrow: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub escrow_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitVesting<'info> {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.sender),
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.sender_token_account),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.escrow),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.escrow_token_account),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.mint),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.token_program),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.system_program),
                            false));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                InitVesting<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.sender));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.sender_token_account));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.escrow));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.escrow_token_account));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.mint));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.token_program));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.system_program));
                    account_infos
                }
            }
        }
    }
    pub mod withdraw {
        use anchor_lang::prelude::*;
        use anchor_spl::token::{
            transfer, Mint, Token, TokenAccount, Transfer,
        };
        use crate::{state::Escrow, VestingError};
        pub fn _withdraw_unlocked(ctx: Context<Withdraw>) -> Result<()> {
            let escrow = &mut ctx.accounts.escrow;
            let current_time = Clock::get()?.unix_timestamp as u64;
            let unlocked_amount =
                escrow.amount_unlocked(current_time).ok_or(VestingError::InvalidAmount)?;
            let seeds =
                &[b"ESCROW_PDA_AUTHORITY".as_ref(),
                            &[ctx.bumps.escrow_pda_authority]];
            transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(),
                            Transfer {
                                from: ctx.accounts.escrow_token_account.to_account_info(),
                                to: ctx.accounts.recipient_token_account.to_account_info(),
                                authority: ctx.accounts.escrow_pda_authority.to_account_info(),
                            }).with_signer(&[&seeds[..]]), unlocked_amount)?;
            escrow.withdrawal += unlocked_amount;
            Ok(())
        }
        pub struct Withdraw<'info> {
            #[account(mut)]
            pub recipient: Signer<'info>,
            #[account(mut, token::mint = mint, token::authority = recipient)]
            pub recipient_token_account: Account<'info, TokenAccount>,
            #[account(mut, has_one = recipient, close = recipient, seeds =
            [escrow.recipient.key().as_ref(),b"ESCROW_SEED"], bump =
            escrow.bump)]
            pub escrow: Account<'info, Escrow>,
            #[account(mut, token::mint = mint, token::authority =
            escrow_pda_authority)]
            pub escrow_token_account: Account<'info, TokenAccount>,
            /// CHECK: we do not read or write to this account
            #[account(seeds = [b"ESCROW_PDA_AUTHORITY"], bump)]
            pub escrow_pda_authority: AccountInfo<'info>,
            pub mint: Account<'info, Mint>,
            pub token_program: Program<'info, Token>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, WithdrawBumps> for
            Withdraw<'info> where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8], __bumps: &mut WithdrawBumps,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let recipient: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("recipient"))?;
                let recipient_token_account:
                        anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("recipient_token_account"))?;
                let escrow: anchor_lang::accounts::account::Account<Escrow> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("escrow"))?;
                let escrow_token_account:
                        anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("escrow_token_account"))?;
                let escrow_pda_authority: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("escrow_pda_authority"))?;
                let mint: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("mint"))?;
                let token_program:
                        anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("token_program"))?;
                let system_program:
                        anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("system_program"))?;
                if !AsRef::<AccountInfo>::as_ref(&recipient).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("recipient"));
                    }
                if !AsRef::<AccountInfo>::as_ref(&recipient_token_account).is_writable
                        {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("recipient_token_account"));
                    }
                {
                    if recipient_token_account.owner != recipient.key() {
                            return Err(anchor_lang::error::ErrorCode::ConstraintTokenOwner.into());
                        }
                    if recipient_token_account.mint != mint.key() {
                            return Err(anchor_lang::error::ErrorCode::ConstraintTokenMint.into());
                        }
                }
                let __pda_address =
                    Pubkey::create_program_address(&[escrow.recipient.key().as_ref(),
                                            b"ESCROW_SEED", &[escrow.bump][..]],
                                &__program_id).map_err(|_|
                                anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds).with_account_name("escrow"))?;
                if escrow.key() != __pda_address {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds).with_account_name("escrow").with_pubkeys((escrow.key(),
                                        __pda_address)));
                    }
                if !AsRef::<AccountInfo>::as_ref(&escrow).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("escrow"));
                    }
                {
                    let my_key = escrow.recipient;
                    let target_key = recipient.key();
                    if my_key != target_key {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintHasOne).with_account_name("escrow").with_pubkeys((my_key,
                                            target_key)));
                        }
                }
                {
                    if escrow.key() == recipient.key() {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintClose).with_account_name("escrow"));
                        }
                }
                if !AsRef::<AccountInfo>::as_ref(&escrow_token_account).is_writable
                        {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("escrow_token_account"));
                    }
                {
                    if escrow_token_account.owner != escrow_pda_authority.key()
                            {
                            return Err(anchor_lang::error::ErrorCode::ConstraintTokenOwner.into());
                        }
                    if escrow_token_account.mint != mint.key() {
                            return Err(anchor_lang::error::ErrorCode::ConstraintTokenMint.into());
                        }
                }
                let (__pda_address, __bump) =
                    Pubkey::find_program_address(&[b"ESCROW_PDA_AUTHORITY"],
                        &__program_id);
                __bumps.escrow_pda_authority = __bump;
                if escrow_pda_authority.key() != __pda_address {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds).with_account_name("escrow_pda_authority").with_pubkeys((escrow_pda_authority.key(),
                                        __pda_address)));
                    }
                Ok(Withdraw {
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
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info>
            where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.recipient.to_account_infos());
                account_infos.extend(self.recipient_token_account.to_account_infos());
                account_infos.extend(self.escrow.to_account_infos());
                account_infos.extend(self.escrow_token_account.to_account_infos());
                account_infos.extend(self.escrow_pda_authority.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.recipient.to_account_metas(None));
                account_metas.extend(self.recipient_token_account.to_account_metas(None));
                account_metas.extend(self.escrow.to_account_metas(None));
                account_metas.extend(self.escrow_token_account.to_account_metas(None));
                account_metas.extend(self.escrow_pda_authority.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Withdraw<'info> where
            'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.recipient,
                            program_id).map_err(|e| e.with_account_name("recipient"))?;
                anchor_lang::AccountsExit::exit(&self.recipient_token_account,
                            program_id).map_err(|e|
                            e.with_account_name("recipient_token_account"))?;
                {
                    let recipient = &self.recipient;
                    anchor_lang::AccountsClose::close(&self.escrow,
                                recipient.to_account_info()).map_err(|e|
                                e.with_account_name("escrow"))?;
                }
                anchor_lang::AccountsExit::exit(&self.escrow_token_account,
                            program_id).map_err(|e|
                            e.with_account_name("escrow_token_account"))?;
                Ok(())
            }
        }
        pub struct WithdrawBumps {
            pub escrow_pda_authority: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for WithdrawBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f,
                    "WithdrawBumps", "escrow_pda_authority",
                    &&self.escrow_pda_authority)
            }
        }
        impl Default for WithdrawBumps {
            fn default() -> Self {
                WithdrawBumps { escrow_pda_authority: u8::MAX }
            }
        }
        impl<'info> anchor_lang::Bumps for Withdraw<'info> where 'info: 'info
            {
            type Bumps = WithdrawBumps;
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_withdraw {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`Withdraw`]."]
            pub struct Withdraw {
                pub recipient: Pubkey,
                pub recipient_token_account: Pubkey,
                pub escrow: Pubkey,
                pub escrow_token_account: Pubkey,
                pub escrow_pda_authority: Pubkey,
                pub mint: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for Withdraw where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.recipient, writer)?;
                    borsh::BorshSerialize::serialize(&self.recipient_token_account,
                            writer)?;
                    borsh::BorshSerialize::serialize(&self.escrow, writer)?;
                    borsh::BorshSerialize::serialize(&self.escrow_token_account,
                            writer)?;
                    borsh::BorshSerialize::serialize(&self.escrow_pda_authority,
                            writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program,
                            writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program,
                            writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Withdraw {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.recipient,
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.recipient_token_account,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.escrow,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.escrow_token_account,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.escrow_pda_authority,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.mint,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.token_program,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.system_program,
                            false));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_withdraw {
            use super::*;
            #[doc = " Generated CPI struct of the accounts for [`Withdraw`]."]
            pub struct Withdraw<'info> {
                pub recipient: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub recipient_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub escrow: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub escrow_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub escrow_pda_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.recipient),
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.recipient_token_account),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.escrow),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.escrow_token_account),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.escrow_pda_authority),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.mint),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.token_program),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.system_program),
                            false));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info>
                {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.recipient));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.recipient_token_account));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.escrow));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.escrow_token_account));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.escrow_pda_authority));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.mint));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.token_program));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.system_program));
                    account_infos
                }
            }
        }
    }
    pub use initialize::*;
    pub use withdraw::*;
}
pub mod state {
    use anchor_lang::prelude::*;
    pub struct Escrow {
        pub recipient: Pubkey,
        pub amount: u64,
        pub withdrawal: u64,
        pub start_time: u64,
        pub end_time: u64,
        pub interval: u64,
        pub bump: u8,
    }
    impl borsh::ser::BorshSerialize for Escrow where
        Pubkey: borsh::ser::BorshSerialize, u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize, u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize, u64: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.recipient, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            borsh::BorshSerialize::serialize(&self.withdrawal, writer)?;
            borsh::BorshSerialize::serialize(&self.start_time, writer)?;
            borsh::BorshSerialize::serialize(&self.end_time, writer)?;
            borsh::BorshSerialize::serialize(&self.interval, writer)?;
            borsh::BorshSerialize::serialize(&self.bump, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Escrow where
        Pubkey: borsh::BorshDeserialize, u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize, u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize, u64: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                    recipient: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    withdrawal: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    start_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    end_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    interval: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Escrow {
        #[inline]
        fn clone(&self) -> Escrow {
            Escrow {
                recipient: ::core::clone::Clone::clone(&self.recipient),
                amount: ::core::clone::Clone::clone(&self.amount),
                withdrawal: ::core::clone::Clone::clone(&self.withdrawal),
                start_time: ::core::clone::Clone::clone(&self.start_time),
                end_time: ::core::clone::Clone::clone(&self.end_time),
                interval: ::core::clone::Clone::clone(&self.interval),
                bump: ::core::clone::Clone::clone(&self.bump),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Escrow {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W)
            -> anchor_lang::Result<()> {
            if writer.write_all(&[31, 213, 123, 187, 186, 22, 218,
                                        155]).is_err() {
                    return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                }
            if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Escrow {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [31, 213, 123, 187, 186, 22, 218, 155].len() {
                    return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
                }
            let given_disc = &buf[..8];
            if &[31, 213, 123, 187, 186, 22, 218, 155] != given_disc {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                        error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.name(),
                                        error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
                                        error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.to_string(),
                                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/fuzz_example3/src/state.rs",
                                                    line: 3u32,
                                                })),
                                        compared_values: None,
                                    }).with_account_name("Escrow"));
                }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8])
            -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data).map_err(|_|
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Escrow {
        const DISCRIMINATOR: [u8; 8] = [31, 213, 123, 187, 186, 22, 218, 155];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for Escrow {
        fn owner() -> Pubkey { crate::ID }
    }
    impl Escrow {
        pub fn amount_unlocked(&self, now: u64) -> Option<u64> {
            let time = if now < self.end_time { now } else { self.end_time };
            let duration = self.end_time.checked_sub(self.start_time)?;
            let interval_amount =
                self.amount.checked_mul(self.interval)?.checked_div(duration)?;
            let nr_intervals =
                time.checked_sub(self.start_time)?.checked_div(self.interval)?.checked_add(1)?;
            nr_intervals.checked_mul(interval_amount)?.checked_sub(self.withdrawal)
        }
    }
}
use crate::instructions::*;
pub use error::*;
#[doc = r" The static program ID"]
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([222u8, 219u8,
                96u8, 222u8, 150u8, 129u8, 32u8, 71u8, 184u8, 221u8, 54u8,
                221u8, 224u8, 97u8, 103u8, 133u8, 11u8, 126u8, 234u8, 11u8,
                186u8, 25u8, 119u8, 161u8, 48u8, 137u8, 77u8, 249u8, 144u8,
                153u8, 133u8, 92u8]);
#[doc = r" Confirms that a given pubkey is equivalent to the program ID"]
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
#[doc = r" Returns the program ID"]
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey { ID }
use self::fuzz_example3::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
#[doc =
r" The Anchor codegen exposes a programming model where a user defines"]
#[doc = r" a set of methods inside of a `#[program]` module in a way similar"]
#[doc =
r" to writing RPC request handlers. The macro then generates a bunch of"]
#[doc =
r" code wrapping these user defined methods into something that can be"]
#[doc = r" executed on Solana."]
#[doc = r""]
#[doc = r" These methods fall into one category for now."]
#[doc = r""]
#[doc = r" Global methods - regular methods inside of the `#[program]`."]
#[doc = r""]
#[doc = r" Care must be taken by the codegen to prevent collisions between"]
#[doc =
r" methods in these different namespaces. For this reason, Anchor uses"]
#[doc = r" a variant of sighash to perform method dispatch, rather than"]
#[doc = r" something like a simple enum variant discriminator."]
#[doc = r""]
#[doc = r" The execution flow of the generated code can be roughly outlined:"]
#[doc = r""]
#[doc = r" * Start program via the entrypoint."]
#[doc =
r" * Strip method identifier off the first 8 bytes of the instruction"]
#[doc = r"   data and invoke the identified method. The method identifier"]
#[doc =
r"   is a variant of sighash. See docs.rs for `anchor_lang` for details."]
#[doc = r" * If the method identifier is an IDL identifier, execute the IDL"]
#[doc = r"   instructions, which are a special set of hardcoded instructions"]
#[doc = r"   baked into every Anchor program. Then exit."]
#[doc = r" * Otherwise, the method identifier is for a user defined"]
#[doc = r"   instruction, i.e., one of the methods in the user defined"]
#[doc = r"   `#[program]` module. Perform method dispatch, i.e., execute the"]
#[doc = r"   big match statement mapping method identifier to method handler"]
#[doc = r"   wrapper."]
#[doc = r" * Run the method handler wrapper. This wraps the code the user"]
#[doc = r"   actually wrote, deserializing the accounts, constructing the"]
#[doc = r"   context, invoking the user's code, and finally running the exit"]
#[doc = r"   routine, which typically persists account changes."]
#[doc = r""]
#[doc = r" The `entry` function here, defines the standard entry to a Solana"]
#[doc = r" program, where execution begins."]
pub fn entry<'info>(program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>], data: &[u8])
    -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data).map_err(|e| { e.log(); e.into() })
}
fn try_entry<'info>(program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>], data: &[u8])
    -> anchor_lang::Result<()> {
    if *program_id != ID {
            return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
        }
    if data.len() < 8 {
            return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
        }
    dispatch(program_id, accounts, data)
}
#[doc = r" Module representing the program."]
pub mod program {
    use super::*;
    #[doc = r" Type representing the program."]
    pub struct FuzzExample3;
    #[automatically_derived]
    impl ::core::clone::Clone for FuzzExample3 {
        #[inline]
        fn clone(&self) -> FuzzExample3 { FuzzExample3 }
    }
    impl anchor_lang::Id for FuzzExample3 {
        fn id() -> Pubkey { ID }
    }
}
#[doc = r" Performs method dispatch."]
#[doc = r""]
#[doc =
r" Each method in an anchor program is uniquely defined by a namespace"]
#[doc = r" and a rust identifier (i.e., the name given to the method). These"]
#[doc = r" two pieces can be combined to creater a method identifier,"]
#[doc = r" specifically, Anchor uses"]
#[doc = r""]
#[doc = r#" Sha256("<namespace>:<rust-identifier>")[..8],"#]
#[doc = r""]
#[doc = r#" where the namespace can be one type. "global" for a"#]
#[doc = r" regular instruction."]
#[doc = r""]
#[doc = r" With this 8 byte identifier, Anchor performs method dispatch,"]
#[doc = r" matching the given 8 byte identifier to the associated method"]
#[doc =
r" handler, which leads to user defined code being eventually invoked."]
fn dispatch<'info>(program_id: &Pubkey, accounts: &'info [AccountInfo<'info>],
    data: &[u8]) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] =
        {
            let mut sighash: [u8; 8] = [0; 8];
            sighash.copy_from_slice(&ix_data[..8]);
            ix_data = &ix_data[8..];
            sighash
        };
    use anchor_lang::Discriminator;
    match sighash {
        instruction::InitVesting::DISCRIMINATOR => {
            __private::__global::init_vesting(program_id, accounts, ix_data)
        }
        instruction::WithdrawUnlocked::DISCRIMINATOR => {
            __private::__global::withdraw_unlocked(program_id, accounts,
                ix_data)
        }
        instruction::WithdrawDummy::DISCRIMINATOR => {
            __private::__global::withdraw_dummy(program_id, accounts, ix_data)
        }
        anchor_lang::idl::IDL_IX_TAG_LE => {

            #[cfg(not(feature = "no-idl"))]
            {
                __private::__idl::__idl_dispatch(program_id, accounts,
                    &ix_data)
            }
        }
        anchor_lang::event::EVENT_IX_TAG_LE => {
            Err(anchor_lang::error::ErrorCode::EventInstructionStub.into())
        }
        _ => {
            Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into())
        }
    }
}
#[doc = r" Create a private module to not clutter the program's namespace."]
#[doc = r" Defines an entrypoint for each individual instruction handler"]
#[doc = r" wrapper."]
mod __private {
    use super::*;
    #[doc =
    r" __idl mod defines handlers for injected Anchor IDL instructions."]
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch<'info>(program_id: &Pubkey,
            accounts: &'info [AccountInfo<'info>], idl_ix_data: &[u8])
            -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix =
                anchor_lang::idl::IdlInstruction::deserialize(&mut data).map_err(|_|
                            anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps =
                        <IdlCreateAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlCreateAccounts::try_accounts(program_id, &mut accounts,
                                &[], &mut bumps, &mut reallocs)?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Resize { data_len } => {
                    let mut bumps =
                        <IdlResizeAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlResizeAccount::try_accounts(program_id, &mut accounts,
                                &[], &mut bumps, &mut reallocs)?;
                    __idl_resize_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Close => {
                    let mut bumps =
                        <IdlCloseAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlCloseAccount::try_accounts(program_id, &mut accounts,
                                &[], &mut bumps, &mut reallocs)?;
                    __idl_close_account(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps =
                        <IdlCreateBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlCreateBuffer::try_accounts(program_id, &mut accounts,
                                &[], &mut bumps, &mut reallocs)?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps =
                        <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlAccounts::try_accounts(program_id, &mut accounts, &[],
                                &mut bumps, &mut reallocs)?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority
                    } => {
                    let mut bumps =
                        <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlAccounts::try_accounts(program_id, &mut accounts, &[],
                                &mut bumps, &mut reallocs)?;
                    __idl_set_authority(program_id, &mut accounts,
                            new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps =
                        <IdlSetBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts =
                        IdlSetBuffer::try_accounts(program_id, &mut accounts, &[],
                                &mut bumps, &mut reallocs)?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        use anchor_lang::idl::ERASED_AUTHORITY;
        pub struct IdlAccount {
            pub authority: Pubkey,
            pub data_len: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccount {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(f,
                    "IdlAccount", "authority", &self.authority, "data_len",
                    &&self.data_len)
            }
        }
        impl borsh::ser::BorshSerialize for IdlAccount where
            Pubkey: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize {
            fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
                -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.data_len, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for IdlAccount where
            Pubkey: borsh::BorshDeserialize, u32: borsh::BorshDeserialize {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(reader:
                    &mut R)
                -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                        authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        data_len: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IdlAccount {
            #[inline]
            fn clone(&self) -> IdlAccount {
                IdlAccount {
                    authority: ::core::clone::Clone::clone(&self.authority),
                    data_len: ::core::clone::Clone::clone(&self.data_len),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for IdlAccount {
            fn try_serialize<W: std::io::Write>(&self, writer: &mut W)
                -> anchor_lang::Result<()> {
                if writer.write_all(&[24, 70, 98, 191, 58, 144, 123,
                                            158]).is_err() {
                        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                    }
                if AnchorSerialize::serialize(self, writer).is_err() {
                        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                    }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for IdlAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [24, 70, 98, 191, 58, 144, 123, 158].len() {
                        return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
                    }
                let given_disc = &buf[..8];
                if &[24, 70, 98, 191, 58, 144, 123, 158] != given_disc {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.name(),
                                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
                                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.to_string(),
                                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/fuzz_example3/src/lib.rs",
                                                        line: 11u32,
                                                    })),
                                            compared_values: None,
                                        }).with_account_name("IdlAccount"));
                    }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8])
                -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data).map_err(|_|
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for IdlAccount {
            const DISCRIMINATOR: [u8; 8] =
                [24, 70, 98, 191, 58, 144, 123, 158];
        }
        impl IdlAccount {
            pub fn address(program_id: &Pubkey) -> Pubkey {
                let program_signer =
                    Pubkey::find_program_address(&[], program_id).0;
                Pubkey::create_with_seed(&program_signer, IdlAccount::seed(),
                        program_id).expect("Seed is always valid")
            }
            pub fn seed() -> &'static str { "anchor:idl" }
        }
        impl anchor_lang::Owner for IdlAccount {
            fn owner() -> Pubkey { crate::ID }
        }
        pub struct IdlCreateAccounts<'info> {
            #[account(signer)]
            pub from: AccountInfo<'info>,
            #[account(mut)]
            pub to: AccountInfo<'info>,
            #[account(seeds = [], bump)]
            pub base: AccountInfo<'info>,
            pub system_program: Program<'info, System>,
            #[account(executable)]
            pub program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateAccountsBumps> for
            IdlCreateAccounts<'info> where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8], __bumps: &mut IdlCreateAccountsBumps,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let from: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("from"))?;
                let to: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("to"))?;
                let base: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("base"))?;
                let system_program:
                        anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("system_program"))?;
                let program: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("program"))?;
                if !&from.is_signer {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSigner).with_account_name("from"));
                    }
                if !&to.is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("to"));
                    }
                let (__pda_address, __bump) =
                    Pubkey::find_program_address(&[], &__program_id);
                __bumps.base = __bump;
                if base.key() != __pda_address {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds).with_account_name("base").with_pubkeys((base.key(),
                                        __pda_address)));
                    }
                if !&program.executable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintExecutable).with_account_name("program"));
                    }
                Ok(IdlCreateAccounts {
                        from,
                        to,
                        base,
                        system_program,
                        program,
                    })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for
            IdlCreateAccounts<'info> where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.from.to_account_infos());
                account_infos.extend(self.to.to_account_infos());
                account_infos.extend(self.base.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.from.to_account_metas(Some(true)));
                account_metas.extend(self.to.to_account_metas(None));
                account_metas.extend(self.base.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for
            IdlCreateAccounts<'info> where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.to,
                            program_id).map_err(|e| e.with_account_name("to"))?;
                Ok(())
            }
        }
        pub struct IdlCreateAccountsBumps {
            pub base: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f,
                    "IdlCreateAccountsBumps", "base", &&self.base)
            }
        }
        impl Default for IdlCreateAccountsBumps {
            fn default() -> Self { IdlCreateAccountsBumps { base: u8::MAX } }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateAccounts<'info> where
            'info: 'info {
            type Bumps = IdlCreateAccountsBumps;
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_create_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlCreateAccounts`]."]
            pub struct IdlCreateAccounts {
                pub from: Pubkey,
                pub to: Pubkey,
                pub base: Pubkey,
                pub system_program: Pubkey,
                pub program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateAccounts where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.from, writer)?;
                    borsh::BorshSerialize::serialize(&self.to, writer)?;
                    borsh::BorshSerialize::serialize(&self.base, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program,
                            writer)?;
                    borsh::BorshSerialize::serialize(&self.program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateAccounts {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.from,
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.to,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.base,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.system_program,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.program,
                            false));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_create_accounts {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlCreateAccounts`]."]
            pub struct IdlCreateAccounts<'info> {
                pub from: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub to: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub base: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for
                IdlCreateAccounts<'info> {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.from),
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.to),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.base),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.system_program),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.program),
                            false));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlCreateAccounts<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.from));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.to));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.base));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.system_program));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.program));
                    account_infos
                }
            }
        }
        pub struct IdlAccounts<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key != & ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlAccountsBumps> for
            IdlAccounts<'info> where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8], __bumps: &mut IdlAccountsBumps,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("idl"));
                    }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintHasOne).with_account_name("idl").with_pubkeys((my_key,
                                            target_key)));
                        }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("authority"));
                    }
                Ok(IdlAccounts { idl, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info>
            where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlAccounts<'info>
            where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl,
                            program_id).map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlAccountsBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlAccountsBumps")
            }
        }
        impl Default for IdlAccountsBumps {
            fn default() -> Self { IdlAccountsBumps {} }
        }
        impl<'info> anchor_lang::Bumps for IdlAccounts<'info> where
            'info: 'info {
            type Bumps = IdlAccountsBumps;
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlAccounts`]."]
            pub struct IdlAccounts {
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlAccounts where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlAccounts {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.idl,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.authority,
                            true));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_accounts {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlAccounts`]."]
            pub struct IdlAccounts<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.idl),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.authority),
                            true));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlAccounts<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.idl));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
                    account_infos
                }
            }
        }
        pub struct IdlResizeAccount<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(mut, constraint = authority.key != & ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlResizeAccountBumps> for
            IdlResizeAccount<'info> where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8], __bumps: &mut IdlResizeAccountBumps,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("authority"))?;
                let system_program:
                        anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("system_program"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("idl"));
                    }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintHasOne).with_account_name("idl").with_pubkeys((my_key,
                                            target_key)));
                        }
                }
                if !AsRef::<AccountInfo>::as_ref(&authority).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("authority"));
                    }
                if !(authority.key != &ERASED_AUTHORITY) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("authority"));
                    }
                Ok(IdlResizeAccount { idl, authority, system_program })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for
            IdlResizeAccount<'info> where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for
            IdlResizeAccount<'info> where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl,
                            program_id).map_err(|e| e.with_account_name("idl"))?;
                anchor_lang::AccountsExit::exit(&self.authority,
                            program_id).map_err(|e| e.with_account_name("authority"))?;
                Ok(())
            }
        }
        pub struct IdlResizeAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlResizeAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlResizeAccountBumps")
            }
        }
        impl Default for IdlResizeAccountBumps {
            fn default() -> Self { IdlResizeAccountBumps {} }
        }
        impl<'info> anchor_lang::Bumps for IdlResizeAccount<'info> where
            'info: 'info {
            type Bumps = IdlResizeAccountBumps;
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_resize_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlResizeAccount`]."]
            pub struct IdlResizeAccount {
                pub idl: Pubkey,
                pub authority: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlResizeAccount where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program,
                            writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlResizeAccount {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.idl,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.authority,
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.system_program,
                            false));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_resize_account {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlResizeAccount`]."]
            pub struct IdlResizeAccount<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for
                IdlResizeAccount<'info> {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.idl),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.authority),
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.system_program),
                            false));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlResizeAccount<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.idl));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.system_program));
                    account_infos
                }
            }
        }
        pub struct IdlCreateBuffer<'info> {
            #[account(zero)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(constraint = authority.key != & ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateBufferBumps> for
            IdlCreateBuffer<'info> where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8], __bumps: &mut IdlCreateBufferBumps,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                if __accounts.is_empty() {
                        return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
                    }
                let buffer = &__accounts[0];
                *__accounts = &__accounts[1..];
                let authority: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("authority"))?;
                let __anchor_rent = Rent::get()?;
                let buffer:
                        anchor_lang::accounts::account::Account<IdlAccount> =
                    {
                        let mut __data: &[u8] = &buffer.try_borrow_data()?;
                        let mut __disc_bytes = [0u8; 8];
                        __disc_bytes.copy_from_slice(&__data[..8]);
                        let __discriminator = u64::from_le_bytes(__disc_bytes);
                        if __discriminator != 0 {
                                return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintZero).with_account_name("buffer"));
                            }
                        match anchor_lang::accounts::account::Account::try_from_unchecked(&buffer)
                            {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("buffer")),
                        }
                    };
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("buffer"));
                    }
                if !__anchor_rent.is_exempt(buffer.to_account_info().lamports(),
                                buffer.to_account_info().try_data_len()?) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRentExempt).with_account_name("buffer"));
                    }
                if !(authority.key != &ERASED_AUTHORITY) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("authority"));
                    }
                Ok(IdlCreateBuffer { buffer, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for
            IdlCreateBuffer<'info> where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for
            IdlCreateBuffer<'info> where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer,
                            program_id).map_err(|e| e.with_account_name("buffer"))?;
                Ok(())
            }
        }
        pub struct IdlCreateBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCreateBufferBumps")
            }
        }
        impl Default for IdlCreateBufferBumps {
            fn default() -> Self { IdlCreateBufferBumps {} }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateBuffer<'info> where
            'info: 'info {
            type Bumps = IdlCreateBufferBumps;
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_create_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlCreateBuffer`]."]
            pub struct IdlCreateBuffer {
                pub buffer: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateBuffer where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateBuffer {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.buffer,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.authority,
                            true));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_create_buffer {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlCreateBuffer`]."]
            pub struct IdlCreateBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info>
                {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.buffer),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.authority),
                            true));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlCreateBuffer<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.buffer));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
                    account_infos
                }
            }
        }
        pub struct IdlSetBuffer<'info> {
            #[account(mut, constraint = buffer.authority == idl.authority)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key != & ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlSetBufferBumps> for
            IdlSetBuffer<'info> where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8], __bumps: &mut IdlSetBufferBumps,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let buffer:
                        anchor_lang::accounts::account::Account<IdlAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("buffer"))?;
                let idl: anchor_lang::accounts::account::Account<IdlAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("buffer"));
                    }
                if !(buffer.authority == idl.authority) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("buffer"));
                    }
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("idl"));
                    }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintHasOne).with_account_name("idl").with_pubkeys((my_key,
                                            target_key)));
                        }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("authority"));
                    }
                Ok(IdlSetBuffer { buffer, idl, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info>
            where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlSetBuffer<'info>
            where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer,
                            program_id).map_err(|e| e.with_account_name("buffer"))?;
                anchor_lang::AccountsExit::exit(&self.idl,
                            program_id).map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlSetBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlSetBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlSetBufferBumps")
            }
        }
        impl Default for IdlSetBufferBumps {
            fn default() -> Self { IdlSetBufferBumps {} }
        }
        impl<'info> anchor_lang::Bumps for IdlSetBuffer<'info> where
            'info: 'info {
            type Bumps = IdlSetBufferBumps;
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_set_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlSetBuffer`]."]
            pub struct IdlSetBuffer {
                pub buffer: Pubkey,
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlSetBuffer where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlSetBuffer {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.buffer,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.idl,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.authority,
                            true));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_set_buffer {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlSetBuffer`]."]
            pub struct IdlSetBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.buffer),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.idl),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.authority),
                            true));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlSetBuffer<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.buffer));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.idl));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
                    account_infos
                }
            }
        }
        pub struct IdlCloseAccount<'info> {
            #[account(mut, has_one = authority, close = sol_destination)]
            pub account: Account<'info, IdlAccount>,
            #[account(constraint = authority.key != & ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            #[account(mut)]
            pub sol_destination: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCloseAccountBumps> for
            IdlCloseAccount<'info> where 'info: 'info {
            #[inline(never)]
            fn try_accounts(__program_id:
                    &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts:
                    &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<'info>],
                __ix_data: &[u8], __bumps: &mut IdlCloseAccountBumps,
                __reallocs:
                    &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>)
                -> anchor_lang::Result<Self> {
                let account:
                        anchor_lang::accounts::account::Account<IdlAccount> =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("account"))?;
                let authority: Signer =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e| e.with_account_name("authority"))?;
                let sol_destination: AccountInfo =
                    anchor_lang::Accounts::try_accounts(__program_id,
                                __accounts, __ix_data, __bumps,
                                __reallocs).map_err(|e|
                                e.with_account_name("sol_destination"))?;
                if !AsRef::<AccountInfo>::as_ref(&account).is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("account"));
                    }
                {
                    let my_key = account.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintHasOne).with_account_name("account").with_pubkeys((my_key,
                                            target_key)));
                        }
                }
                {
                    if account.key() == sol_destination.key() {
                            return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintClose).with_account_name("account"));
                        }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRaw).with_account_name("authority"));
                    }
                if !&sol_destination.is_writable {
                        return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintMut).with_account_name("sol_destination"));
                    }
                Ok(IdlCloseAccount { account, authority, sol_destination })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for
            IdlCloseAccount<'info> where 'info: 'info {
            fn to_account_infos(&self)
                ->
                    Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.account.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.sol_destination.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
            fn to_account_metas(&self, is_signer: Option<bool>)
                ->
                    Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.account.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.sol_destination.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for
            IdlCloseAccount<'info> where 'info: 'info {
            fn exit(&self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey)
                -> anchor_lang::Result<()> {
                {
                    let sol_destination = &self.sol_destination;
                    anchor_lang::AccountsClose::close(&self.account,
                                sol_destination.to_account_info()).map_err(|e|
                                e.with_account_name("account"))?;
                }
                anchor_lang::AccountsExit::exit(&self.sol_destination,
                            program_id).map_err(|e|
                            e.with_account_name("sol_destination"))?;
                Ok(())
            }
        }
        pub struct IdlCloseAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCloseAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCloseAccountBumps")
            }
        }
        impl Default for IdlCloseAccountBumps {
            fn default() -> Self { IdlCloseAccountBumps {} }
        }
        impl<'info> anchor_lang::Bumps for IdlCloseAccount<'info> where
            'info: 'info {
            type Bumps = IdlCloseAccountBumps;
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc = r" implementation detail), to generate a struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
        #[doc =
        r" instead of an `AccountInfo`. This is useful for clients that want"]
        #[doc =
        r" to generate a list of accounts, without explicitly knowing the"]
        #[doc = r" order all the fields should be in."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" `accounts` module (also generated), which re-exports this."]
        pub(crate) mod __client_accounts_idl_close_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            #[doc = " Generated client accounts for [`IdlCloseAccount`]."]
            pub struct IdlCloseAccount {
                pub account: Pubkey,
                pub authority: Pubkey,
                pub sol_destination: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCloseAccount where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize {
                fn serialize<W: borsh::maybestd::io::Write>(&self,
                    writer: &mut W)
                    -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.account, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.sol_destination,
                            writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCloseAccount {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.account,
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.authority,
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(self.sol_destination,
                            false));
                    account_metas
                }
            }
        }
        #[doc = r" An internal, Anchor generated module. This is used (as an"]
        #[doc =
        r" implementation detail), to generate a CPI struct for a given"]
        #[doc =
        r" `#[derive(Accounts)]` implementation, where each field is an"]
        #[doc = r" AccountInfo."]
        #[doc = r""]
        #[doc =
        r" To access the struct in this module, one should use the sibling"]
        #[doc =
        r" [`cpi::accounts`] module (also generated), which re-exports this."]
        pub(crate) mod __cpi_client_accounts_idl_close_account {
            use super::*;
            #[doc =
            " Generated CPI struct of the accounts for [`IdlCloseAccount`]."]
            pub struct IdlCloseAccount<'info> {
                pub account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub sol_destination: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info>
                {
                fn to_account_metas(&self, is_signer: Option<bool>)
                    ->
                        Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.account),
                            false));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(anchor_lang::Key::key(&self.authority),
                            true));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(anchor_lang::Key::key(&self.sol_destination),
                            false));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for
                IdlCloseAccount<'info> {
                fn to_account_infos(&self)
                    ->
                        Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.account));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.sol_destination));
                    account_infos
                }
            }
        }
        use std::cell::{Ref, RefMut};
        pub trait IdlTrailingData<'info> {
            fn trailing_data(self)
            -> Ref<'info, [u8]>;
            fn trailing_data_mut(self)
            -> RefMut<'info, [u8]>;
        }
        impl<'a, 'info: 'a> IdlTrailingData<'a> for
            &'a Account<'info, IdlAccount> {
            fn trailing_data(self) -> Ref<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                Ref::map(info.try_borrow_data().unwrap(), |d| &d[44..])
            }
            fn trailing_data_mut(self) -> RefMut<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                RefMut::map(info.try_borrow_mut_data().unwrap(),
                    |d| &mut d[44..])
            }
        }
        #[inline(never)]
        pub fn __idl_create_account(program_id: &Pubkey,
            accounts: &mut IdlCreateAccounts, data_len: u64)
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                    return Err(anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into());
                }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = std::cmp::min(8 + 32 + 4 + data_len as usize, 10_000);
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix =
                anchor_lang::solana_program::system_instruction::create_account_with_seed(from,
                    &to, &base, seed, lamports, space as u64, owner);
            anchor_lang::solana_program::program::invoke_signed(&ix,
                    &[accounts.from.clone(), accounts.to.clone(),
                                accounts.base.clone(),
                                accounts.system_program.to_account_info()], &[seeds])?;
            let mut idl_account =
                {
                    let mut account_data = accounts.to.try_borrow_data()?;
                    let mut account_data_slice: &[u8] = &account_data;
                    IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
                };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_resize_account(program_id: &Pubkey,
            accounts: &mut IdlResizeAccount, data_len: u64)
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlResizeAccount");
            let data_len: usize = data_len as usize;
            if accounts.idl.data_len != 0 {
                    return Err(anchor_lang::error::ErrorCode::IdlAccountNotEmpty.into());
                }
            let idl_ref = AsRef::<AccountInfo>::as_ref(&accounts.idl);
            let new_account_space =
                idl_ref.data_len().checked_add(std::cmp::min(data_len.checked_sub(idl_ref.data_len()).expect("data_len should always be >= the current account space"),
                            10_000)).unwrap();
            if new_account_space > idl_ref.data_len() {
                    let sysvar_rent = Rent::get()?;
                    let new_rent_minimum =
                        sysvar_rent.minimum_balance(new_account_space);
                    anchor_lang::system_program::transfer(anchor_lang::context::CpiContext::new(accounts.system_program.to_account_info(),
                                anchor_lang::system_program::Transfer {
                                    from: accounts.authority.to_account_info(),
                                    to: accounts.idl.to_account_info().clone(),
                                }),
                            new_rent_minimum.checked_sub(idl_ref.lamports()).unwrap())?;
                    idl_ref.realloc(new_account_space, false)?;
                }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_close_account(program_id: &Pubkey,
            accounts: &mut IdlCloseAccount) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCloseAccount");
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(program_id: &Pubkey,
            accounts: &mut IdlCreateBuffer) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(program_id: &Pubkey, accounts: &mut IdlAccounts,
            idl_data: Vec<u8>) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let prev_len: usize =
                ::std::convert::TryInto::<usize>::try_into(accounts.idl.data_len).unwrap();
            let new_len: usize =
                prev_len.checked_add(idl_data.len()).unwrap() as usize;
            accounts.idl.data_len =
                accounts.idl.data_len.checked_add(::std::convert::TryInto::<u32>::try_into(idl_data.len()).unwrap()).unwrap();
            use IdlTrailingData;
            let mut idl_bytes = accounts.idl.trailing_data_mut();
            let idl_expansion = &mut idl_bytes[prev_len..new_len];
            if idl_expansion.len() != idl_data.len() {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                        error_name: anchor_lang::error::ErrorCode::RequireEqViolated.name(),
                                        error_code_number: anchor_lang::error::ErrorCode::RequireEqViolated.into(),
                                        error_msg: anchor_lang::error::ErrorCode::RequireEqViolated.to_string(),
                                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/fuzz_example3/src/lib.rs",
                                                    line: 11u32,
                                                })),
                                        compared_values: None,
                                    }).with_values((idl_expansion.len(), idl_data.len())));
                };
            idl_expansion.copy_from_slice(&idl_data[..]);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(program_id: &Pubkey,
            accounts: &mut IdlAccounts, new_authority: Pubkey)
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(program_id: &Pubkey,
            accounts: &mut IdlSetBuffer) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data_len = accounts.buffer.data_len;
            use IdlTrailingData;
            let buffer_len =
                ::std::convert::TryInto::<usize>::try_into(accounts.buffer.data_len).unwrap();
            let mut target = accounts.idl.trailing_data_mut();
            let source = &accounts.buffer.trailing_data()[..buffer_len];
            if target.len() < buffer_len {
                    return Err(anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                        error_name: anchor_lang::error::ErrorCode::RequireGteViolated.name(),
                                        error_code_number: anchor_lang::error::ErrorCode::RequireGteViolated.into(),
                                        error_msg: anchor_lang::error::ErrorCode::RequireGteViolated.to_string(),
                                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/fuzz_example3/src/lib.rs",
                                                    line: 11u32,
                                                })),
                                        compared_values: None,
                                    }).with_values((target.len(), buffer_len)));
                };
            target[..buffer_len].copy_from_slice(source);
            Ok(())
        }
    }
    #[doc =
    r" __global mod defines wrapped handlers for global instructions."]
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn init_vesting<'info>(__program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>], __ix_data: &[u8])
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitVesting");
            let ix =
                instruction::InitVesting::deserialize(&mut &__ix_data[..]).map_err(|_|
                            anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitVesting {
                    recipient, _recipient, amount, start_at, end_at, interval
                    } = ix;
            let mut __bumps =
                <InitVesting as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts =
                InitVesting::try_accounts(__program_id,
                        &mut __remaining_accounts, __ix_data, &mut __bumps,
                        &mut __reallocs)?;
            let result =
                fuzz_example3::init_vesting(anchor_lang::context::Context::new(__program_id,
                            &mut __accounts, __remaining_accounts, __bumps), recipient,
                        _recipient, amount, start_at, end_at, interval)?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn withdraw_unlocked<'info>(__program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>], __ix_data: &[u8])
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: WithdrawUnlocked");
            let ix =
                instruction::WithdrawUnlocked::deserialize(&mut &__ix_data[..]).map_err(|_|
                            anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::WithdrawUnlocked = ix;
            let mut __bumps =
                <Withdraw as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts =
                Withdraw::try_accounts(__program_id,
                        &mut __remaining_accounts, __ix_data, &mut __bumps,
                        &mut __reallocs)?;
            let result =
                fuzz_example3::withdraw_unlocked(anchor_lang::context::Context::new(__program_id,
                            &mut __accounts, __remaining_accounts, __bumps))?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn withdraw_dummy<'info>(__program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>], __ix_data: &[u8])
            -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: WithdrawDummy");
            let ix =
                instruction::WithdrawDummy::deserialize(&mut &__ix_data[..]).map_err(|_|
                            anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::WithdrawDummy = ix;
            let mut __bumps =
                <Withdraw as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts =
                Withdraw::try_accounts(__program_id,
                        &mut __remaining_accounts, __ix_data, &mut __bumps,
                        &mut __reallocs)?;
            let result =
                fuzz_example3::withdraw_dummy(anchor_lang::context::Context::new(__program_id,
                            &mut __accounts, __remaining_accounts, __bumps))?;
            __accounts.exit(__program_id)
        }
    }
}
pub mod fuzz_example3 {
    use super::*;
    pub fn init_vesting(ctx: Context<InitVesting>, recipient: Pubkey,
        _recipient: anchor_lang::prelude::Pubkey, amount: u64, start_at: u64,
        end_at: u64, interval: u64) -> Result<()> {
        _init_vesting(ctx, recipient, amount, start_at, end_at, interval)
    }
    pub fn withdraw_unlocked(ctx: Context<Withdraw>) -> Result<()> {
        _withdraw_unlocked(ctx)
    }
    pub fn withdraw_dummy(ctx: Context<Withdraw>) -> Result<()> { Ok(()) }
}
#[doc = r" An Anchor generated module containing the program's set of"]
#[doc =
r" instructions, where each method handler in the `#[program]` mod is"]
#[doc = r" associated with a struct defining the input arguments to the"]
#[doc =
r" method. These should be used directly, when one wants to serialize"]
#[doc = r" Anchor instruction data, for example, when speciying"]
#[doc = r" instructions on a client."]
pub mod instruction {
    use super::*;
    #[doc = r" Instruction."]
    pub struct InitVesting {
        pub recipient: Pubkey,
        pub _recipient: anchor_lang::prelude::Pubkey,
        pub amount: u64,
        pub start_at: u64,
        pub end_at: u64,
        pub interval: u64,
    }
    impl borsh::ser::BorshSerialize for InitVesting where
        Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::prelude::Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize, u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize, u64: borsh::ser::BorshSerialize {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.recipient, writer)?;
            borsh::BorshSerialize::serialize(&self._recipient, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            borsh::BorshSerialize::serialize(&self.start_at, writer)?;
            borsh::BorshSerialize::serialize(&self.end_at, writer)?;
            borsh::BorshSerialize::serialize(&self.interval, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitVesting where
        Pubkey: borsh::BorshDeserialize,
        anchor_lang::prelude::Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize, u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize, u64: borsh::BorshDeserialize {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                    recipient: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    _recipient: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    start_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    end_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    interval: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
        }
    }
    impl anchor_lang::Discriminator for InitVesting {
        const DISCRIMINATOR: [u8; 8] = [119, 192, 67, 41, 47, 82, 152, 27];
    }
    impl anchor_lang::InstructionData for InitVesting {}
    impl anchor_lang::Owner for InitVesting {
        fn owner() -> Pubkey { ID }
    }
    #[doc = r" Instruction."]
    pub struct WithdrawUnlocked;
    impl borsh::ser::BorshSerialize for WithdrawUnlocked {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WithdrawUnlocked {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for WithdrawUnlocked {
        const DISCRIMINATOR: [u8; 8] = [213, 161, 76, 199, 38, 28, 209, 80];
    }
    impl anchor_lang::InstructionData for WithdrawUnlocked {}
    impl anchor_lang::Owner for WithdrawUnlocked {
        fn owner() -> Pubkey { ID }
    }
    #[doc = r" Instruction."]
    pub struct WithdrawDummy;
    impl borsh::ser::BorshSerialize for WithdrawDummy {
        fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W)
            -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WithdrawDummy {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R)
            -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for WithdrawDummy {
        const DISCRIMINATOR: [u8; 8] = [117, 156, 173, 123, 159, 55, 55, 150];
    }
    impl anchor_lang::InstructionData for WithdrawDummy {}
    impl anchor_lang::Owner for WithdrawDummy {
        fn owner() -> Pubkey { ID }
    }
}
#[doc = r" An Anchor generated module, providing a set of structs"]
#[doc = r" mirroring the structs deriving `Accounts`, where each field is"]
#[doc = r" a `Pubkey`. This is useful for specifying accounts for a client."]
pub mod accounts {
    pub use crate::__client_accounts_withdraw::*;
    pub use crate::__client_accounts_init_vesting::*;
}
