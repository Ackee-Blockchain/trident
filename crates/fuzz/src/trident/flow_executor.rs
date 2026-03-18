use std::io::IsTerminal;
use std::panic::catch_unwind;
use std::panic::AssertUnwindSafe;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use trident_config::TridentConfig;
use trident_fuzz_metrics::TridentFuzzingData;

use crate::trident::Trident;

// Thread-local storage for panic location information.
// When a panic occurs, the panic handler stores the location here so we can retrieve it
// after catching the panic with catch_unwind.
thread_local! {
    static PANIC_LOCATION: std::cell::Cell<Option<String>> = const { std::cell::Cell::new(None) };
}

/// Configuration constants for the flow executor
mod config {
    use std::time::Duration;

    /// How often to update progress bars (in flow calls)
    pub const PROGRESS_UPDATE_INTERVAL: u64 = 100;

    /// How often to update progress bars (in time)
    pub const PROGRESS_UPDATE_DURATION: Duration = Duration::from_millis(50);

    /// Default seed size in bytes
    pub const SEED_SIZE: usize = 32;

    /// Environment variable names
    pub const ENV_FUZZ_DEBUG: &str = "TRIDENT_FUZZ_DEBUG";
    pub const ENV_FUZZ_SEED: &str = "TRIDENT_FUZZ_SEED";
    pub const ENV_FUZZING_METRICS: &str = "FUZZING_METRICS";
    pub const ENV_EXIT_CODE_MODE: &str = "TRIDENT_EXIT_CODE_MODE";
}

/// Specifies which type of failures should cause a non-zero exit code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCodeMode {
    /// Exit non-zero on any policy failure (program panics or custom invariant failures)
    All,
    /// Exit non-zero only on custom invariant failures in fuzz tests
    Invariants,
}

impl ExitCodeMode {
    /// Parse from environment variable value
    pub fn from_env() -> Option<Self> {
        std::env::var(config::ENV_EXIT_CODE_MODE).ok().map(|s| {
            match s.to_lowercase().as_str() {
                "all" => ExitCodeMode::All,
                "invariants" => ExitCodeMode::Invariants,
                _ => ExitCodeMode::All, // Invalid values fall back to All
            }
        })
    }
}

/// Events sent from worker threads to the UI/controller thread in parallel fuzzing.
enum WorkerEvent {
    ProgressDelta(u64),
    InvariantFailure(String),
    ProgramPanicsDelta(u64),
}

/// Final process exit outcomes for a fuzzing run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FuzzRunExit {
    Success,
    PolicyFailure,
    RuntimeFailure,
}

impl FuzzRunExit {
    fn code(self) -> i32 {
        match self {
            FuzzRunExit::Success => 0,
            FuzzRunExit::PolicyFailure => 99,
            FuzzRunExit::RuntimeFailure => 1,
        }
    }
}

/// Inputs required to decide the final process outcome for policy-controlled failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExitDecisionInput {
    exit_code_mode: Option<ExitCodeMode>,
    invariant_failed: bool,
    program_panicked: bool,
}

fn determine_exit_outcome(input: ExitDecisionInput) -> FuzzRunExit {
    let Some(mode) = input.exit_code_mode else {
        // No explicit exit-code policy: invariants/program panics do not fail the run.
        // Unexpected fuzz-test panics (e.g. unwrap on None) are handled separately
        // as runtime failures.
        return FuzzRunExit::Success;
    };

    let should_fail = match mode {
        ExitCodeMode::All => input.invariant_failed || input.program_panicked,
        ExitCodeMode::Invariants => input.invariant_failed,
    };

    if should_fail {
        FuzzRunExit::PolicyFailure
    } else {
        FuzzRunExit::Success
    }
}

fn colors_enabled() -> bool {
    std::io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none()
}

