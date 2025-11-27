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

        let flow_executor_impl = self.generate_flow_executor_trait_impl();

        quote! {
            impl #impl_generics #type_name #ty_generics #where_clause {
                #(#impl_items)*
            }

            #flow_executor_impl
        }
    }

    /// Generate the FlowExecutor trait implementation
    fn generate_flow_executor_trait_impl(&self) -> TokenStream {
        let type_name = &self.type_name;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let execute_flows_method = self.generate_execute_flows_method();
        let coverage_method = self.generate_coverage_method();

        quote! {
            impl #impl_generics FlowExecutor for #type_name #ty_generics #where_clause {
                fn new() -> Self {
                    Self::new()
                }

                fn execute_flows(&mut self, flow_calls_per_iteration: u64) -> std::result::Result<(), FuzzingError> {
                #execute_flows_method
                    Ok(())
                }

                fn trident_mut(&mut self) -> &mut Trident {
                    &mut self.trident
                }

                fn reset_fuzz_accounts(&mut self) {
                    // this will ensure the fuzz accounts will reset without
                    // specifying the type of the fuzz accounts
                    let _ = std::mem::take(&mut self.fuzz_accounts);
                }

                fn handle_llvm_coverage(&mut self, current_iteration: u64) {
                    #coverage_method
                }
            }
        }
    }

    /// Generate the execute_flows method implementation
    fn generate_execute_flows_method(&self) -> TokenStream {
        let init_call = self.generate_init_call();
        let flow_execution_logic = self.generate_flow_execution_logic();
        let end_call = self.generate_end_call();

        quote! {
                #init_call
                #flow_execution_logic
                #end_call

        }
    }

    /// Generate the initialization call if an init method exists
    fn generate_init_call(&self) -> TokenStream {
        if let Some(init_method) = &self.init_method {
            quote! {
                self.#init_method();
            }
        } else {
            quote! {}
        }
    }

    /// Generate the end call if an end method exists
    fn generate_end_call(&self) -> TokenStream {
        if let Some(end_method) = &self.end_method {
            quote! {
                self.#end_method();
            }
        } else {
            quote! {}
        }
    }

    /// Generate the flow execution logic
    fn generate_flow_execution_logic(&self) -> TokenStream {
        // Filter out ignored flows
        let active_methods: Vec<_> = self
            .flow_methods
            .iter()
            .filter(|method| !method.constraints.ignore)
            .collect();

        if active_methods.is_empty() {
            quote! {
                // No flow methods defined or all are ignored, nothing to execute
            }
        } else {
            let flow_selection_logic = self.generate_flow_selection_logic(&active_methods);

            quote! {
                #flow_selection_logic
            }
        }
    }

    /// Generate the random flow selection logic
    fn generate_flow_selection_logic(
        &self,
        active_methods: &[&crate::types::trident_flow_executor::FlowMethod],
    ) -> TokenStream {
        // Check if any flow has weights
        let has_weights = active_methods
            .iter()
            .any(|method| method.constraints.weight.is_some());

        if has_weights {
            // Generate weighted selection logic
            self.generate_weighted_flow_selection(active_methods)
        } else {
            // Generate uniform random selection logic (original behavior)
            self.generate_uniform_flow_selection(active_methods)
        }
    }

    /// Generate uniform random flow selection (original behavior)
    fn generate_uniform_flow_selection(
        &self,
        active_methods: &[&crate::types::trident_flow_executor::FlowMethod],
    ) -> TokenStream {
        let flow_match_arms = active_methods.iter().enumerate().map(|(index, method)| {
            let method_ident = &method.ident;
            quote! {
                #index => self.#method_ident(),
            }
        });
        let num_flows = active_methods.len();

        quote! {
            // Randomly select and execute flows for the specified number of calls
            for _ in 0..flow_calls_per_iteration {
                let flow_index = self.trident.random_from_range(0..#num_flows);
                match flow_index {
                    #(#flow_match_arms)*
                    _ => unreachable!("Invalid flow index"),
                }
            }
        }
    }

    /// Generate weighted flow selection logic
    fn generate_weighted_flow_selection(
        &self,
        active_methods: &[&crate::types::trident_flow_executor::FlowMethod],
    ) -> TokenStream {
        // Filter out flows with weight 0 (they should be skipped)
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
                        self.#method_ident();
                        continue;
                    }
                }
            })
            .collect();

        quote! {
            // Weighted flow selection based on specified weights
            for _ in 0..flow_calls_per_iteration {
                let random_weight = self.trident.random_from_range(0..#total_weight);
                #(#weight_ranges)*
            }
        }
    }

    fn generate_coverage_method(&self) -> TokenStream {
        // Check if coverage is enabled by looking for RUSTFLAGS containing -C instrument-coverage
        // This is set by the Trident CLI when running with coverage via run_with_coverage()
        let rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
        let coverage_enabled = rustflags.contains("-C instrument-coverage");

        if coverage_enabled {
            quote! {
                // LLVM coverage profiling calls - only generated when coverage is enabled
                unsafe {
                    let filename = format!("target/fuzz-cov-run-{}.profraw", current_iteration);
                    if let Ok(filename_cstr) = std::ffi::CString::new(filename) {
                        trident_fuzz::fuzzing::__llvm_profile_set_filename(filename_cstr.as_ptr());
                        let _ = trident_fuzz::fuzzing::__llvm_profile_write_file();
                        trident_fuzz::fuzzing::__llvm_profile_reset_counters();

                        // Set final filename to avoid overwriting intermediate files
                        if let Ok(final_filename_cstr) = std::ffi::CString::new("target/fuzz-cov-run-final.profraw") {
                            trident_fuzz::fuzzing::__llvm_profile_set_filename(final_filename_cstr.as_ptr());
                        }
                    }
                }
            }
        } else {
            quote! {
                // Coverage profiling disabled - prevents linking errors
            }
        }
    }
}
