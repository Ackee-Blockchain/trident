use std::{error::Error, fs::File, io::Read};

use cargo_metadata::camino::Utf8PathBuf;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Error as ParseError, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{
    parse_quote, Attribute, Fields, GenericArgument, Item, ItemStruct, PathArguments, TypePath,
};

pub fn generate_snapshots_code(code_path: Vec<(String, Utf8PathBuf)>) -> Result<String, String> {
    let code = code_path.iter().map(|(code, path)| {
        let mut mod_program = None::<syn::ItemMod>;
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| e.to_string())?;

        let parse_result = syn::parse_file(&content).map_err(|e| e.to_string())?;

        // locate the program module to extract instructions and corresponding Context structs
        for item in parse_result.items.iter() {
            if let Item::Mod(module) = item {
                // Check if the module has the #[program] attribute
                if has_program_attribute(&module.attrs) {
                    mod_program = Some(module.clone())
                }
            }
        }

        let mod_program = mod_program.ok_or("module with program attribute not found")?;

        let (_, items) = mod_program
            .content
            .ok_or("the content of program module is missing")?;

        let mut ix_ctx_pairs = Vec::new();
        for item in items {
            // Iterate through items in program module and find functions with the Context<_> parameter. Save the function name and the Context's inner type.
            if let syn::Item::Fn(func) = item {
                let func_name = &func.sig.ident;
                let first_param_type = if let Some(param) = func.sig.inputs.into_iter().next() {
                    let mut ty = None::<GenericArgument>;
                    if let syn::FnArg::Typed(t) = param {
                        if let syn::Type::Path(tp) = *t.ty.clone() {
                            if let Some(seg) = tp.path.segments.into_iter().next() {
                                if let PathArguments::AngleBracketed(arg) = seg.arguments {
                                    ty = arg.args.first().cloned();
                                }
                            }
                        }
                    }
                    ty
                } else {
                    None
                };

                let first_param_type = first_param_type.ok_or(format!(
                    "The function {} does not have the Context parameter and is malformed.",
                    func_name
                ))?;

                ix_ctx_pairs.push((func_name.clone(), first_param_type));
            }
        }

        // Find definition of each Context struct and create new struct with fields wrapped in Option<_>
        let mut structs = String::new();
        let mut desers = String::new();
        let parse_result = syn::parse_file(code).map_err(|e| e.to_string())?;
        for pair in ix_ctx_pairs {
            let mut ty = None;
            if let GenericArgument::Type(syn::Type::Path(tp)) = &pair.1 {
                ty = tp.path.get_ident().cloned();
                // TODO add support for types with fully qualified path such as ix::Initialize
            }
            let ty = ty.ok_or(format!("malformed parameters of {} instruction", pair.0))?;
            // println!("{} - {}", pair.0, ty);

            // recursively find the context struct and create a new version with wrapped field into Option
            if let Some(ctx) = get_ctx_struct(&parse_result.items, &ty) {
                let wrapped_struct = wrap_fields_in_option(ctx).unwrap();
                // println!("{}", wrapped_struct);
                let deser_code = deserialize_ctx_struct(ctx).unwrap();
                // println!("{}", deser_code);
                structs = format!("{}{}", structs, wrapped_struct.into_token_stream());
                desers = format!("{}{}", desers, deser_code.into_token_stream());
            } else {
                return Err(format!("The Context struct {} was not found", ty));
            }
        }
        let use_statements = quote! {
            use trdelnik_client::anchor_lang::{prelude::*, self};
            use trdelnik_client::anchor_lang::solana_program::instruction::AccountMeta;
            use trdelnik_client::fuzzing::{get_account_infos_option, FuzzingError};
        }
        .into_token_stream();
        Ok(format!("{}{}{}", use_statements, structs, desers))
    });

    code.into_iter().collect()
}

/// Recursively find a struct with a given `name`
fn get_ctx_struct<'a>(items: &'a Vec<syn::Item>, name: &'a syn::Ident) -> Option<&'a ItemStruct> {
    for item in items {
        if let Item::Struct(struct_item) = item {
            if struct_item.ident == *name {
                // println!("we found the struct {}", name);
                return Some(struct_item);
            }
        }
    }

    // if the ctx struct is not found on the first level, recursively continue to search in submodules
    for item in items {
        if let Item::Mod(mod_item) = item {
            if let Some((_, items)) = &mod_item.content {
                let r = get_ctx_struct(items, name);
                if r.is_some() {
                    return r;
                }
            };
        }
    }

    None
}

