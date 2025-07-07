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

        let generated_impl = self.generate_generated_impl_block();

        quote! {
            impl #impl_generics #type_name #ty_generics #where_clause {
                #(#impl_items)*
            }

            #generated_impl
        }
    }

    /// Generate the main implementation block with flow execution methods
    fn generate_generated_impl_block(&self) -> TokenStream {
        let type_name = &self.type_name;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let execute_flows_method = self.generate_execute_flows_method();
        let fuzz_method = self.generate_fuzz_method();

        quote! {
            impl #impl_generics #type_name #ty_generics #where_clause {
                #execute_flows_method
                #fuzz_method
            }
        }
    }

    /// Generate the main execute_flows method
    fn generate_execute_flows_method(&self) -> TokenStream {
        let init_call = self.generate_init_call();
        let flow_execution_logic = self.generate_flow_execution_logic();
        let end_call = self.generate_end_call();

        quote! {
            pub fn execute_flows(
                &mut self,
                flow_calls_per_iteration: u64,
            ) -> std::result::Result<(), FuzzingError> {
                #init_call
                #flow_execution_logic
                #end_call
                Ok(())
            }
        }
    }

    /// Generate the initialization call if an init method exists
    fn generate_init_call(&self) -> TokenStream {
        if let Some(init_method) = &self.init_method {
            quote! {
                self.#init_method()?;
            }
        } else {
            quote! {}
        }
    }

    /// Generate the end call if an end method exists
    fn generate_end_call(&self) -> TokenStream {
        if let Some(end_method) = &self.end_method {
            quote! {
                self.#end_method()?;
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
                #index => self.#method_ident()?,
            }
        });
        let num_flows = active_methods.len();

        quote! {
            // Randomly select and execute flows for the specified number of calls
            let flows_results = for _ in 0..flow_calls_per_iteration {
                let flow_index = self.rng.gen_range(0..#num_flows);
                match flow_index {
                    #(#flow_match_arms)*
                    _ => unreachable!("Invalid flow index"),
                }
            };
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
                        self.#method_ident()?;
                        continue;
                    }
                }
            })
            .collect();

        quote! {
            // Weighted flow selection based on specified weights
            let flows_results = for _ in 0..flow_calls_per_iteration {
                let random_weight = self.rng.gen_range(0..#total_weight);
                #(#weight_ranges)*
            };
        }
    }

    /// Generate the unified fuzz method that runs in parallel by default
    fn generate_fuzz_method(&self) -> TokenStream {
        let thread_management = self.generate_thread_management_logic();
        let single_threaded_fallback = self.generate_single_threaded_fallback();

        quote! {
            fn fuzz(iterations: u64, flow_calls_per_iteration: u64) {
                // Check for debug mode first - if present, run single iteration immediately
                if std::env::var("TRIDENT_FUZZ_DEBUG").is_ok() {
                    println!("Debug mode detected: Running single iteration with provided seed");
                    let iterations = 1u64;
                    #single_threaded_fallback
                    return;
                }

                use std::thread;
                use std::time::{Duration, Instant};

                let num_threads = thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(1)
                    .min(iterations as usize);

                if num_threads <= 1 || iterations <= 1 {
                    // Single-threaded fallback
                    #single_threaded_fallback
                    return;
                }

                #thread_management
            }
        }
    }

    /// Generate single-threaded fallback logic
    fn generate_single_threaded_fallback(&self) -> TokenStream {
        let type_name = &self.type_name;
        let progress_bar_setup = self.generate_progress_bar_setup(false);
        let fuzzing_loop = self.generate_single_threaded_fuzzing_loop();
        let metrics_output = self.generate_metrics_output();

        quote! {
            let mut fuzzer = #type_name::new();

            // Set debug seed if in debug mode
            if let Ok(debug_seed_hex) = std::env::var("TRIDENT_FUZZ_DEBUG") {
                // Parse hex string to [u8; 32] using hex crate
                let seed_bytes = hex::decode(&debug_seed_hex)
                    .unwrap_or_else(|_| panic!("Invalid hex string in debug seed: {}", debug_seed_hex));

                if seed_bytes.len() != 32 {
                    panic!("Debug seed must be exactly 32 bytes (64 hex characters), got: {}", seed_bytes.len());
                }

                let mut seed = [0u8; 32];
                seed.copy_from_slice(&seed_bytes);

                println!("Using debug seed: {}", debug_seed_hex);
                fuzzer.rng = TridentRng::new(seed);
            }

            let total_flow_calls = iterations * flow_calls_per_iteration;

            #progress_bar_setup
            #fuzzing_loop
            #metrics_output
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
        let generate_write_profile_logic = self.generate_write_profile_logic();
        let loopcount_retrieval = self.generate_loopcount_retrieval();
        let generate_coverage_server_port_retrieval =
            self.generate_coverage_server_port_retrieval();

        quote! {
            #loopcount_retrieval
            #generate_coverage_server_port_retrieval

            for i in 0..iterations {
                let result = fuzzer.execute_flows(flow_calls_per_iteration);

                fuzzer.client._clear_accounts();
                fuzzer.rng.rotate_seed();
                // this will ensure the fuzz accounts will reset without
                // specifiyng the type of the fuzz accounts
                let _ = std::mem::take(&mut fuzzer.fuzz_accounts);

                pb.inc(flow_calls_per_iteration);
                pb.set_message(format!("Iteration {}/{} completed", i + 1, iterations));

                #generate_write_profile_logic
            }

            pb.finish_with_message("Fuzzing completed!");
        }
    }

    /// Generate thread management logic for parallel execution
    fn generate_thread_management_logic(&self) -> TokenStream {
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
        let generate_loopcount_retrieval = self.generate_loopcount_retrieval();
        let generate_coverage_server_port_retrieval =
            self.generate_coverage_server_port_retrieval();
        let generate_write_profile_logic = self.generate_multi_threaded_coverage();

        quote! {
            let main_pb_clone = main_pb.clone();
            let thread_id_capture = thread_id; // Capture thread_id for the closure
            let handle = thread::spawn(move || -> FuzzingStatistics {
                // Each thread creates its own client and fuzzer instance
                let mut fuzzer = #type_name::new();

                // Update progress every 100 flow calls or every 50ms, whichever comes first
                const UPDATE_INTERVAL: u64 = 100;
                let mut last_update = Instant::now();
                let update_duration = Duration::from_millis(50);

                let mut local_counter = 0u64;
                let thread_id = thread_id_capture; // Make thread_id available inside the closure

                #generate_loopcount_retrieval
                #generate_coverage_server_port_retrieval

                for i in 0..thread_iterations {
                    let _ = fuzzer.execute_flows(flow_calls_per_iteration);
                    fuzzer.client._clear_accounts();
                    fuzzer.rng.rotate_seed();
                    // this will ensure the fuzz accounts will reset without
                    // specifiyng the type of the fuzz accounts
                    let _ = std::mem::take(&mut fuzzer.fuzz_accounts);

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

                    #generate_write_profile_logic
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

    fn generate_loopcount_retrieval(&self) -> TokenStream {
        quote! {
            let loop_count = match std::env::var("FUZZER_LOOPCOUNT") {
                Ok(val) => val.parse().unwrap_or(0),
                Err(_) => 0,
            };
        }
    }

    fn generate_coverage_server_port_retrieval(&self) -> TokenStream {
        quote! {
            let coverage_server_port = std::env::var("COVERAGE_SERVER_PORT").unwrap_or("58432".to_string());
        }
    }

    fn retrieve_collect_coverage_flag(&self) -> String {
        std::env::var("COLLECT_COVERAGE").unwrap_or("0".to_string())
    }

    #[allow(unused_doc_comments)]
    fn generate_write_profile_logic(&self) -> TokenStream {
        let generate_notify_extension_logic = self.generate_notify_extension_logic();

        /// This part is a bit tricky and requires a thorough explanation:
        ///
        /// LLVM automatically creates a profraw file when the process ends, but since
        /// we run fuzz tests in a single process with multiple threads, we only get
        /// one file with combined data from all threads. To enable real-time coverage
        /// display, we manually create profraw files at intervals.
        ///
        /// set_filename: sets the filename for the profraw file
        /// write_file: creates a profraw file with collected data
        /// reset_counters: resets the counters to 0
        ///
        /// Only thread 0 writes files to avoid duplicates. We use unique filenames
        /// for each iteration and reset counters after writing. Since the final
        /// profraw file is created automatically at process end, we preemptively
        /// set the filename to avoid overwriting the last intermediate file.
        ///
        /// Coverage won't be 100% accurate because while the first thread creates
        /// the profraw file, the other threads are still running and generating data,
        /// which we reset after writing.
        ///
        /// We only generate this code if COLLECT_COVERAGE is set to 1 because
        /// if -C instrument-coverage is not enabled, llvm methods will not be available
        match self.retrieve_collect_coverage_flag().as_str() {
            "1" => quote! {
                if loop_count > 0 &&
                    i > 0 &&
                    i % loop_count == 0 {

                    unsafe {
                        let filename = format!("target/fuzz-cov-run-{}.profraw", i);
                        let filename_cstr = std::ffi::CString::new(filename).unwrap();
                        __llvm_profile_set_filename(filename_cstr.as_ptr());

                        let _ = __llvm_profile_write_file();
                        __llvm_profile_reset_counters();

                        #generate_notify_extension_logic

                        let final_filename = std::ffi::CString::new("target/fuzz-cov-run-final.profraw").unwrap();
                        __llvm_profile_set_filename(final_filename.as_ptr());
                    }
                }
            },
            _ => quote! {},
        }
    }

    fn generate_multi_threaded_coverage(&self) -> TokenStream {
        let generate_write_profile_logic = self.generate_write_profile_logic();

        quote! {
            if thread_id == 0 {
                #generate_write_profile_logic
            }
        }
    }

    /// Notifies the extension to update coverage
    /// decorations if dynamic coverage is enabled
    ///
    /// Not very nice many things hardcoded here
    /// TODO: improve architecture to avoid this
    fn generate_notify_extension_logic(&self) -> TokenStream {
        quote! {
            let url = format!(
                "http://localhost:{}/update-decorations",
                coverage_server_port
            );

            // Right now requests are rapidly fired, regardless of whether extension is ready
            // TODO: Only fire if extension responded
            std::thread::spawn(move || {
                let client = reqwest::blocking::Client::new();
                let _ = client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .body("")
                    .send();
            });
        }
    }
}
