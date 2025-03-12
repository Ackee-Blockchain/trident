use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_flow_executor::TridentFlowExecutorImpl;

impl ToTokens for TridentFlowExecutorImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let type_name = &self.type_name;
        let impl_items = &self.impl_block;
        let methods = &self.flow_methods;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        // Generate init call if present
        let init_call = if let Some(init_method) = &self.init_method {
            quote! {
                self.#init_method();
            }
        } else {
            quote! {}
        };

        // Use default_random_flows if there are no flow methods
        let execute_impl = if methods.is_empty() {
            quote! {
                // No flow methods or all are ignored, use default implementation
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

            quote! {
                // Execute all defined flow methods
                #(self.#methods(fuzzer_data, &mut accounts)?;)*

                // Optional random tail transactions
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
                                self.client._clear_accounts();
                            });
                        }
                    } else if cfg!(afl) {
                        let fuzzing_metrics = std::env::var("FUZZING_METRICS");
                        if fuzzing_metrics.is_ok() {
                            let mut signals = signal_hook::iterator::Signals::new([signal_hook::consts::SIGINT]).expect("Failed to register signal handlers");
                            std::thread::spawn(move || {
                                #[allow(clippy::never_loop)]
                                for sig in signals.forever() {
                                let shmem_id = format!("fuzzer_stats_{}", std::process::id());
                                if let Ok(signal_shmem) = shared_memory::ShmemConf::new().os_id(&shmem_id).open() {
                                    let stats = unsafe { &mut *(signal_shmem.as_ptr() as *mut FuzzStats) };
                                    stats.save_to_file(Some(sig));
                                }
                                    // SHOULD_EXIT.store(true, Ordering::SeqCst);
                                    std::process::exit(0);
                                }
                            });
                        }


                        fuzz_afl(true, |fuzzer_data| {
                            let mut buf = Unstructured::new(fuzzer_data);
                            let _ = self.execute_flows(&mut buf);
                            self.client._clear_accounts();
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
                        self.client._clear_accounts();
                    } else {
                        panic!("Select Honggfuzz or AFL for fuzzing!!!")
                    }
                }
            }
        };

        tokens.extend(expanded);
    }
}
