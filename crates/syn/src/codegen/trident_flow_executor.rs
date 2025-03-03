use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_flow_executor::TridentFlowExecutorImpl;

impl ToTokens for TridentFlowExecutorImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let type_name = &self.type_name;
        let impl_items = &self.impl_block;
        let methods = &self.flow_methods;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let init_call = if let Some(init_method) = &self.init_method {
            quote! {
                self.#init_method();
            }
        } else {
            quote! {}
        };

        let execute_impl = if methods.is_empty() {
            quote! {
                self.default_random_transactions(fuzzer_data, &mut accounts)?;
            }
        } else {
            let random_tail = if self.args.random_tail {
                quote! {
                    self.default_random_transactions(fuzzer_data, &mut accounts)?;
                }
            } else {
                quote! {}
            };

            let flow_execution = if self.args.shuffle {
                // Global shuffle - shuffle all methods
                let indices: Vec<usize> = (0..methods.len()).collect();
                quote! {
                    let mut indices = vec![#(#indices),*];
                    indices.shuffle(&mut rand::thread_rng());

                    for &i in indices.iter() {
                        match i {
                            #(
                                #indices => self.#methods(fuzzer_data, &mut accounts)?,
                            )*
                            _ => unreachable!(),
                        }
                    }
                }
            } else {
                // Process methods in order, shuffling marked ones when encountered
                let mut current_shuffle_group = Vec::new();
                let mut execution_tokens = TokenStream::new();

                for method in &self.flow_methods {
                    if self.shuffled_methods.contains(method) {
                        // Add to current shuffle group
                        current_shuffle_group.push(method);
                    } else {
                        // If we have a shuffle group, emit it before this method
                        if !current_shuffle_group.is_empty() {
                            let shuffle_indices: Vec<usize> =
                                (0..current_shuffle_group.len()).collect();
                            let shuffle_methods = &current_shuffle_group;
                            execution_tokens.extend(quote! {
                                let mut shuffle_indices = vec![#(#shuffle_indices),*];
                                shuffle_indices.shuffle(&mut rand::thread_rng());
                                for &i in shuffle_indices.iter() {
                                    match i {
                                        #(
                                            #shuffle_indices => self.#shuffle_methods(fuzzer_data, &mut accounts)?,
                                        )*
                                        _ => unreachable!(),
                                    }
                                }
                            });
                            current_shuffle_group.clear();
                        }
                        // Emit the current non-shuffled method
                        execution_tokens.extend(quote! {
                            self.#method(fuzzer_data, &mut accounts)?;
                        });
                    }
                }

                // Handle any remaining shuffle group at the end
                if !current_shuffle_group.is_empty() {
                    let shuffle_indices: Vec<usize> = (0..current_shuffle_group.len()).collect();
                    let shuffle_methods = &current_shuffle_group;
                    execution_tokens.extend(quote! {
                        let mut shuffle_indices = vec![#(#shuffle_indices),*];
                        shuffle_indices.shuffle(&mut rand::thread_rng());
                        for &i in shuffle_indices.iter() {
                            match i {
                                #(
                                    #shuffle_indices => self.#shuffle_methods(fuzzer_data, &mut accounts)?,
                                )*
                                _ => unreachable!(),
                            }
                        }
                    });
                }

                quote! { #execution_tokens }
            };

            quote! {
                #flow_execution
                #random_tail
            }
        };

        let expanded = quote! {
            impl #impl_generics #type_name #ty_generics #where_clause {
                #(#impl_items)*
            }

            impl #impl_generics #type_name #ty_generics #where_clause {
                pub fn execute_flows(
                    &mut self,
                    fuzzer_data: &mut FuzzerData,
                ) -> std::result::Result<(), FuzzingError> {
                    let mut accounts = FuzzAccounts::default();

                    #init_call
                    #execute_impl
                    Ok(())
                }

                fn default_random_transactions(
                    &mut self,
                    fuzzer_data: &mut FuzzerData,
                    accounts: &mut FuzzAccounts,
                ) -> std::result::Result<(), FuzzingError> {
                    let mut transactions = <Vec<FuzzTransactions>>::arbitrary(fuzzer_data)?;
                    for transaction in transactions.iter_mut() {
                        transaction.transaction_selector(&mut self.client, accounts)?
                    }
                    Ok(())
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
