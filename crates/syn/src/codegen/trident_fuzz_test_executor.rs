use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_fuzz_test_executor::TridentFuzzTestExecutor;

impl ToTokens for TridentFuzzTestExecutor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let (impl_generics, ty_generics, _) = self.generics.split_for_impl();
        let client_type = &self.client_type;

        let expanded = quote! {
            impl #impl_generics #name #ty_generics {
                fn new(client: #client_type, config: TridentConfig) -> Self {
                    Self { client, config }
                }

                fn fuzz(&mut self) {
                    if cfg!(honggfuzz) {
                        loop {
                            fuzz_honggfuzz(|fuzzer_data| {
                                let mut buf = Unstructured::new(fuzzer_data);
                                let _ = self.execute_flows(&mut buf);
                                self.client.clear_accounts();
                            });
                        }
                    } else if cfg!(afl) {
                        fuzz_afl(true, |fuzzer_data| {
                            let mut buf = Unstructured::new(fuzzer_data);
                            let _ = self.execute_flows(&mut buf);
                            self.client.clear_accounts();
                        });
                    } else if cfg!(honggfuzz_debug) {
                        let mut crash_file = String::new();
                        std::io::stdin()
                            .read_line(&mut crash_file)
                            .expect("Failed to read crash file path from stdin");
                        let crash_file = crash_file.trim();

                        let fuzzer_data = std::fs::read(crash_file).expect("Failed to read crash file");
                        let mut buf = Unstructured::new(&fuzzer_data);
                        let _ = self.execute_flows(&mut buf);
                        self.client.clear_accounts();
                    } else {
                        panic!("Select Honggfuzz or AFL for fuzzing!!!")
                    }
                }
            }
        };

        tokens.extend(expanded);
    }
}
