use proc_macro::TokenStream;
use syn::{ItemFn, spanned::Spanned,};

#[proc_macro]
pub fn program(input: TokenStream) -> TokenStream {
    input
}