fn paint_red(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[31m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn paint_bold_yellow(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[1;33m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn paint_cyan(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[36m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn paint_magenta(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[35m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn format_invariant_line(text: &str) -> String {
    const PREFIX: &str = "Assertion failed at ";
    const SEED_PREFIX: &str = " (seed: ";

    if !colors_enabled() {
        return text.to_string();
    }

    // Expected format from handle_panic:
    // "Assertion failed at <location>: <message> (seed: <seed>)"
    if let Some(location_start) = text.strip_prefix(PREFIX) {
        if let Some(seed_idx) = location_start.rfind(SEED_PREFIX) {
            let before_seed = &location_start[..seed_idx];
            let seed_with_suffix = &location_start[seed_idx + SEED_PREFIX.len()..];
            if let Some(seed) = seed_with_suffix.strip_suffix(')') {
                if let Some(separator_idx) = before_seed.find(": ") {
                    let location = &before_seed[..separator_idx];
                    let message = &before_seed[separator_idx + 2..];
                    return format!(
                        "{} {}{}: {}{}{}{}",
                        paint_red("!"),
                        PREFIX,
                        paint_cyan(location),
                        message,
                        SEED_PREFIX,
                        paint_magenta(seed),
                        ")"
                    );
                }
            }
        }
    }

    // Fallback to accent-only if line doesn't match expected format.
    format!("{} {}", paint_red("!"), text)
}

/// Final aggregated runtime summary produced by the UI/controller thread.
struct ParallelRunSummary {
    invariant_failures: u64,
    program_panics: u64,
    panic_messages: Vec<String>,
}

/// Trait for executing fuzzing flows in the Trident framework
///
/// This trait defines the interface for fuzzing executors that can run
/// multiple iterations of randomized program interactions. Implementors
/// should provide the core fuzzing logic while this trait handles
/// parallelization, progress tracking, and metrics collection.
pub trait FlowExecutor: Send + 'static + Sized {
    /// Creates a new instance of the flow executor
    fn new() -> Self;

    /// Executes a specified number of flow calls in a single iteration
    ///
    /// # Arguments
    /// * `flow_calls_per_iteration` - Number of individual flow calls to execute
    ///
    /// # Returns
    /// Result indicating success or a fuzzing error
    fn execute_flows(
        &mut self,
        flow_calls_per_iteration: u64,
    ) -> Result<(), crate::error::FuzzingError>;

    /// Returns a mutable reference to the underlying Trident instance
    fn trident_mut(&mut self) -> &mut Trident;

    /// Resets fuzz accounts to their initial state for the next iteration
    fn reset_fuzz_accounts(&mut self);

    /// Handles LLVM coverage collection (generated by macro)
    ///
    /// This method is typically empty or contains LLVM coverage calls
    /// depending on whether coverage profiling is enabled.
    ///
    /// # Arguments
    /// * `current_iteration` - The current iteration number
    fn handle_llvm_coverage(&mut self, current_iteration: u64);

    /// Main entry point for fuzzing execution
    ///
    /// This method orchestrates the entire fuzzing process, handling both
    /// single-threaded and parallel execution based on the environment
    /// and available system resources.
    ///
    /// # Arguments
    /// * `iterations` - Total number of fuzzing iterations to run
    /// * `flow_calls_per_iteration` - Number of flow calls per iteration
    fn fuzz(iterations: u64, flow_calls_per_iteration: u64) {
        // Setup panic handler to capture location information when panics occur
        Self::setup_panic_handler();

        // Process forked accounts BEFORE any parallel processing starts
        // This ensures all RPC calls and cache writes happen in a single thread
        Self::ensure_forks_processed();

        // Debug mode: run single iteration with provided seed (for reproducing specific failures)
        if std::env::var(config::ENV_FUZZ_DEBUG).is_ok() {
            println!("Debug mode detected: Running single iteration with provided seed");
            Self::fuzz_single_threaded(1, flow_calls_per_iteration);
            return;
        }

        // Get or generate master seed for reproducible fuzzing
        let master_seed = Self::get_or_generate_master_seed();

        // Determine number of threads to use (limited by available parallelism and iteration count)
        let num_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(iterations as usize);

        // Use single-threaded mode if we only have one thread or one iteration
        if num_threads <= 1 || iterations <= 1 {
            Self::fuzz_single_threaded(iterations, flow_calls_per_iteration);
            return;
        }

        // Use parallel mode for better performance with multiple threads
        Self::fuzz_parallel(
            iterations,
            flow_calls_per_iteration,
            num_threads,
            master_seed,
        );
    }

    /// Ensures all forked accounts are processed before parallel execution starts.
    /// This is called once at the beginning to avoid race conditions with RPC calls
    /// and cache writes when multiple threads are spawned.
    fn ensure_forks_processed() {
        let config = TridentConfig::new();
        let _ = config.fork();
    }

    /// Sets up a global panic handler that captures panic location information.
    /// This allows us to retrieve the file, line, and column where a panic occurred
    /// even after catching it with catch_unwind.
    fn setup_panic_handler() {
        std::panic::set_hook(Box::new(|info| {
            let location = info
                .location()
                .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()))
                .unwrap_or_else(|| "unknown".to_string());

            PANIC_LOCATION.with(|cell| {
                cell.set(Some(location));
            });
        }));
    }

    /// Extracts the panic message from a panic payload.
    /// Handles InvariantViolation, &str, and String payloads.
    fn extract_panic_message(panic_err: &Box<dyn std::any::Any + Send>) -> String {
        if let Some(inv) = panic_err.downcast_ref::<crate::invariant::InvariantViolation>() {
            return inv.0.clone();
        }
        panic_err
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| panic_err.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "unknown panic".to_string())
    }

    /// Handles a caught panic (invariant/assertion failure) by logging it and updating the tracking flag.
    /// Returns the formatted panic message for display.
    fn handle_panic(
        panic_err: &Box<dyn std::any::Any + Send>,
        fuzzer: &mut Self,
        invariant_failed: Option<&Arc<AtomicBool>>,
    ) -> String {
        // Mark that an invariant/assertion failure occurred (for exit code handling)
        if let Some(flag) = invariant_failed {
            flag.store(true, Ordering::Relaxed);
        }

        // Extract panic details
        let message = Self::extract_panic_message(panic_err);
        let location =
            PANIC_LOCATION.with(|cell| cell.take().unwrap_or_else(|| "unknown".to_string()));
        let seed = hex::encode(fuzzer.trident_mut().get_current_seed());

        format!(
            "Assertion failed at {}: {} (seed: {})",
            location, message, seed
        )
    }

    /// Gets the master seed from environment variable or generates a random one.
    /// The master seed is used to initialize all fuzzer instances for reproducible runs.
    fn get_or_generate_master_seed() -> [u8; config::SEED_SIZE] {
        if let Ok(seed_hex) = std::env::var(config::ENV_FUZZ_SEED) {
            Self::parse_hex_seed(&seed_hex)
        } else {
            Self::generate_random_seed()
        }
    }

    /// Parses a hex-encoded seed string into a byte array.
    /// Validates that the seed is exactly the required size.
    fn parse_hex_seed(seed_hex: &str) -> [u8; config::SEED_SIZE] {
        let seed_bytes = hex::decode(seed_hex)
            .unwrap_or_else(|_| panic!("Invalid hex string in seed: {}", seed_hex));

        if seed_bytes.len() != config::SEED_SIZE {
            panic!(
                "Seed must be exactly {} bytes ({} hex characters), got: {}",
                config::SEED_SIZE,
                config::SEED_SIZE * 2,
                seed_bytes.len()
            );
        }

        let mut seed = [0u8; config::SEED_SIZE];
        seed.copy_from_slice(&seed_bytes);
        seed
    }

    /// Generates a cryptographically secure random seed.
    fn generate_random_seed() -> [u8; config::SEED_SIZE] {
        let mut seed = [0u8; config::SEED_SIZE];
        if let Err(err) = getrandom::fill(&mut seed) {
            panic!("Failed to generate random seed: {}", err);
        }
        seed
    }

    /// Outputs fuzzing metrics (JSON, dashboard, etc.) if metrics are enabled.
    fn output_metrics_if_enabled(fuzzing_data: &TridentFuzzingData) {
        if std::env::var(config::ENV_FUZZING_METRICS).is_ok() {
            if let Err(e) = fuzzing_data.generate() {
                eprintln!("Warning: Failed to generate metrics: {}", e);
            }
        }
    }

    /// Executes fuzzing in a single thread.
    /// This is used for debug mode, small iteration counts, or when only one thread is available.
    fn fuzz_single_threaded(iterations: u64, flow_calls_per_iteration: u64) {
        let mut fuzzer = Self::new();
        let is_debug_mode = std::env::var(config::ENV_FUZZ_DEBUG).is_ok();
        let exit_code_mode = ExitCodeMode::from_env();
        let mut invariant_failed = false; // Tracks fuzz test assertion/invariant failures
        let mut invariant_failure_count: u64 = 0;
        let mut panic_messages: Vec<String> = Vec::new();

        // Configure debug seed if in debug mode
        if is_debug_mode {
            let debug_seed_hex = std::env::var(config::ENV_FUZZ_DEBUG).unwrap();
            let debug_seed = Self::parse_hex_seed(&debug_seed_hex);
            println!("Using debug seed: {}", debug_seed_hex);
            fuzzer.trident_mut().set_master_seed_for_debug(debug_seed);
        }

        // Setup progress bar (disabled in debug mode for cleaner output)
        let pb = if is_debug_mode {
            None
        } else {
            let total_flow_calls = iterations * flow_calls_per_iteration;
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
            Some(pb)
        };

        // Main fuzzing loop: execute flows, catch panics, and track progress
        for i in 0..iterations {
            // Catch panics from user code (assertions, invariants, etc.)
            let panic_result = catch_unwind(AssertUnwindSafe(|| {
                let _ = fuzzer.execute_flows(flow_calls_per_iteration);
            }));

            // Handle panics - only catch InvariantViolation, re-throw others
            if let Err(panic_err) = panic_result {
                if panic_err
                    .downcast_ref::<crate::invariant::InvariantViolation>()
                    .is_some()
                {
                    // Intentional invariant failure - count it, continue fuzzing
                    invariant_failed = true;
                    invariant_failure_count += 1;
                    let panic_msg = Self::handle_panic(&panic_err, &mut fuzzer, None);

                    // In debug mode print immediately, otherwise collect for end
                    if is_debug_mode {
                        eprintln!("{}", format_invariant_line(&panic_msg));
                    } else {
                        panic_messages.push(panic_msg);
                    }
                } else {
                    // Unexpected panic (bug in fuzz test) - re-throw it
                    std::panic::resume_unwind(panic_err);
                }
            }

            // Prepare for next iteration
            fuzzer.trident_mut().next_iteration();
            fuzzer.reset_fuzz_accounts();

            // Handle coverage profiling if enabled
            Self::handle_coverage_if_enabled(&mut fuzzer, i + 1);

            // Update progress bar with live stats
            if let Some(ref pb) = pb {
                pb.inc(flow_calls_per_iteration);
                let program_panics = fuzzer
                    .trident_mut()
                    .get_fuzzing_data()
                    .get_program_panic_count();
                pb.set_message(format!(
                    "Iteration {}/{} | Invariant failures: {} | Program panics: {}",
                    i + 1,
                    iterations,
                    invariant_failure_count,
                    program_panics
                ));
            }
        }

        // Finalize progress bar
        if let Some(pb) = pb {
            pb.finish_with_message("Fuzzing completed!");
        }

        // Print collected invariant failure messages
        if !panic_messages.is_empty() {
            eprintln!(
                "\n{}",
                paint_bold_yellow(&format!(
                    "--- Invariant Failures ({}) ---",
                    panic_messages.len()
                ))
            );
            for msg in &panic_messages {
                eprintln!("{}", format_invariant_line(msg));
            }
        }

        // Generate metrics if enabled
        let fuzzing_data = fuzzer.trident_mut().get_fuzzing_data();
        Self::output_metrics_if_enabled(&fuzzing_data);

        let outcome = determine_exit_outcome(ExitDecisionInput {
            exit_code_mode,
            invariant_failed,
            program_panicked: fuzzing_data.get_program_panic_count() > 0,
        });

        // In single-thread mode we only force-exit when policy asks for a non-zero code.
        if outcome == FuzzRunExit::PolicyFailure {
            std::process::exit(outcome.code());
        }
    }

    /// Executes fuzzing across multiple threads for better performance.
    /// Each thread runs a subset of iterations with its own fuzzer instance.
    fn fuzz_parallel(
        iterations: u64,
        flow_calls_per_iteration: u64,
        num_threads: usize,
        master_seed: [u8; 32],
    ) {
        let iterations_per_thread = iterations / num_threads as u64;
        let remainder_iterations = iterations % num_threads as u64;
        let total_flow_calls = iterations * flow_calls_per_iteration;
        let exit_code_mode = ExitCodeMode::from_env();
        let (event_tx, event_rx) = mpsc::channel::<WorkerEvent>();

        // Setup shared progress bar
        let main_pb = indicatif::ProgressBar::new(total_flow_calls);
        main_pb.set_style(
            indicatif::ProgressStyle::with_template(
                "Overall: {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) [{eta_precise}] {msg}"
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        main_pb.set_message(format!(
            "Fuzzing with {} threads | Invariant failures: 0 | Program panics: 0",
            num_threads
        ));

        // Single UI/controller owner: consumes worker events, updates progress bar,
        // and builds global live counters + invariant messages.
        let ui_handle = thread::spawn(move || -> ParallelRunSummary {
            let mut invariant_failures = 0u64;
            let mut program_panics = 0u64;
            let mut panic_messages = Vec::new();

            while let Ok(event) = event_rx.recv() {
                match event {
                    WorkerEvent::ProgressDelta(delta) => {
                        main_pb.inc(delta);
                    }
                    WorkerEvent::InvariantFailure(message) => {
                        invariant_failures += 1;
                        panic_messages.push(message);
                    }
                    WorkerEvent::ProgramPanicsDelta(delta) => {
                        program_panics += delta;
                    }
                }

                main_pb.set_message(format!(
                    "Invariant failures: {} | Program panics: {}",
                    invariant_failures, program_panics
                ));
            }

            main_pb.finish_with_message("Parallel fuzzing completed!");
            ParallelRunSummary {
                invariant_failures,
                program_panics,
                panic_messages,
            }
        });

        // Spawn worker threads
        let mut handles = Vec::new();
        for thread_id in 0..num_threads {
            let thread_iterations = iterations_per_thread
                + if (thread_id as u64) < remainder_iterations {
                    1
                } else {
                    0
                };
            if thread_iterations == 0 {
                continue; // Skip threads with no work
            }

            let event_tx_clone = event_tx.clone();
            let handle = thread::spawn(move || -> Result<TridentFuzzingData, String> {
                let panic_result = catch_unwind(AssertUnwindSafe(|| {
                    run_thread_workload_impl::<Self>(
                        master_seed,
                        thread_id,
                        thread_iterations,
                        flow_calls_per_iteration,
                        event_tx_clone,
                    )
                }));

                match panic_result {
                    Ok(thread_metrics) => Ok(thread_metrics),
                    Err(panic_err) => {
                        let location = PANIC_LOCATION
                            .with(|cell| cell.take().unwrap_or_else(|| "unknown".to_string()));
                        let message = Self::extract_panic_message(&panic_err);
                        Err(format!("{} at {}", message, location))
                    }
                }
            });

            handles.push(handle);
        }
        // Drop original sender so channel closes when all workers are done.
        drop(event_tx);

        // Collect results from all threads
        let mut fuzzing_data = TridentFuzzingData::with_master_seed(master_seed);
        let mut worker_thread_failed = false;
        let mut worker_thread_panic_messages: Vec<String> = Vec::new();
        for handle in handles {
            match handle.join() {
                Ok(Ok(thread_metrics)) => {
                    fuzzing_data._merge(thread_metrics);
                }
                Ok(Err(worker_panic_msg)) => {
                    worker_thread_panic_messages.push(worker_panic_msg);
                    worker_thread_failed = true;
                }
                Err(err) => {
                    // Worker thread crashed unexpectedly (not a handled invariant failure).
                    // Buffer messages and print them only after progress bar finalization.
                    let message = if let Some(s) = err.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = err.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "unknown panic payload".to_string()
                    };
                    worker_thread_panic_messages.push(message);
                    worker_thread_failed = true;
                }
            }
        }

        // Get final aggregated runtime summary from the controller thread.
        let run_summary = ui_handle.join().unwrap_or_else(|_| ParallelRunSummary {
            invariant_failures: 0,
            program_panics: 0,
            panic_messages: Vec::new(),
        });

        // Print collected invariant failure messages
        if !run_summary.panic_messages.is_empty() {
            eprintln!(
                "\n{}",
                paint_bold_yellow(&format!(
                    "--- Invariant Failures ({}) ---",
                    run_summary.panic_messages.len()
                ))
            );
            for msg in &run_summary.panic_messages {
                eprintln!("{}", format_invariant_line(msg));
            }
        }

        if !worker_thread_panic_messages.is_empty() {
            eprintln!(
                "\n--- Worker Thread Crashes ({}) ---",
                worker_thread_panic_messages.len()
            );
            for message in &worker_thread_panic_messages {
                eprintln!("Warning: Thread failed to join (not a fuzz test panic)");
                eprintln!("  Message: {}", message);
            }
        }

        if worker_thread_failed {
            eprintln!("Fuzzing aborted: one or more worker threads crashed unexpectedly.");
            std::process::exit(FuzzRunExit::RuntimeFailure.code());
        }

        Self::output_metrics_if_enabled(&fuzzing_data);
        // Sanity output for debugging possible metric/reporting drift.
        debug_assert_eq!(
            run_summary.program_panics,
            fuzzing_data.get_program_panic_count(),
            "Runtime panic counter diverged from merged metrics"
        );
        println!("MASTER SEED used: {:?}", &hex::encode(master_seed));

        let outcome = determine_exit_outcome(ExitDecisionInput {
            exit_code_mode,
            invariant_failed: run_summary.invariant_failures > 0,
            program_panicked: run_summary.program_panics > 0,
        });
        std::process::exit(outcome.code());
    }

    /// Handles LLVM coverage collection if coverage profiling is enabled.
    /// Coverage is collected periodically based on FUZZER_LOOPCOUNT environment variable.
    fn handle_coverage_if_enabled(fuzzer: &mut Self, current_iteration: u64) {
        let loopcount = std::env::var("FUZZER_LOOPCOUNT")
            .ok()
            .and_then(|val| val.parse::<u64>().ok())
            .unwrap_or(0);

        // Collect coverage at specified intervals
        if loopcount > 0 && current_iteration > 0 && current_iteration.is_multiple_of(loopcount) {
            // Call the macro-generated LLVM method to write coverage data
            fuzzer.handle_llvm_coverage(current_iteration);

            // Notify VS Code extension to update coverage decorations
            Self::notify_coverage_extension();
        }
    }

    /// Notifies the VS Code coverage extension to update coverage decorations.
    /// This runs in a background thread to avoid blocking fuzzing execution.
    fn notify_coverage_extension() {
        let coverage_server_port =
            std::env::var("COVERAGE_SERVER_PORT").unwrap_or_else(|_| "58432".to_string());

        let url = format!(
            "http://localhost:{}/update-decorations",
            coverage_server_port
        );
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

fn send_worker_event(event_tx: &mpsc::Sender<WorkerEvent>, event: WorkerEvent) -> bool {
    event_tx.send(event).is_ok()
}

#[allow(clippy::too_many_arguments)]
/// Runs the fuzzing workload for a single thread.
/// This is extracted to reduce complexity in fuzz_parallel.
fn run_thread_workload_impl<E: FlowExecutor>(
    master_seed: [u8; 32],
    thread_id: usize,
    thread_iterations: u64,
    flow_calls_per_iteration: u64,
    event_tx: mpsc::Sender<WorkerEvent>,
) -> TridentFuzzingData {
    let mut fuzzer = E::new();
    fuzzer
        .trident_mut()
        .set_master_seed_and_thread_id(master_seed, thread_id);

    // Track progress updates to avoid excessive bar updates
    let mut last_update = Instant::now();
    let mut local_counter = 0u64;
    let mut local_observed_program_panics = 0u64;

    // Execute iterations for this thread
    for i in 0..thread_iterations {
        // Catch panics from user code (assertions, invariants, etc.)
        let panic_result = catch_unwind(AssertUnwindSafe(|| {
            let _ = fuzzer.execute_flows(flow_calls_per_iteration);
        }));

        // Handle panics - only catch InvariantViolation, re-throw others
        if let Err(panic_err) = panic_result {
            if panic_err
                .downcast_ref::<crate::invariant::InvariantViolation>()
                .is_some()
            {
                // Intentional invariant failure - count it, continue fuzzing
                let panic_msg = E::handle_panic(&panic_err, &mut fuzzer, None);
                if !send_worker_event(&event_tx, WorkerEvent::InvariantFailure(panic_msg)) {
                    return fuzzer.trident_mut().get_fuzzing_data();
                }
            } else {
                // Unexpected panic (bug in fuzz test) - re-throw it
                std::panic::resume_unwind(panic_err);
            }
        }

        // Prepare for next iteration
        fuzzer.trident_mut().next_iteration();
        fuzzer.reset_fuzz_accounts();

        // Handle coverage profiling (only thread 0 to avoid duplicate work)
        if thread_id == 0 {
            E::handle_coverage_if_enabled(&mut fuzzer, i + 1);
        }

        // Batch progress updates for performance
        local_counter += flow_calls_per_iteration;
        let should_update = local_counter >= config::PROGRESS_UPDATE_INTERVAL
            || last_update.elapsed() >= config::PROGRESS_UPDATE_DURATION
            || i == thread_iterations - 1; // Always update on last iteration

        if should_update {
            if !send_worker_event(&event_tx, WorkerEvent::ProgressDelta(local_counter)) {
                return fuzzer.trident_mut().get_fuzzing_data();
            }
            let thread_prog_panics = fuzzer
                .trident_mut()
                .get_fuzzing_data()
                .get_program_panic_count();
            let new_panics = thread_prog_panics.saturating_sub(local_observed_program_panics);
            if new_panics > 0 {
                if !send_worker_event(&event_tx, WorkerEvent::ProgramPanicsDelta(new_panics)) {
                    return fuzzer.trident_mut().get_fuzzing_data();
                }
                local_observed_program_panics = thread_prog_panics;
            }
            local_counter = 0;
            last_update = Instant::now();
        }
    }

    // Ensure any remaining progress is reported
    if local_counter > 0 && !send_worker_event(&event_tx, WorkerEvent::ProgressDelta(local_counter))
    {
        return fuzzer.trident_mut().get_fuzzing_data();
    }

    // Flush any panics that happened since the last batched UI update.
    let final_thread_prog_panics = fuzzer
        .trident_mut()
        .get_fuzzing_data()
        .get_program_panic_count();
    let final_new_panics = final_thread_prog_panics.saturating_sub(local_observed_program_panics);
    if final_new_panics > 0
        && !send_worker_event(&event_tx, WorkerEvent::ProgramPanicsDelta(final_new_panics))
    {
        return fuzzer.trident_mut().get_fuzzing_data();
    }

    fuzzer.trident_mut().get_fuzzing_data()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_outcome_no_mode_ignores_policy_failures() {
        let outcome = determine_exit_outcome(ExitDecisionInput {
            exit_code_mode: None,
            invariant_failed: true,
            program_panicked: true,
        });
        assert_eq!(outcome, FuzzRunExit::Success);
    }

    #[test]
    fn exit_outcome_invariants_mode_only_fails_on_invariants() {
        let invariant_fail = determine_exit_outcome(ExitDecisionInput {
            exit_code_mode: Some(ExitCodeMode::Invariants),
            invariant_failed: true,
            program_panicked: false,
        });
        assert_eq!(invariant_fail, FuzzRunExit::PolicyFailure);

        let program_panic_only = determine_exit_outcome(ExitDecisionInput {
            exit_code_mode: Some(ExitCodeMode::Invariants),
            invariant_failed: false,
            program_panicked: true,
        });
        assert_eq!(program_panic_only, FuzzRunExit::Success);
    }

    #[test]
    fn exit_outcome_all_mode_fails_on_either_source() {
        let invariant_fail = determine_exit_outcome(ExitDecisionInput {
            exit_code_mode: Some(ExitCodeMode::All),
            invariant_failed: true,
            program_panicked: false,
        });
        assert_eq!(invariant_fail, FuzzRunExit::PolicyFailure);

        let program_panic = determine_exit_outcome(ExitDecisionInput {
            exit_code_mode: Some(ExitCodeMode::All),
            invariant_failed: false,
            program_panicked: true,
        });
        assert_eq!(program_panic, FuzzRunExit::PolicyFailure);
    }

    #[test]
    fn exit_codes_are_stable() {
        assert_eq!(FuzzRunExit::Success.code(), 0);
        assert_eq!(FuzzRunExit::PolicyFailure.code(), 99);
        assert_eq!(FuzzRunExit::RuntimeFailure.code(), 1);
    }
}
