use anchor_syn::{AccountField, AccountTy};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::{Ident, ItemStruct, Result as ParseResult};

#[proc_macro_derive(AccountsSnapshots)]
pub fn derive_accounts_snapshots(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(item as TridentAccountsStruct)
        .to_token_stream()
        .into()
}

struct TridentAccountsStruct(anchor_syn::AccountsStruct);

impl Parse for TridentAccountsStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let strct = <ItemStruct as Parse>::parse(input)?;
        // TODO make sure that these convertions between types are correct
        Ok(TridentAccountsStruct(anchor_syn::parser::accounts::parse(
            &strct,
        )?))
    }
}

fn snapshot_field(field: &anchor_syn::Field, is_optional: bool) -> proc_macro2::TokenStream {
    let account_ty = field.account_ty();
    let container_ty = field.container_ty();

    let inner_ty = match &field.ty {
        anchor_syn::Ty::AccountInfo => {
            quote! {
               &'info AccountInfo<'info>
            }
        }
        anchor_syn::Ty::UncheckedAccount => {
            quote! {
               UncheckedAccount<'info>
            }
        }
        anchor_syn::Ty::AccountLoader(_) => {
            quote! {
                #container_ty<'info,#account_ty>
            }
        }
        anchor_syn::Ty::Sysvar(_) => {
            quote! {
               Sysvar<'info,#account_ty>
            }
        }
        anchor_syn::Ty::Account(AccountTy { boxed, .. }) => {
            // Verbously say that if the account is boxed we dont care.
            #[allow(clippy::if_same_then_else)]
            if *boxed {
                quote! {
                    #container_ty<'info,#account_ty>
                }
            } else {
                quote! {
                    #container_ty<'info,#account_ty>
                }
            }
        }
        anchor_syn::Ty::Program(_) => {
            quote! {
                #container_ty<'info,#account_ty>
            }
        }
        anchor_syn::Ty::Interface(_) => {
            quote! {
                #container_ty<'info,#account_ty>
            }
        }
        anchor_syn::Ty::InterfaceAccount(_) => {
            quote! {
                #container_ty<'info,#account_ty>
            }
        }
        anchor_syn::Ty::Signer => {
            quote! {
               Signer<'info>
            }
        }
        anchor_syn::Ty::SystemAccount => {
            quote! {
               SystemAccount<'info>
            }
        }
        anchor_syn::Ty::ProgramData => {
            todo!()
        }
    };
    let f_name = &field.ident;

    if is_optional {
        quote! {
            #f_name:Option<#inner_ty>
        }
    } else {
        quote! {
            #f_name:#inner_ty
        }
    }
}
fn type_decl_try_from(field: &anchor_syn::Field) -> proc_macro2::TokenStream {
    let _account_ty = field.account_ty();
    let _container_ty = field.container_ty();

    let inner_ty = match &field.ty {
        anchor_syn::Ty::AccountInfo => {
            quote! {}
        }
        anchor_syn::Ty::UncheckedAccount => {
            quote! {
               anchor_lang::accounts::unchecked_account::UncheckedAccount
            }
        }
        anchor_syn::Ty::AccountLoader(_) => {
            quote! {
                anchor_lang::accounts::account_loader::AccountLoader
            }
        }
        anchor_syn::Ty::Sysvar(_) => {
            quote! {
                anchor_lang::accounts::sysvar::Sysvar
            }
        }
        anchor_syn::Ty::Account(AccountTy { boxed, .. }) => {
            // Verbously say that if the account is boxed we dont care.
            #[allow(clippy::if_same_then_else)]
            if *boxed {
                quote! {
                    anchor_lang::accounts::account::Account
                }
            } else {
                quote! {
                    anchor_lang::accounts::account::Account
                }
            }
        }
        anchor_syn::Ty::Program(_) => {
            quote! {
               anchor_lang::accounts::program::Program
            }
        }
        anchor_syn::Ty::Interface(_) => {
            quote! {
                anchor_lang::accounts::interface::Interface
            }
        }
        anchor_syn::Ty::InterfaceAccount(_) => {
            quote! {
                anchor_lang::accounts::interface_account::InterfaceAccount
            }
        }
        anchor_syn::Ty::Signer => {
            quote! {
               anchor_lang::accounts::signer::Signer
            }
        }
        anchor_syn::Ty::SystemAccount => {
            quote! {
                anchor_lang::accounts::system_account::SystemAccount
            }
        }
        anchor_syn::Ty::ProgramData => {
            quote! {}
        }
    };
    quote! {
        #inner_ty
    }
}

