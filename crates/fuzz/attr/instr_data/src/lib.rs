extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn instr_data(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    // Use the path to the re-exported arbitrary
    let expanded = quote! {
        #[cfg(feature = "trident-fuzzing")]
        #[cfg(target_os = "solana")]
        compile_error!("Do not use fuzzing with Production Code");
        #[cfg(feature = "trident-fuzzing")]
        #[derive(Debug, arbitrary::Arbitrary, Clone)]
        #input
        #[cfg(not(feature = "trident-fuzzing"))]
        #input
    };

    TokenStream::from(expanded)
}
