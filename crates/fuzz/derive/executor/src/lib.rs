use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn, ItemImpl};
use trident_syn::parser::trident_flow_executor::parse_trident_flow_executor;

#[proc_macro_attribute]
pub fn flow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    quote::quote!(#input_fn).into()
}

#[proc_macro_attribute]
pub fn init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    quote::quote!(#input_fn).into()
}

#[proc_macro_attribute]
pub fn flow_ignore(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    quote::quote!(#input_fn).into()
}

/// Flow executor that can optionally shuffle the execution order of flow methods
///
/// Usage:
/// ```rust
/// #[flow_executor(shuffle)]  // or #[flow_executor(shuffle = true)]
/// impl MyTest {
///     #[init]
///     fn start(&mut self, client: &mut impl FuzzClient) {
///         client.deploy();
///     }
///
///     #[flow]
///     fn test1(&mut self) -> Result<(), arbitrary::Error> { Ok(()) }
///
///     #[flow]
///     #[ignore]  // This method will be skipped
///     fn test2(&mut self) -> Result<(), arbitrary::Error> { Ok(()) }
/// }
/// ```
#[proc_macro_attribute]
pub fn flow_executor(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let attr_tokens: proc_macro2::TokenStream = attr.into();

    match parse_trident_flow_executor(attr_tokens, &input) {
        Ok(executor) => executor.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
