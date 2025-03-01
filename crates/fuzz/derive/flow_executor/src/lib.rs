use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn, ItemImpl};
use trident_syn::parser::trident_flow_executor::parse_trident_flow_executor;

/// Marks a method to be executed as part of the fuzzing flow
#[proc_macro_attribute]
pub fn flow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    quote::quote!(#input_fn).into()
}

/// Marks a method to run once before any flow methods
#[proc_macro_attribute]
pub fn init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    quote::quote!(#input_fn).into()
}

/// Marks a flow method to be skipped during execution
#[proc_macro_attribute]
pub fn flow_ignore(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    quote::quote!(#input_fn).into()
}

/// Implements the flow executor for a struct
#[proc_macro_attribute]
pub fn flow_executor(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let attr_tokens: proc_macro2::TokenStream = attr.into();

    match parse_trident_flow_executor(attr_tokens, &input) {
        Ok(executor) => executor.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
