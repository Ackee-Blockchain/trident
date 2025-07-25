use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::types::trident_fuzz_test_methods::TridentFuzzTestMethodsStruct;

impl ToTokens for TridentFuzzTestMethodsStruct {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        // We dont need to do anything here, kept for future use
    }
}