fn type_decl_deserialize(field: &anchor_syn::Field, is_optional: bool) -> proc_macro2::TokenStream {
    let name = &field.ident;
    let account_ty = field.account_ty();
    let container_ty = field.container_ty();

    let ty_decl = match &field.ty {
        anchor_syn::Ty::AccountInfo => {
            quote! {
                AccountInfo
            }
        }
        anchor_syn::Ty::UncheckedAccount => {
            quote! {
                UncheckedAccount
            }
        }
        anchor_syn::Ty::AccountLoader(_) => {
            quote! {
                anchor_lang::accounts::account_loader::AccountLoader<#account_ty>
            }
        }
        anchor_syn::Ty::Sysvar(_) => {
            quote! {
                Sysvar<#account_ty>
            }
        }
        anchor_syn::Ty::Account(AccountTy { boxed, .. }) => {
            // Verbously say that if the account is boxed we dont care.
            #[allow(clippy::if_same_then_else)]
            if *boxed {
                quote! {
                    #container_ty<#account_ty>
                }
            } else {
                quote! {
                    #container_ty<#account_ty>
                }
            }
        }
        anchor_syn::Ty::Program(_) => {
            quote! {
                #container_ty<#account_ty>
            }
        }
        anchor_syn::Ty::Interface(_) => {
            quote! {
                anchor_lang::accounts::interface::Interface<#account_ty>
            }
        }
        anchor_syn::Ty::InterfaceAccount(_) => {
            quote! {
                anchor_lang::accounts::interface_account::InterfaceAccount<#account_ty>
            }
        }
        anchor_syn::Ty::Signer => {
            quote! {
                Signer
            }
        }
        anchor_syn::Ty::SystemAccount => {
            quote! {
                SystemAccount
            }
        }
        anchor_syn::Ty::ProgramData => {
            quote! {}
        }
    };
    if is_optional {
        quote! {
            #name: Option<#ty_decl>
        }
    } else {
        quote! {
            #name: #ty_decl
        }
    }
}

impl From<&TridentAccountsStruct> for TokenStream {
    fn from(accounts: &TridentAccountsStruct) -> Self {
        generate(accounts)
    }
}

impl ToTokens for TridentAccountsStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

