use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemStruct};
use trident_syn::parser::trident_transaction::parse_trident_transaction;

#[proc_macro_derive(TridentTransaction, attributes(name))]
pub fn derive_trident_transaction(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);

    match parse_trident_transaction(&item_struct) {
        Ok(transaction) => transaction.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
