use trident_config::TridentConfig;
use trident_fuzz_metrics::TridentFuzzingData;
use trident_svm::trident_svm::TridentSVM;
use trident_svm::types::trident_account::TridentAccountSharedData;
use trident_svm::types::trident_program::TridentProgram;

use crate::fuzzing::TridentRng;

/// Trait that all fuzz test structs must implement to work with the flow executor
pub trait FuzzTestExecutor {
    fn trident(&mut self) -> &mut Trident;
    fn trident_ref(&self) -> &Trident;
}

mod client;
mod metrics;
mod random;
mod seed;
#[cfg(feature = "stake")]
mod stake;
#[cfg(feature = "token")]
pub mod token;
#[cfg(feature = "token")]
pub mod token2022;
#[cfg(feature = "vote")]
mod vote;

pub struct Trident {
    pub(crate) client: TridentSVM,
    pub(crate) fuzzing_data: TridentFuzzingData,
    pub(crate) rng: TridentRng,
}

impl Default for Trident {
    fn default() -> Self {
        Self {
            client: Self::new_client(),
            fuzzing_data: TridentFuzzingData::default(),
            rng: TridentRng::default(),
        }
    }
}

impl Trident {
    /// Internal fuzzing method that handles all flow execution logic
    /// This replaces the complex macro-generated code with a simple runtime approach
    pub fn fuzz_internal<T, InitFn, FlowFn>(
        iterations: u64,
        flow_calls_per_iteration: u64,
        init_fn: InitFn,
        flow_fn: FlowFn,
    ) where
        T: FuzzTestExecutor + Send + 'static,
        InitFn: Fn() -> T + Send + Sync + 'static,
        FlowFn: Fn(&mut T, u64) -> Result<(), crate::error::FuzzingError> + Send + Sync + 'static,
    {
        use std::sync::Arc;
        use std::thread;

        // Check for debug mode first
        if std::env::var("TRIDENT_FUZZ_DEBUG").is_ok() {
            println!("Debug mode detected: Running single iteration with provided seed");
            let iterations = 1u64;
            Self::run_single_threaded(iterations, flow_calls_per_iteration, &init_fn, &flow_fn);
            return;
        } else {
            // Suppress panic messages in normal fuzzing
            std::panic::set_hook(Box::new(|_info| {}));
        }

        let master_seed = Self::get_or_generate_master_seed();
        let num_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(iterations as usize);

        if num_threads <= 1 || iterations <= 1 {
            Self::run_single_threaded(iterations, flow_calls_per_iteration, &init_fn, &flow_fn);
            return;
        }

        let init_fn = Arc::new(init_fn);
        let flow_fn = Arc::new(flow_fn);
        Self::run_parallel(
            iterations,
            flow_calls_per_iteration,
            master_seed,
            num_threads,
            init_fn,
            flow_fn,
        );
    }

    fn get_or_generate_master_seed() -> [u8; 32] {
        if let Ok(seed) = std::env::var("TRIDENT_FUZZ_SEED") {
            let seed_bytes = hex::decode(&seed)
                .unwrap_or_else(|_| panic!("The seed is not a valid hex string: {}", seed));
            let mut seed = [0; 32];
            seed.copy_from_slice(&seed_bytes);
            seed
        } else {
            let mut seed = [0; 32];
            if let Err(err) = getrandom::fill(&mut seed) {
                panic!("from_entropy failed: {}", err);
            }
            seed
        }
    }

    fn run_single_threaded<T, InitFn, FlowFn>(
        iterations: u64,
        flow_calls_per_iteration: u64,
        init_fn: &InitFn,
        flow_fn: &FlowFn,
    ) where
        T: FuzzTestExecutor + Send + 'static,
        InitFn: Fn() -> T + Send + Sync + 'static,
        FlowFn: Fn(&mut T, u64) -> Result<(), crate::error::FuzzingError> + Send + Sync + 'static,
    {
        // Initialize the base fuzzer instance with programs deployed
        let mut base_fuzzer = init_fn();

        // Set debug seed if in debug mode
        if let Ok(debug_seed_hex) = std::env::var("TRIDENT_FUZZ_DEBUG") {
            let seed_bytes = hex::decode(&debug_seed_hex)
                .unwrap_or_else(|_| panic!("Invalid hex string in debug seed: {}", debug_seed_hex));

            if seed_bytes.len() != 32 {
                panic!(
                    "Debug seed must be exactly 32 bytes (64 hex characters), got: {}",
                    seed_bytes.len()
                );
            }

            let mut seed = [0u8; 32];
            seed.copy_from_slice(&seed_bytes);

            println!("Using debug seed: {}", debug_seed_hex);
            base_fuzzer.trident()._set_master_seed_for_debug(seed);
        }

        let total_flow_calls = iterations * flow_calls_per_iteration;

        // Setup progress bar
        let pb = indicatif::ProgressBar::new(total_flow_calls);
        pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) [{eta_precise}] {msg}"
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        pb.set_message(format!(
            "Fuzzing {} iterations with {} flow calls each...",
            iterations, flow_calls_per_iteration
        ));

