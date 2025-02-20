use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};
use trident_syn::parser::trident_remaining_accounts::parse_trident_remaining_accounts;

#[proc_macro_derive(TridentRemainingAccounts)]
pub fn derive_trident_remaining_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match parse_trident_remaining_accounts(&input) {
        Ok(remaining_accounts) => remaining_accounts.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
