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
            quote! {
                // Execute all defined flow methods
                #(self.#methods(fuzzer_data, &mut accounts)?;)*
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
            }
        };

        tokens.extend(expanded);
    }
}
