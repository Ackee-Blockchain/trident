use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::types::trident_flow_executor::TridentFlowExecutorImpl;

impl ToTokens for TridentFlowExecutorImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let type_name = &self.type_name;
        let impl_items = &self.impl_block;
        let methods = &self.flow_methods;

        // Generate init call if present
        let init_call = if let Some(init_method) = &self.init_method {
            quote! {
                self.#init_method(client);
            }
        } else {
            quote! {}
        };

        let execute_impl = if methods.is_empty() {
            quote! {
                self.default(fuzzer_data, client, &mut accounts)?;
            }
        } else if self.shuffle {
            // When shuffle is enabled, create a mutable vec of methods and shuffle it
            quote! {
                let mut methods = vec![#((|s: &mut Self, f: &mut FuzzerData| s.#methods(f)) as fn(&mut Self, &mut FuzzerData) -> std::result::Result<(), arbitrary::Error>),*];
                use rand::seq::SliceRandom;
                methods.shuffle(&mut rand::thread_rng());
                for method in methods {
                    method(self, fuzzer_data)?;
                }
            }
        } else {
            quote! {
                #(self.#methods(fuzzer_data,client, &mut accounts)?;)*
            }
        };

        let expanded = quote! {
            impl #type_name {
                #(#impl_items)*
            }

            impl #type_name {
                pub fn execute_flows(
                    &mut self,
                    fuzzer_data: &mut FuzzerData,
                    client: &mut impl FuzzClient,
                ) -> std::result::Result<(), arbitrary::Error> {
                    let mut accounts = FuzzAccounts::default();


                    #init_call
                    #execute_impl
                    Ok(())
                }

                fn default(
                    &mut self,
                    fuzzer_data: &mut FuzzerData,
                    client: &mut impl FuzzClient,
                    accounts: &mut FuzzAccounts,
                ) -> std::result::Result<(), arbitrary::Error> {

                    let mut transactions = <Vec<FuzzTransactions>>::arbitrary(fuzzer_data)?;
                    for transaction in transactions.iter_mut() {
                        if transaction
                            .execute_with_hooks(client, accounts)
                            .is_err()
                        {
                            client.clear_accounts();
                            return Ok(());
                        }
                    }

                    client.clear_accounts();
                    Ok(())
                }

            }
        };

        tokens.extend(expanded);
    }
}