        let loopcount = match std::env::var("FUZZER_LOOPCOUNT") {
            Ok(val) => val.parse().unwrap_or(0),
            Err(_) => 0,
        };
        let coverage_server_port =
            std::env::var("COVERAGE_SERVER_PORT").unwrap_or("58432".to_string());

        for i in 0..iterations {
            let _ = flow_fn(&mut base_fuzzer, flow_calls_per_iteration);
            base_fuzzer.trident()._next_iteration();

            pb.inc(flow_calls_per_iteration);
            pb.set_message(format!("Iteration {}/{} completed", i + 1, iterations));

            // Handle coverage collection
            Self::handle_coverage_collection(i, loopcount, &coverage_server_port);
        }

        pb.finish_with_message("Fuzzing completed!");

        let fuzzing_data = base_fuzzer.trident()._get_fuzzing_data();
        if std::env::var("FUZZING_METRICS").is_ok() {
            fuzzing_data.generate().unwrap();
        }
    }

    fn run_parallel<T, InitFn, FlowFn>(
        iterations: u64,
        flow_calls_per_iteration: u64,
        master_seed: [u8; 32],
        num_threads: usize,
        init_fn: std::sync::Arc<InitFn>,
        flow_fn: std::sync::Arc<FlowFn>,
    ) where
        T: FuzzTestExecutor + Send + 'static,
        InitFn: Fn() -> T + Send + Sync + 'static,
        FlowFn: Fn(&mut T, u64) -> Result<(), crate::error::FuzzingError> + Send + Sync + 'static,
    {
        use std::thread;
        use std::time::{Duration, Instant};
        use trident_fuzz_metrics::TridentFuzzingData;

        let iterations_per_thread = iterations / num_threads as u64;
        let total_flow_calls = iterations * flow_calls_per_iteration;

        let mut handles = Vec::new();

        // Create overall progress bar
        let main_pb = indicatif::ProgressBar::new(total_flow_calls);
        main_pb.set_style(
            indicatif::ProgressStyle::with_template(
                "Overall: {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) [{eta_precise}] {msg}"
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        main_pb.set_message(format!(
            "Fuzzing with {} threads - {} iterations with {} flow calls each",
            num_threads, iterations, flow_calls_per_iteration
        ));

        for thread_id in 0..num_threads {
            let thread_iterations = iterations_per_thread;
            if thread_iterations == 0 {
                continue;
            }

            let main_pb_clone = main_pb.clone();
            let thread_init_fn = init_fn.clone();
            let thread_flow_fn = flow_fn.clone();

            let handle = thread::spawn(move || -> TridentFuzzingData {
                // Each thread creates its own fuzzer instance with programs deployed
                let mut fuzzer = (*thread_init_fn)();
                fuzzer
                    .trident()
                    ._set_master_seed_and_thread_id(master_seed, thread_id);

                // Update progress tracking
                const UPDATE_INTERVAL: u64 = 100;
                let mut last_update = Instant::now();
                let update_duration = Duration::from_millis(50);
                let mut local_counter = 0u64;

                let loopcount = match std::env::var("FUZZER_LOOPCOUNT") {
                    Ok(val) => val.parse().unwrap_or(0),
                    Err(_) => 0,
                };
                let coverage_server_port =
                    std::env::var("COVERAGE_SERVER_PORT").unwrap_or("58432".to_string());

                for i in 0..thread_iterations {
                    let _ = (*thread_flow_fn)(&mut fuzzer, flow_calls_per_iteration);
                    fuzzer.trident()._next_iteration();

                    local_counter += flow_calls_per_iteration;

                    // Update progress bars with granularity control
                    let should_update = local_counter >= UPDATE_INTERVAL
                        || last_update.elapsed() >= update_duration
                        || i == thread_iterations - 1;

                    if should_update {
                        main_pb_clone.inc(local_counter);
                        local_counter = 0;
                        last_update = Instant::now();
                    }

                    // Handle coverage collection (only thread 0)
                    if thread_id == 0 {
                        Self::handle_coverage_collection(i, loopcount, &coverage_server_port);
                    }
                }

                // Ensure final update
                if local_counter > 0 {
                    main_pb_clone.inc(local_counter);
                }

                fuzzer.trident()._get_fuzzing_data()
            });

            handles.push(handle);
        }

        let mut fuzzing_data = TridentFuzzingData::with_master_seed(master_seed);

        for handle in handles {
            match handle.join() {
                Ok(thread_metrics) => {
                    fuzzing_data._merge(thread_metrics);
                }
                Err(err) => {
                    if let Some(s) = err.downcast_ref::<&str>() {
                        eprintln!("Thread panicked with message: {}", s);
                    } else if let Some(s) = err.downcast_ref::<String>() {
                        eprintln!("Thread panicked with message: {}", s);
                    } else {
                        eprintln!("Thread panicked with unknown error type");
                    }
                    panic!("Error joining thread: {:?}", err);
                }
            }
        }

        main_pb.finish_with_message("Parallel fuzzing completed!");

        let exit_code = fuzzing_data.get_exit_code();

        if std::env::var("FUZZING_METRICS").is_ok() {
            fuzzing_data.generate().unwrap();
        }

        println!("MASTER SEED used: {:?}", &hex::encode(master_seed));
        std::process::exit(exit_code);
    }

    fn handle_coverage_collection(iteration: u64, loopcount: u64, coverage_server_port: &str) {
        // Only compile coverage code if COLLECT_COVERAGE environment variable is set at compile time
        if option_env!("COLLECT_COVERAGE").unwrap_or("0") == "1" {
            let collect_coverage = std::env::var("COLLECT_COVERAGE").unwrap_or("0".to_string());

            if collect_coverage == "1"
                && loopcount > 0
                && iteration > 0
                && iteration % loopcount == 0
            {
                unsafe {
                    // Use the LLVM functions declared in lib.rs
                    use crate::fuzzing::{
                        __llvm_profile_reset_counters, __llvm_profile_set_filename,
                        __llvm_profile_write_file,
                    };

                    let filename = format!("target/fuzz-cov-run-{}.profraw", iteration);
                    let filename_cstr = std::ffi::CString::new(filename).unwrap();
                    __llvm_profile_set_filename(filename_cstr.as_ptr());

                    let _ = __llvm_profile_write_file();
                    __llvm_profile_reset_counters();

                    // Notify extension
                    let url = format!(
                        "http://localhost:{}/update-decorations",
                        coverage_server_port
                    );
                    std::thread::spawn(move || {
                        // Simple HTTP POST without reqwest blocking client
                        // This is a basic implementation for coverage notification
                        let _ = std::process::Command::new("curl")
                            .arg("-X")
                            .arg("POST")
                            .arg(&url)
                            .arg("-H")
                            .arg("Content-Type: application/json")
                            .arg("-d")
                            .arg("")
                            .output();
                    });

                    let final_filename =
                        std::ffi::CString::new("target/fuzz-cov-run-final.profraw").unwrap();
                    __llvm_profile_set_filename(final_filename.as_ptr());
                }
            }
        }
    }

    fn new_client() -> TridentSVM {
        let config = TridentConfig::new();
        let program_binaries =
            config
                .programs()
                .iter()
                .fold(Vec::new(), |mut sbf_programs, config_program| {
                    let target = TridentProgram::new(
                        config_program.address,
                        config_program.upgrade_authority,
                        config_program.data.clone(),
                    );

                    sbf_programs.push(target);
                    sbf_programs
                });

        let permanent_accounts =
            config
                .accounts()
                .iter()
                .fold(Vec::new(), |mut permanent_accounts, config_account| {
                    let account = TridentAccountSharedData::new(
                        config_account.pubkey,
                        config_account.account.clone(),
                    );
                    permanent_accounts.push(account);
                    permanent_accounts
                });

        let mut svm_builder = TridentSVM::builder();
        svm_builder.with_syscalls_v1();
        svm_builder.with_syscalls_v2();
        svm_builder.with_sbf_programs(program_binaries);
        svm_builder.with_permanent_accounts(permanent_accounts);

        if std::env::var("TRIDENT_FUZZ_DEBUG_PATH").is_ok()
            && std::env::var("TRIDENT_FUZZ_DEBUG").is_ok()
        {
            let debug_path =
                std::env::var("TRIDENT_FUZZ_DEBUG_PATH").unwrap_or("trident_debug.log".to_string());
            svm_builder.with_debug_file_logs(&debug_path);
        } else if std::env::var("TRIDENT_LOG").is_ok() {
            svm_builder.with_cli_logs();
        }

        svm_builder.build()
    }
}
