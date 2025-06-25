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
                #(self.#methods(&mut accounts)?;)*

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
                ) -> std::result::Result<(), FuzzingError> {
                    let mut accounts = FuzzAccounts::default();

                    #init_call
                    #execute_impl
                    Ok(())
                }

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

                fn fuzz(iterations: u64) {
                    let mut fuzzer = #type_name::new();

                    let pb = indicatif::ProgressBar::new(iterations);
                    pb.set_style(
                        indicatif::ProgressStyle::with_template(
                            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) [{eta_precise}] {msg}"
                        )
                        .unwrap()
                        .progress_chars("#>-"),
                    );
                    pb.set_message("Fuzzing iterations...");

                    for i in 0..iterations {
                        let _ = fuzzer.execute_flows();
                        fuzzer.client._clear_accounts();

                        pb.set_position(i + 1);
                        pb.set_message(format!("Iteration {}/{}", i + 1, iterations));
                    }

                    pb.finish_with_message("Fuzzing completed!");

                    if std::env::var("FUZZING_METRICS").is_ok() {
                        fuzzer.metrics.show_table();
                        fuzzer.metrics.print_to_file("fuzzing_metrics.json");
                    }
                }

                fn fuzz_parallel(iterations: u64){
                    use std::thread;
                    use std::time::{Duration, Instant};

                    let num_threads = thread::available_parallelism()
                        .map(|n| n.get())
                        .unwrap_or(1)
                        .min(iterations as usize);

                    if num_threads <= 1 || iterations <= 1 {
                        // Single-threaded fallback
                        #type_name::fuzz(iterations);
                        return;
                    }

                    let iterations_per_thread = iterations / num_threads as u64;
                    let remaining_iterations = iterations % num_threads as u64;

                    let mut handles = Vec::new();


                    // Create a separate progress bar for overall status
                    let main_pb = indicatif::ProgressBar::new(iterations);
                    main_pb.set_style(
                        indicatif::ProgressStyle::with_template(
                            "Overall: {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) [{eta_precise}] {msg}"
                        )
                        .unwrap()
                        .progress_chars("#>-"),
                    );
                    main_pb.set_message(format!("Fuzzing with {} threads", num_threads));

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


                        let main_pb_clone = main_pb.clone();
                        let handle = thread::spawn(move || -> FuzzingStatistics {
                            // Each thread creates its own client and fuzzer instance
                            let mut fuzzer = #type_name::new();

                            // Update progress every 100 iterations or every 50ms, whichever comes first
                            const UPDATE_INTERVAL: u64 = 100;
                            let mut last_update = Instant::now();
                            let update_duration = Duration::from_millis(50);

                            let mut local_counter = 0u64;

                            for i in 0..thread_iterations {
                                let _ = fuzzer.execute_flows();
                                fuzzer.client._clear_accounts();

                                local_counter += 1;

                                // Update progress bars with granularity control
                                let should_update = local_counter % UPDATE_INTERVAL == 0 ||
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
        };

        tokens.extend(expanded);
    }
}
