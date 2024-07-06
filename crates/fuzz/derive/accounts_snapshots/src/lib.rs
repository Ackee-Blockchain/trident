use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_derive(AccountsSnapshots)]
pub fn derive_accounts_snapshots(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as trident_fuzz::trident_accounts_struct::TridentAccountsStruct)
        .to_token_stream()
        .into()
}
