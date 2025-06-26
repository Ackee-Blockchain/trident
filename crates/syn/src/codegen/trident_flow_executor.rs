use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

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

        let user_impl = self.generate_user_impl_block();
        let generated_impl = self.generate_generated_impl_block();

        quote! {
            impl #impl_generics #type_name #ty_generics #where_clause {
                #(#impl_items)*
            }

            #generated_impl
        }
    }

    /// Generate the user's implementation block (placeholder for future use)
    fn generate_user_impl_block(&self) -> TokenStream {
        // This could be used for user-defined methods in the future
        quote! {}
    }

    /// Generate the main implementation block with flow execution methods
    fn generate_generated_impl_block(&self) -> TokenStream {
        let type_name = &self.type_name;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let execute_flows_method = self.generate_execute_flows_method();
        let default_random_transactions_method = self.generate_default_random_transactions_method();
        let fuzz_method = self.generate_fuzz_method();
        let fuzz_parallel_method = self.generate_fuzz_parallel_method();

        quote! {
            impl #impl_generics #type_name #ty_generics #where_clause {
                #execute_flows_method
                #default_random_transactions_method
                #fuzz_method
                #fuzz_parallel_method
            }
        }
    }

    /// Generate the main execute_flows method
    fn generate_execute_flows_method(&self) -> TokenStream {
        let init_call = self.generate_init_call();
        let flow_execution_logic = self.generate_flow_execution_logic();

        quote! {
            pub fn execute_flows(
                &mut self,
                flow_calls_per_iteration: u64,
            ) -> std::result::Result<(), FuzzingError> {
                let mut accounts = FuzzAccounts::default();

                #init_call
                #flow_execution_logic
                Ok(())
            }
        }
    }

    /// Generate the initialization call if an init method exists
    fn generate_init_call(&self) -> TokenStream {
        if let Some(init_method) = &self.init_method {
            quote! {
                self.#init_method(&mut accounts)?;
            }
        } else {
            quote! {}
        }
    }

    /// Generate the flow execution logic (random selection or default)
    fn generate_flow_execution_logic(&self) -> TokenStream {
        let methods = &self.flow_methods;

        if methods.is_empty() {
            quote! {
                // No flow methods or all are ignored, use default implementation
                self.default_random_transactions(&mut accounts)?;
            }
        } else {
            let flow_selection_logic = self.generate_flow_selection_logic();
            let random_tail = self.generate_random_tail_logic();

            quote! {
                #flow_selection_logic
                #random_tail
            }
        }
    }

    /// Generate the random flow selection logic
    fn generate_flow_selection_logic(&self) -> TokenStream {
        let methods = &self.flow_methods;
        let flow_match_arms = methods.iter().enumerate().map(|(index, method)| {
            quote! {
                #index => self.#method(&mut accounts)?,
            }
        });
        let num_flows = methods.len();

        quote! {
            // Randomly select and execute flows for the specified number of calls
            for _ in 0..flow_calls_per_iteration {
                let flow_index = self.rng.gen_range(0..#num_flows);
                match flow_index {
                    #(#flow_match_arms)*
                    _ => unreachable!("Invalid flow index"),
                }
            }
        }
    }

    /// Generate the random tail logic if enabled
    fn generate_random_tail_logic(&self) -> TokenStream {
        if self.args.random_tail {
            quote! {
                self.default_random_transactions(&mut accounts)?;
            }
        } else {
            quote! {}
        }
    }

    /// Generate the default random transactions method
    fn generate_default_random_transactions_method(&self) -> TokenStream {
        quote! {
            fn default_random_transactions(
                &mut self,
                accounts: &mut FuzzAccounts,
            ) -> std::result::Result<(), FuzzingError> {
                // let mut transactions = <Vec<FuzzTransactions>>::arbitrary(fuzzer_data)?;
                // for transaction in transactions.iter_mut() {
                //     transaction.transaction_selector(&mut self.metrics, &mut self.client, accounts, &mut self.rng)?
                // }
                Ok(())
            }
        }
    }

    /// Generate the single-threaded fuzz method
    fn generate_fuzz_method(&self) -> TokenStream {
        let type_name = &self.type_name;
        let progress_bar_setup = self.generate_progress_bar_setup(false);
        let fuzzing_loop = self.generate_single_threaded_fuzzing_loop();
        let metrics_output = self.generate_metrics_output();

        quote! {
            fn fuzz(iterations: u64, flow_calls_per_iteration: u64) {
                let mut fuzzer = #type_name::new();
                let total_flow_calls = iterations * flow_calls_per_iteration;

                #progress_bar_setup
                #fuzzing_loop
                #metrics_output
            }
        }
    }

    /// Generate the multi-threaded fuzz method
    fn generate_fuzz_parallel_method(&self) -> TokenStream {
        let type_name = &self.type_name;
        let thread_management = self.generate_thread_management_logic();

        quote! {
            fn fuzz_parallel(iterations: u64, flow_calls_per_iteration: u64){
                use std::thread;
                use std::time::{Duration, Instant};

                let num_threads = thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(1)
                    .min(iterations as usize);

                if num_threads <= 1 || iterations <= 1 {
                    // Single-threaded fallback
                    #type_name::fuzz(iterations, flow_calls_per_iteration);
                    return;
                }

                #thread_management
            }
        }
    }

    /// Generate progress bar setup code
    fn generate_progress_bar_setup(&self, is_parallel: bool) -> TokenStream {
        let message_prefix = if is_parallel { "Overall: " } else { "" };
        let message_content = if is_parallel {
            quote! { format!("Fuzzing with {} threads - {} iterations with {} flow calls each", num_threads, iterations, flow_calls_per_iteration) }
        } else {
            quote! { format!("Fuzzing {} iterations with {} flow calls each...", iterations, flow_calls_per_iteration) }
        };

        quote! {
            let pb = indicatif::ProgressBar::new(total_flow_calls);
            pb.set_style(
                indicatif::ProgressStyle::with_template(
                    concat!(#message_prefix, "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) [{eta_precise}] {msg}")
                )
                .unwrap()
                .progress_chars("#>-"),
            );
            pb.set_message(#message_content);
        }
    }

    /// Generate the single-threaded fuzzing loop
    fn generate_single_threaded_fuzzing_loop(&self) -> TokenStream {
        quote! {
            for i in 0..iterations {
                let result = fuzzer.execute_flows(flow_calls_per_iteration);

                fuzzer.client._clear_accounts();
                fuzzer.rng.rotate_seed();

                pb.inc(flow_calls_per_iteration);
                pb.set_message(format!("Iteration {}/{} completed", i + 1, iterations));
            }

            pb.finish_with_message("Fuzzing completed!");
        }
    }

    /// Generate thread management logic for parallel execution
    fn generate_thread_management_logic(&self) -> TokenStream {
        let type_name = &self.type_name;
        let parallel_progress_setup = self.generate_parallel_progress_setup();
        let thread_spawn_logic = self.generate_thread_spawn_logic();
        let metrics_collection = self.generate_metrics_collection_logic();

        quote! {
            let iterations_per_thread = iterations / num_threads as u64;
            let remaining_iterations = iterations % num_threads as u64;
            let total_flow_calls = iterations * flow_calls_per_iteration;

            let mut handles = Vec::new();

            #parallel_progress_setup

            for thread_id in 0..num_threads {
                // Calculate iterations for this thread
                let thread_iterations = if thread_id < remaining_iterations as usize {
                    iterations_per_thread + 1
                } else {
                    iterations_per_thread
                };

                if thread_iterations == 0 {
                    continue;
                }

                #thread_spawn_logic
            }

            #metrics_collection
        }
    }

    /// Generate parallel progress bar setup
    fn generate_parallel_progress_setup(&self) -> TokenStream {
        quote! {
            // Create a separate progress bar for overall status
            let main_pb = indicatif::ProgressBar::new(total_flow_calls);
            main_pb.set_style(
                indicatif::ProgressStyle::with_template(
                    "Overall: {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) [{eta_precise}] {msg}"
                )
                .unwrap()
                .progress_chars("#>-"),
            );
            main_pb.set_message(format!("Fuzzing with {} threads - {} iterations with {} flow calls each", num_threads, iterations, flow_calls_per_iteration));
        }
    }

    /// Generate thread spawning logic
    fn generate_thread_spawn_logic(&self) -> TokenStream {
        let type_name = &self.type_name;

        quote! {
            let main_pb_clone = main_pb.clone();
            let handle = thread::spawn(move || -> FuzzingStatistics {
                // Each thread creates its own client and fuzzer instance
                let mut fuzzer = #type_name::new();

                // Update progress every 100 flow calls or every 50ms, whichever comes first
                const UPDATE_INTERVAL: u64 = 100;
                let mut last_update = Instant::now();
                let update_duration = Duration::from_millis(50);

                let mut local_counter = 0u64;

                for i in 0..thread_iterations {
                    let _ = fuzzer.execute_flows(flow_calls_per_iteration);
                    fuzzer.client._clear_accounts();
                    fuzzer.rng.rotate_seed();

                    local_counter += flow_calls_per_iteration;

                    // Update progress bars with granularity control
                    let should_update = local_counter >= UPDATE_INTERVAL ||
                                      last_update.elapsed() >= update_duration ||
                                      i == thread_iterations - 1; // Always update on last iteration

                    if should_update {
                        main_pb_clone.inc(local_counter);
                        local_counter = 0;
                        last_update = Instant::now();
                    }
                }

                // Ensure final update
                if local_counter > 0 {
                    main_pb_clone.inc(local_counter);
                }

                // Return the metrics from this thread
                fuzzer.metrics
            });

            handles.push(handle);
        }
    }

    /// Generate metrics collection and output logic
    fn generate_metrics_collection_logic(&self) -> TokenStream {
        quote! {
            // Collect metrics from all threads
            let mut merged_metrics = FuzzingStatistics::new();

            for handle in handles {
                if let Ok(thread_metrics) = handle.join() {
                    // Merge the thread metrics directly
                    merged_metrics.merge_from(thread_metrics);
                }
            }

            main_pb.finish_with_message("Parallel fuzzing completed!");

            if std::env::var("FUZZING_METRICS").is_ok() {
                merged_metrics.show_table();
                merged_metrics.print_to_file("fuzzing_metrics.json");
            }
        }
    }

    /// Generate metrics output logic
    fn generate_metrics_output(&self) -> TokenStream {
        quote! {
            if std::env::var("FUZZING_METRICS").is_ok() {
                fuzzer.metrics.show_table();
                fuzzer.metrics.print_to_file("fuzzing_metrics.json");
            }
        }
    }
}
