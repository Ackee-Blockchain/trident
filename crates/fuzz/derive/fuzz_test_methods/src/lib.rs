use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::DeriveInput;
use trident_syn::parser::trident_fuzz_test_methods::parse_trident_fuzz_test_methods;

#[proc_macro_derive(FuzzTestMethods)]
pub fn derive_fuzz_test_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match parse_trident_fuzz_test_methods(&input) {
        Ok(fuzz_test_methods) => fuzz_test_methods.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
