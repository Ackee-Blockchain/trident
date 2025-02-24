use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::DeriveInput;
use trident_syn::parser::trident_fuzz_test_executor::parse_trident_fuzz_test_executor;

#[proc_macro_derive(FuzzTestExecutor)]
pub fn trident_fuzz_test_executor(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);

    match parse_trident_fuzz_test_executor(&item) {
        Ok(executor) => executor.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
