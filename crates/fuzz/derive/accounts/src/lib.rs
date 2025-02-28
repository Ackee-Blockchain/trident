use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemStruct};

use trident_syn::parser::trident_accounts::parse_trident_accounts;

#[proc_macro_derive(TridentAccounts, attributes(account, instruction_data, storage))]
pub fn derive_trident_accounts(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);

    match parse_trident_accounts(&item_struct) {
        Ok(accounts) => accounts.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
