use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use crate::types::trident_flow_executor::TridentFlowExecutorImpl;

impl ToTokens for TridentFlowExecutorImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expanded = self.generate_flow_executor_impl();
        tokens.extend(expanded);
    }
}

impl TridentFlowExecutorImpl {
    /// Generate the complete flow executor implementation
    fn generate_flow_executor_impl(&self) -> TokenStream {
        let type_name = &self.type_name;
        let impl_items = &self.impl_block;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let generated_impl = self.generate_simplified_impl_block();

        quote! {
            impl #impl_generics #type_name #ty_generics #where_clause {
                #(#impl_items)*
            }

            #generated_impl
        }
    }

    /// Generate the simplified implementation block that uses Trident's fuzz_internal
    fn generate_simplified_impl_block(&self) -> TokenStream {
        let type_name = &self.type_name;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let flow_executor_closure = self.generate_flow_executor_closure();

        let init_closure = self.generate_init_closure();
        let trait_impl = self.generate_trait_impl();

        quote! {
            #trait_impl

            impl #impl_generics #type_name #ty_generics #where_clause {
                fn fuzz(iterations: u64, flow_calls_per_iteration: u64) {
                    Trident::fuzz_internal(
                        iterations,
                        flow_calls_per_iteration,
                        #init_closure,
                        #flow_executor_closure
                    );
                }
            }
        }
    }

    /// Generate the FuzzTestExecutor trait implementation
    fn generate_trait_impl(&self) -> TokenStream {
        let type_name = &self.type_name;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        quote! {
            impl #impl_generics FuzzTestExecutor for #type_name #ty_generics #where_clause {
                fn trident(&mut self) -> &mut Trident {
                    &mut self.trident
                }

                fn trident_ref(&self) -> &Trident {
                    &self.trident
                }
            }
        }
    }

    /// Generate a closure that creates and initializes the fuzzer (calls new())
    fn generate_init_closure(&self) -> TokenStream {
        let type_name = &self.type_name;

        quote! {
            || #type_name::new()
        }
    }

    /// Generate a closure that handles flow execution per iteration
    /// This replaces all the complex macro-generated logic with a simple runtime approach
    fn generate_flow_executor_closure(&self) -> TokenStream {
        let type_name = &self.type_name;

        // Filter out ignored flows
        let active_methods: Vec<_> = self
            .flow_methods
            .iter()
            .filter(|method| !method.constraints.ignore)
            .collect();

        let init_call = if let Some(init_method) = &self.init_method {
            quote! { fuzzer.#init_method(); }
        } else {
            quote! {}
        };

        let end_call = if let Some(end_method) = &self.end_method {
            quote! { fuzzer.#end_method(); }
        } else {
            quote! {}
        };

        if active_methods.is_empty() {
            return quote! {
                |_fuzzer: &mut #type_name, _flow_calls_per_iteration: u64| -> Result<(), FuzzingError> {
                    // No flow methods defined or all are ignored
                    Ok(())
                }
            };
        }

        // Check if any flow has weights
        let has_weights = active_methods
            .iter()
            .any(|method| method.constraints.weight.is_some());

        let flow_selection_logic = if has_weights {
            self.generate_weighted_selection_logic(&active_methods)
        } else {
            self.generate_uniform_selection_logic(&active_methods)
        };

        quote! {
            |fuzzer: &mut #type_name, flow_calls_per_iteration: u64| -> Result<(), FuzzingError> {
                #init_call
                #flow_selection_logic
                #end_call
                Ok(())
            }
        }
    }

    /// Generate uniform flow selection logic for the closure
    fn generate_uniform_selection_logic(
        &self,
        active_methods: &[&crate::types::trident_flow_executor::FlowMethod],
    ) -> TokenStream {
        let flow_match_arms = active_methods.iter().enumerate().map(|(index, method)| {
            let method_ident = &method.ident;
            quote! {
                #index => fuzzer.#method_ident(),
            }
        });
        let num_flows = active_methods.len();

        quote! {
            for _ in 0..flow_calls_per_iteration {
                let flow_index = fuzzer.trident.gen_range(0..#num_flows);
                match flow_index {
                    #(#flow_match_arms)*
                    _ => unreachable!("Invalid flow index"),
                }
            }
        }
    }

    /// Generate weighted flow selection logic for the closure
    fn generate_weighted_selection_logic(
        &self,
        active_methods: &[&crate::types::trident_flow_executor::FlowMethod],
    ) -> TokenStream {
        // Filter out flows with weight 0
        let weighted_methods: Vec<_> = active_methods
            .iter()
            .filter(|method| method.constraints.weight.unwrap_or(0) > 0)
            .collect();

        if weighted_methods.is_empty() {
            return quote! {
                // All flows have weight 0, nothing to execute
            };
        }

        // Calculate total weight
        let total_weight: u32 = weighted_methods
            .iter()
            .map(|method| method.constraints.weight.unwrap())
            .sum();

        // Generate weight ranges and method calls
        let mut cumulative_weight = 0u32;
        let weight_ranges: Vec<_> = weighted_methods
            .iter()
            .map(|method| {
                let weight = method.constraints.weight.unwrap();
                let _start = cumulative_weight;
                cumulative_weight += weight;
                let end = cumulative_weight;
                let method_ident = &method.ident;

                quote! {
                    if random_weight < #end {
                        fuzzer.#method_ident();
                        continue;
                    }
                }
            })
            .collect();

        quote! {
            for _ in 0..flow_calls_per_iteration {
                let random_weight = fuzzer.trident.gen_range(0..#total_weight);
                #(#weight_ranges)*
            }
        }
    }
}