fn wrap_fields_in_option(orig_struct: &ItemStruct) -> Result<TokenStream, Box<dyn Error>> {
    let struct_name = format_ident!("{}Snapshot", orig_struct.ident);
    let wrapped_fields = match orig_struct.fields.clone() {
        Fields::Named(named) => {
            let field_wrappers = named.named.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                quote! {
                    pub #field_name: Option<#field_type>,
                }
            });

            quote! {
                { #(#field_wrappers)* }
            }
        }

        _ => return Err("Only structs with named fields are supported".into()),
    };

    // Generate the new struct with Option-wrapped fields
    let generated_struct: syn::ItemStruct = parse_quote! {
        pub struct #struct_name<'info> #wrapped_fields
    };

    Ok(generated_struct.to_token_stream())
}

fn deserialize_ctx_struct(orig_struct: &ItemStruct) -> Result<TokenStream, Box<dyn Error>> {
    let impl_name = format_ident!("{}Snapshot", orig_struct.ident);
    let names_deser_pairs = match orig_struct.fields.clone() {
        Fields::Named(named) => {
            let field_deser = named.named.iter().map(|field| {
                let field_name = match &field.ident {
                    Some(name) => name,
                    None => {
                        return Err(ParseError::new(
                            field.ident.span(),
                            "invalid account name given",
                        ))
                    }
                };
                let field_type = &field.ty;

                let path = match &field_type {
                    syn::Type::Path(ty_path) => ty_path.path.clone(),
                    _ => {
                        return Err(ParseError::new(
                            field_type.span(),
                            "invalid account type given",
                        ))
                    }
                };
                let id = path.segments[0].clone();
                // println!("field name: {}, type: {}", field_name, id.ident);
                let ty = match id.ident.to_string().as_str() {
                    "AccountInfo" => AnchorType::AccountInfo,
                    "Signer" => AnchorType::Signer,
                    "Account" => AnchorType::Account(parse_account_ty(&path)?),
                    "Program" => AnchorType::Program(parse_program_ty(&path)?), // TODO
                    _ => return Err(ParseError::new(id.span(), "invalid account type given")),
                };
                let deser_tokens = match ty.to_tokens() {
                    Some((return_type, deser_method)) => {
                        deserialize_tokens(field_name, return_type, deser_method)
                    }
                    None => acc_info_tokens(field_name),
                };
                Ok((
                    quote! {#field_name},
                    quote! {
                        #deser_tokens
                    },
                ))
            });
            let result: Result<Vec<(TokenStream, TokenStream)>, _> =
                field_deser.into_iter().collect();
            result
        }

        _ => return Err("Only structs with named fields are supported".into()),
    }?;

    let (names, fields_deser): (Vec<_>, Vec<_>) = names_deser_pairs.iter().cloned().unzip();

    let generated_deser_impl: syn::Item = parse_quote! {
        impl<'info> #impl_name<'info> {
            pub fn deserialize_option(
                metas: &'info [AccountMeta],
                accounts: &'info mut [Option<trdelnik_client::solana_sdk::account::Account>],
            ) -> core::result::Result<Self, FuzzingError> {
                let accounts = get_account_infos_option(accounts, metas)
                    .map_err(|_| FuzzingError::CannotGetAccounts)?;

                let mut accounts_iter = accounts.into_iter();

                #(#fields_deser)*

                Ok(Self {
                    #(#names),*
                })
            }
        }
    };

    Ok(generated_deser_impl.to_token_stream())
}

// TODO add all account types as in https://github.com/coral-xyz/anchor/blob/master/lang/syn/src/parser/accounts/mod.rs#L351
pub enum AnchorType {
    AccountInfo,
    Signer,
    Account(AccountTy),
    Program(ProgramTy),
}

impl AnchorType {
    pub fn to_tokens(&self) -> Option<(TokenStream, TokenStream)> {
        let (return_type, deser_method) = match self {
            AnchorType::AccountInfo => return None,
            AnchorType::Signer => (
                quote! { Signer<'_>},
                quote!(anchor_lang::accounts::signer::Signer::try_from(&acc)),
            ),
            AnchorType::Account(acc) => {
                let path = &acc.account_type_path;
                (
                    quote! { anchor_lang::accounts::account::Account<#path>},
                    quote! {anchor_lang::accounts::account::Account::try_from(&acc)},
                )
            }
            AnchorType::Program(prog) => {
                let path = &prog.account_type_path;
                (
                    quote! { anchor_lang::accounts::program::Program<#path>},
                    quote!(anchor_lang::accounts::program::Program::try_from(&acc)),
                )
            }
        };
        Some((return_type, deser_method))
    }
}

pub struct ProgramTy {
    // The struct type of the account.
    pub account_type_path: TypePath,
}

pub struct AccountTy {
    // The struct type of the account.
    pub account_type_path: TypePath,
    // True if the account has been boxed via `Box<T>`.
    pub boxed: bool,
}

fn deserialize_tokens(
    name: &syn::Ident,
    return_type: TokenStream,
    deser_method: TokenStream,
) -> TokenStream {
    quote! {
        let #name:Option<#return_type> = accounts_iter
        .next()
        .ok_or(FuzzingError::NotEnoughAccounts)?
        .map(|acc| #deser_method)
        .transpose()
        .map_err(|_| FuzzingError::CannotDeserializeAccount)?;
    }
}

fn acc_info_tokens(name: &syn::Ident) -> TokenStream {
    quote! {
        let #name = accounts_iter
        .next()
        .ok_or(FuzzingError::NotEnoughAccounts)?;
    }
}

fn parse_account_ty(path: &syn::Path) -> ParseResult<AccountTy> {
    let account_type_path = parse_account(path)?;
    let boxed = tts_to_string(path)
        .replace(' ', "")
        .starts_with("Box<Account<");
    Ok(AccountTy {
        account_type_path,
        boxed,
    })
}

fn parse_program_ty(path: &syn::Path) -> ParseResult<ProgramTy> {
    let account_type_path = parse_account(path)?;
    Ok(ProgramTy { account_type_path })
}

pub fn tts_to_string<T: quote::ToTokens>(item: T) -> String {
    let mut tts = proc_macro2::TokenStream::new();
    item.to_tokens(&mut tts);
    tts.to_string()
}

fn parse_account(mut path: &syn::Path) -> ParseResult<syn::TypePath> {
    let path_str = tts_to_string(path).replace(' ', "");
    if path_str.starts_with("Box<Account<") || path_str.starts_with("Box<InterfaceAccount<") {
        let segments = &path.segments[0];
        match &segments.arguments {
            syn::PathArguments::AngleBracketed(args) => {
                // Expected: <'info, MyType>.
                if args.args.len() != 1 {
                    return Err(ParseError::new(
                        args.args.span(),
                        "bracket arguments must be the lifetime and type",
                    ));
                }
                match &args.args[0] {
                    syn::GenericArgument::Type(syn::Type::Path(ty_path)) => {
                        path = &ty_path.path;
                    }
                    _ => {
                        return Err(ParseError::new(
                            args.args[1].span(),
                            "first bracket argument must be a lifetime",
                        ))
                    }
                }
            }
            _ => {
                return Err(ParseError::new(
                    segments.arguments.span(),
                    "expected angle brackets with a lifetime and type",
                ))
            }
        }
    }

    let segments = &path.segments[0];
    match &segments.arguments {
        syn::PathArguments::AngleBracketed(args) => {
            // Expected: <'info, MyType>.
            if args.args.len() != 2 {
                return Err(ParseError::new(
                    args.args.span(),
                    "bracket arguments must be the lifetime and type",
                ));
            }
            match &args.args[1] {
                syn::GenericArgument::Type(syn::Type::Path(ty_path)) => Ok(ty_path.clone()),
                _ => Err(ParseError::new(
                    args.args[1].span(),
                    "first bracket argument must be a lifetime",
                )),
            }
        }
        _ => Err(ParseError::new(
            segments.arguments.span(),
            "expected angle brackets with a lifetime and type",
        )),
    }
}

fn has_program_attribute(attrs: &Vec<Attribute>) -> bool {
    for attr in attrs {
        if attr.path.is_ident("program") {
            return true;
        }
    }
    false
}
