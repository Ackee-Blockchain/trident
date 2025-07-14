use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use crate::types::trident_fuzz_test_methods::TridentFuzzTestMethodsStruct;

impl ToTokens for TridentFuzzTestMethodsStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_name = &self.ident;
        let client_field = &self.client_field;
        let metrics_field = &self.metrics_field;
        let rng_field = &self.rng_field;

        let expanded = quote! {
            impl FuzzTestGetters for #struct_name {
                fn get_client(&mut self) -> &mut TridentSVM {
                    &mut self.#client_field
                }

                fn get_metrics(&mut self) -> &mut FuzzingStatistics {
                    &mut self.#metrics_field
                }

                fn get_rng(&self) -> &TridentRng {
                    &self.#rng_field
                }
            }

            impl FuzzTestExecutor for #struct_name {}
        };

        tokens.extend(expanded);
    }
}