fn deserialize_option_account(
    typed_name: TokenStream,
    ty_decl: TokenStream,
    f_name_as_string: String,
    is_optional: bool,
) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
        let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    #ty_decl::try_from(acc).map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))
                } else {Err(trident_fuzz::error::FuzzingError::OptionalAccountNotProvided(
                        #f_name_as_string.to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        }
    } else {
        quote! {
            let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(#ty_decl::try_from)
            .ok_or(trident_fuzz::error::FuzzingError::AccountNotFound(#f_name_as_string.to_string()))?
            // TODO It would be helpful to do something like line below.
            // where we propagate anchor error
            // However I suggest that this is not possible right now as for
            // fuzz_example3 the anchor_lang has version 0.28.0. However trident
            // uses 0.29.0 I think this is the reason why the '?' operator cannot propagate
            // the error even though I implemnted From<anchor_lang::error::Error> trait
            // that i
            // .map_err(|e| e.with_account_name(#name_str).into())?;
            .map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))?;
        }
    }
}

fn deserialize_option_account_info(
    f_name: &Ident,
    f_name_as_string: String,
    is_optional: bool,
) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
            let #f_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref();
        }
    } else {
        quote! {
            let #f_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .ok_or(trident_fuzz::error::FuzzingError::AccountNotFound(#f_name_as_string.to_string()))?;
        }
    }
}
fn deserialize_option_unchecked_account(
    f_name: &Ident,
    f_name_as_string: String,
    ty_decl: TokenStream,
    is_optional: bool,
) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
            let #f_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(#ty_decl::try_from);
        }
    } else {
        quote! {
            let #f_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(#ty_decl::try_from)
            .ok_or(trident_fuzz::error::FuzzingError::AccountNotFound(#f_name_as_string.to_string()))?;
        }
    }
}
// TODO optional ?
fn deserialize_option_program(
    typed_name: TokenStream,
    ty_decl: TokenStream,
    f_name_as_string: String,
    is_optional: bool,
) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
        let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    #ty_decl::try_from(acc).map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))
                } else {Err(trident_fuzz::error::FuzzingError::OptionalAccountNotProvided(
                        #f_name_as_string.to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        }
    } else {
        quote! {
            let #typed_name = accounts_iter
                .next()
                .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
                .as_ref()
                .map(#ty_decl::try_from)
                .ok_or(trident_fuzz::error::FuzzingError::AccountNotFound(#f_name_as_string.to_string()))?
                .map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))?;
        }
    }
}

// TODO optional ?
fn deserialize_option_signer(
    typed_name: TokenStream,
    ty_decl: TokenStream,
    f_name_as_string: String,
    is_optional: bool,
) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
        let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    #ty_decl::try_from(acc).map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))
                } else {Err(trident_fuzz::error::FuzzingError::OptionalAccountNotProvided(
                        #f_name_as_string.to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        }
    } else {
        quote! {
            let #typed_name = accounts_iter
                .next()
                .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
                .as_ref()
                .map(#ty_decl::try_from)
                .ok_or(trident_fuzz::error::FuzzingError::AccountNotFound(#f_name_as_string.to_string()))?
                .map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))?;
        }
    }
}

fn deserialize_option_sysvar(
    typed_name: TokenStream,
    ty_decl: TokenStream,
    f_name_as_string: String,
    is_optional: bool,
) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
        let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    #ty_decl::from_account_info(acc).map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))
                } else {Err(trident_fuzz::error::FuzzingError::OptionalAccountNotProvided(
                        #f_name_as_string.to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        }
    } else {
        quote! {
            let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(#ty_decl::from_account_info)
            .ok_or(trident_fuzz::error::FuzzingError::AccountNotFound(#f_name_as_string.to_string()))?
            .map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))?;
        }
    }
}

fn deserialize_option_interface(
    typed_name: TokenStream,
    ty_decl: TokenStream,
    f_name_as_string: String,
    is_optional: bool,
) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
        let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    #ty_decl::try_from(acc).map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))
                } else {Err(trident_fuzz::error::FuzzingError::OptionalAccountNotProvided(
                        #f_name_as_string.to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        }
    } else {
        quote! {
            let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(#ty_decl::try_from)
            .ok_or(trident_fuzz::error::FuzzingError::AccountNotFound(#f_name_as_string.to_string()))?
            .map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))?;
        }
    }
}

fn deserialize_option_interface_account(
    typed_name: TokenStream,
    ty_decl: TokenStream,
    f_name_as_string: String,
    is_optional: bool,
) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
        let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    #ty_decl::try_from(acc).map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))
                } else {Err(trident_fuzz::error::FuzzingError::OptionalAccountNotProvided(
                        #f_name_as_string.to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        }
    } else {
        quote! {
            let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(#ty_decl::try_from)
            .ok_or(trident_fuzz::error::FuzzingError::AccountNotFound(#f_name_as_string.to_string()))?
            .map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))?;
        }
    }
}

fn deserialize_option_system_account(
    typed_name: TokenStream,
    ty_decl: TokenStream,
    f_name_as_string: String,
    is_optional: bool,
) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
        let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    #ty_decl::try_from(acc).map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))
                } else {Err(trident_fuzz::error::FuzzingError::OptionalAccountNotProvided(
                        #f_name_as_string.to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        }
    } else {
        quote! {
            let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(#ty_decl::try_from)
            .ok_or(trident_fuzz::error::FuzzingError::AccountNotFound(#f_name_as_string.to_string()))?
            .map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))?;
        }
    }
}

fn deserialize_option_account_loader(
    typed_name: TokenStream,
    ty_decl: TokenStream,
    f_name_as_string: String,
    is_optional: bool,
) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
        let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    #ty_decl::try_from(acc).map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))
                } else {Err(trident_fuzz::error::FuzzingError::OptionalAccountNotProvided(
                        #f_name_as_string.to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        }
    } else {
        quote! {
            let #typed_name = accounts_iter
            .next()
            .ok_or(trident_fuzz::error::FuzzingError::NotEnoughAccounts(#f_name_as_string.to_string()))?
            .as_ref()
            .map(#ty_decl::try_from)
            .ok_or(trident_fuzz::error::FuzzingError::AccountNotFound(#f_name_as_string.to_string()))?
            .map_err(|_| trident_fuzz::error::FuzzingError::CannotDeserializeAccount(#f_name_as_string.to_string()))?;
        }
    }
}

fn generate(accs: &TridentAccountsStruct) -> proc_macro2::TokenStream {
    let context_name = &accs.0.ident;
    let snapshot_name = syn::Ident::new(&format!("{}Snapshot", context_name), context_name.span());
    let module_name = syn::Ident::new(
        &format!("trident_fuzz_{}_snapshot", context_name),
        context_name.span(),
    );

    // CONSTRUCT DESERIALIZE OPTION
    let deserialize_fields = accs.0.fields.iter().map(|field| {
        let is_optional = is_optional(field);
        match &field {
            anchor_syn::AccountField::Field(field) => {
                let f_name = &field.ident;
                let f_name_as_string = f_name.to_string();
                let typed_name = type_decl_deserialize(field, is_optional);
                let ty_decl = type_decl_try_from(field);

                match field.ty {
                    anchor_syn::Ty::AccountInfo => {
                        deserialize_option_account_info(f_name, f_name_as_string, is_optional)
                    }
                    anchor_syn::Ty::UncheckedAccount => deserialize_option_unchecked_account(
                        f_name,
                        f_name_as_string,
                        ty_decl,
                        is_optional,
                    ),
                    anchor_syn::Ty::AccountLoader(_) => deserialize_option_account_loader(
                        typed_name,
                        ty_decl,
                        f_name_as_string,
                        is_optional,
                    ),
                    anchor_syn::Ty::Sysvar(_) => deserialize_option_sysvar(
                        typed_name,
                        ty_decl,
                        f_name_as_string,
                        is_optional,
                    ),
                    anchor_syn::Ty::Account(_) => deserialize_option_account(
                        typed_name,
                        ty_decl,
                        f_name_as_string,
                        is_optional,
                    ),
                    anchor_syn::Ty::Program(_) => deserialize_option_program(
                        typed_name,
                        ty_decl,
                        f_name_as_string,
                        is_optional,
                    ),
                    anchor_syn::Ty::Interface(_) => deserialize_option_interface(
                        typed_name,
                        ty_decl,
                        f_name_as_string,
                        is_optional,
                    ),
                    anchor_syn::Ty::InterfaceAccount(_) => deserialize_option_interface_account(
                        typed_name,
                        ty_decl,
                        f_name_as_string,
                        is_optional,
                    ),
                    anchor_syn::Ty::Signer => deserialize_option_signer(
                        typed_name,
                        ty_decl,
                        f_name_as_string,
                        is_optional,
                    ),
                    anchor_syn::Ty::SystemAccount => deserialize_option_system_account(
                        typed_name,
                        ty_decl,
                        f_name_as_string,
                        is_optional,
                    ),
                    anchor_syn::Ty::ProgramData => todo!(),
                }
            }
            anchor_syn::AccountField::CompositeField(_) => todo!(),
        }
    });

    // CONSTRUCT SNAPSHOT STRUCT
    let snapshot_fields = accs.0.fields.iter().map(|field| {
        let is_optional = is_optional(field);

        let snapshot_field = match &field {
            anchor_syn::AccountField::Field(field) => snapshot_field(field, is_optional),
            anchor_syn::AccountField::CompositeField(_composite) => todo!(),
        };
        quote! {
            pub #snapshot_field,
        }
    });

    // CONSTRUCT RETURN VALUE
    let struct_fields = accs.0.fields.iter().map(|field| {
        let field_name = match &field {
            anchor_syn::AccountField::Field(field) => field.ident.to_owned(),
            anchor_syn::AccountField::CompositeField(_composite) => todo!(),
        };
        quote! { #field_name }
    });

    quote! {
        #[cfg(feature = "trident-fuzzing")]
        pub mod #module_name{
            #[cfg(target_os = "solana")]
            compile_error!("Do not use fuzzing with Production Code");
            use super::*;
            impl<'info> #snapshot_name<'info> {
                pub fn deserialize_option(
                    _program_id: &anchor_lang::prelude::Pubkey,
                    accounts: &'info mut [Option<AccountInfo<'info>>],
                ) -> core::result::Result<Self, trident_fuzz::error::FuzzingError> {
                    let mut accounts_iter = accounts.iter();

                    #(#deserialize_fields)*

                    Ok(Self {
                        #(#struct_fields),*
                    })
                }
            }
            pub struct #snapshot_name<'info> {
                #(#snapshot_fields)*
            }
        }
    }
}

/// Determines if an Account should be wrapped into the `Option` type.
/// The function returns true if the account has the init or close constraints set
/// or if it is wrapped into the `Option` type.
fn is_optional(parsed_field: &AccountField) -> bool {
    let is_optional = match parsed_field {
        AccountField::Field(field) => field.is_optional,
        AccountField::CompositeField(_) => false,
    };
    let constraints = match parsed_field {
        AccountField::Field(f) => &f.constraints,
        AccountField::CompositeField(f) => &f.constraints,
    };

    constraints.init.is_some() || constraints.is_close() || is_optional || constraints.is_zeroed()
}
