use std::io::IsTerminal;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

/// Events sent from worker threads to the UI/controller thread in parallel fuzzing.
pub(crate) enum WorkerEvent {
    ProgressDelta(u64),
    InvariantFailure(String),
    ProgramPanicsDelta(u64),
}

/// Final aggregated runtime summary produced by the UI/controller thread.
pub(crate) struct ParallelRunSummary {
    pub(crate) invariant_failures: u64,
    pub(crate) program_panics: u64,
    pub(crate) panic_messages: Vec<String>,
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

pub(crate) fn paint_bold_yellow(text: &str) -> String {
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

fn paint_green(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[32m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn paint_dim(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[2m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn paint_counter(value: u64, is_problem: bool) -> String {
    let raw = value.to_string();
    if !colors_enabled() {
        return raw;
    }
    if value == 0 {
        paint_green(&raw)
    } else if is_problem {
        paint_red(&raw)
    } else {
        paint_bold_yellow(&raw)
    }
}

pub(crate) fn format_invariant_line(text: &str) -> String {
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

fn format_exec_rate(completed: u64, elapsed: Duration) -> f64 {
    let secs = elapsed.as_secs_f64();
    if secs <= f64::EPSILON {
        0.0
    } else {
        completed as f64 / secs
    }
}

pub(crate) fn format_live_status_single(
    iteration: u64,
    iterations: u64,
    completed_flow_calls: u64,
    total_flow_calls: u64,
    invariant_failures: u64,
    program_panics: u64,
    elapsed: Duration,
) -> String {
    let exec_rate = format_exec_rate(completed_flow_calls, elapsed);
    let iter_label = paint_cyan("iter");
    let flow_label = paint_cyan("flow calls");
    let inv_label = paint_magenta("inv");
    let panics_label = paint_magenta("panics");
    let exec_label = paint_cyan("exec/s");
    let inv_val = paint_counter(invariant_failures, true);
    let panic_val = paint_counter(program_panics, true);
    let iter_sep = paint_dim("|");
    let metric_sep = paint_dim("|");
    format!(
        "{iter_label}: {iteration}/{iterations} {iter_sep} {flow_label}: {completed_flow_calls}/{total_flow_calls}\n{inv_label}: {inv_val} {metric_sep} {panics_label}: {panic_val} {metric_sep} {exec_label}: {exec_rate:.0}",
        iter_label = iter_label,
        iteration = iteration,
        iterations = iterations,
        iter_sep = iter_sep,
        flow_label = flow_label,
        completed_flow_calls = completed_flow_calls,
        total_flow_calls = total_flow_calls,
        inv_label = inv_label,
        inv_val = inv_val,
        metric_sep = metric_sep,
        panics_label = panics_label,
        panic_val = panic_val,
        exec_label = exec_label,
        exec_rate = exec_rate
    )
}

fn format_live_status_parallel(
    num_threads: usize,
    completed_flow_calls: u64,
    total_flow_calls: u64,
    invariant_failures: u64,
    program_panics: u64,
    elapsed: Duration,
) -> String {
    let exec_rate = format_exec_rate(completed_flow_calls, elapsed);
    let threads_label = paint_cyan("threads");
    let flow_label = paint_cyan("flow calls");
    let inv_label = paint_magenta("inv");
    let panics_label = paint_magenta("panics");
    let exec_label = paint_cyan("exec/s");
    let inv_val = paint_counter(invariant_failures, true);
    let panic_val = paint_counter(program_panics, true);
    let sep = paint_dim("|");
    format!(
        "{threads_label}: {num_threads} {sep} {flow_label}: {completed_flow_calls}/{total_flow_calls}\n{inv_label}: {inv_val} {sep} {panics_label}: {panic_val} {sep} {exec_label}: {exec_rate:.0}",
        threads_label = threads_label,
        num_threads = num_threads,
        sep = sep,
        flow_label = flow_label,
        completed_flow_calls = completed_flow_calls,
        total_flow_calls = total_flow_calls,
        inv_label = inv_label,
        inv_val = inv_val,
        panics_label = panics_label,
        panic_val = panic_val,
        exec_label = exec_label,
        exec_rate = exec_rate
    )
}

pub(crate) fn create_single_progress_bar(
    total_flow_calls: u64,
    iterations: u64,
    flow_calls_per_iteration: u64,
) -> indicatif::ProgressBar {
    let pb = indicatif::ProgressBar::new(total_flow_calls);
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) [{eta_precise}] {msg}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb.set_message(format!(
        "Fuzzing {} iterations with {} flow calls each...",
        iterations, flow_calls_per_iteration
    ));
    pb
}

pub(crate) fn spawn_parallel_ui_controller(
    event_rx: mpsc::Receiver<WorkerEvent>,
    num_threads: usize,
    total_flow_calls: u64,
) -> thread::JoinHandle<ParallelRunSummary> {
    let main_pb = indicatif::ProgressBar::new(total_flow_calls);
    main_pb.set_style(
        indicatif::ProgressStyle::with_template(
            "Overall: {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) [{eta_precise}] {msg}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    let ui_start_time = Instant::now();
    main_pb.set_message(format_live_status_parallel(
        num_threads,
        0,
        total_flow_calls,
        0,
        0,
        ui_start_time.elapsed(),
    ));

    thread::spawn(move || -> ParallelRunSummary {
        let mut invariant_failures = 0u64;
        let mut program_panics = 0u64;
        let mut panic_messages = Vec::new();
        let mut completed_flow_calls = 0u64;

        while let Ok(event) = event_rx.recv() {
            match event {
                WorkerEvent::ProgressDelta(delta) => {
                    completed_flow_calls += delta;
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

            main_pb.set_message(format_live_status_parallel(
                num_threads,
                completed_flow_calls,
                total_flow_calls,
                invariant_failures,
                program_panics,
                ui_start_time.elapsed(),
            ));
        }

        main_pb.finish_with_message("Parallel fuzzing completed!");
        ParallelRunSummary {
            invariant_failures,
            program_panics,
            panic_messages,
        }
    })
}
