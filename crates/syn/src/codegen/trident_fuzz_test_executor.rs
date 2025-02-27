use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_fuzz_test_executor::TridentFuzzTestExecutor;

impl ToTokens for TridentFuzzTestExecutor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let generics = &self.generics;
        let where_clause = &self.where_clause;

        let expanded = quote! {
            impl<#(#generics),*> #name<#(#generics),*> #where_clause {
                fn fuzz() {
                    let mut _self = Self {};

                    let config = TridentConfig::new();
                    let mut client = TridentSVM::new_client(Some(&config));

                    if cfg!(honggfuzz) {
                        loop {
                            fuzz_honggfuzz(|fuzzer_data| {
                                let mut buf = Unstructured::new(fuzzer_data);
                                let res = _self.execute_flows(&mut buf, &mut client);
                                client.clear_accounts();
                            });
                        }
                    } else if cfg!(afl) {
                        fuzz_afl(true, |fuzzer_data| {
                            let mut buf = Unstructured::new(fuzzer_data);
                            let res = _self.execute_flows(&mut buf, &mut client);
                            client.clear_accounts();
                        });
                    } else if cfg!(honggfuzz_debug) {
                        let mut crash_file = String::new();
                        std::io::stdin()
                            .read_line(&mut crash_file)
                            .expect("Failed to read crash file path from stdin");
                        let crash_file = crash_file.trim();

                        let fuzzer_data = std::fs::read(crash_file).expect("Failed to read crash file");

                        let mut buf = Unstructured::new(&fuzzer_data);
                        let res = _self.execute_flows(&mut buf, &mut client);
                        client.clear_accounts();
                    } else {
                        panic!("Select Honggfuzz or AFL for fuzzing!!!")
                    }
                }
            }
        };

        tokens.extend(expanded);
    }
}
