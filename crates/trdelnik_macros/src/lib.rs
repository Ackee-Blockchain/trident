use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{Item, ItemMod, FnArg, PatType, Type, TypePath, PathArguments, AngleBracketedGenericArguments, spanned::Spanned};
use quote::{ToTokens, quote, quote_spanned};
use heck::ToUpperCamelCase;

// #[trdelnik_macros::program]
// #[program]
// pub mod turnstile {
//     use super::*;
//
//     pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
//         // ..    
//         Ok(())
//     }
//    
//     pub fn coin(ctx: Context<UpdateState>) -> ProgramResult {
//         // ..    
//         Ok(())
//     }
// }
//
// to
//
// #[program]
// pub mod turnstile {
//     use super::*;
//
//     pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
//         // ..    
//         Ok(())
//     }
//    
//     pub fn coin(ctx: Context<UpdateState>) -> ProgramResult {
//         // ..    
//         Ok(())
//     }
// }
//
// pub mod ix_with_accounts {
//     pub struct Initialize {
//         pub instruction: super::instruction::Initialize, 
//         pub accounts: super::accounts::Initialize,
//     }
//     pub struct Coin {
//         pub instruction: super::instruction::Coin,
//         pub accounts: super::accounts::UpdateState,
//     }
// }
#[proc_macro_attribute]
pub fn program(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> TokenStream {
    let item_mod: ItemMod = syn::parse(input)
        .expect("'trdelnik_macros::program' failed to parse Anchor program module");

    if item_mod.attrs.iter().all(|attr| !attr.path.is_ident("program")) {
        panic!("'trdelnik_macros::program' can't find attribute '#[program]'");
    }

    let mod_items = if let Some((_, items)) = &item_mod.content {
        items
    } else {
        return item_mod.into_token_stream().into();
    };

    let instruction_structs = mod_items.iter().filter_map(|item| {
        let item_fn = if let Item::Fn(item_fn) = item {
            item_fn
        } else {
            None?
        };
        let accounts_name = item_fn.sig.inputs.iter().find_map(|fn_arg| {
            let arg_type = if let FnArg::Typed(PatType { ty, .. }) = fn_arg {
                ty.as_ref()
            } else {
                None?
            };
            let arg_type_path = if let Type::Path(TypePath { path, .. }) = arg_type {
                path
            } else {
                None?
            };
            let arg_type_path_segment = arg_type_path.segments.first()?;
            if arg_type_path_segment.ident != "Context" {
                None?
            }
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = &arg_type_path_segment.arguments {
                Some(Ident::new(&args.first()?.to_token_stream().to_string(), Span::call_site()))
            } else {
                None
            }
        })?;
        let instruction_name = Ident::new(&item_fn.sig.ident.to_string().to_upper_camel_case(), Span::call_site());
        let instruction_struct = quote!(
            pub struct #instruction_name {
                pub instruction: super::instruction::#instruction_name,
                pub accounts: super::accounts::#accounts_name,
            }
        );
        Some(instruction_struct)
    }).collect::<Vec<_>>();

    let item_mod_span = item_mod.span();
    quote_spanned!(item_mod_span=>
        #item_mod
        pub mod ix_with_accounts {
            #(#instruction_structs)*
        }
    )
    .into()
}
