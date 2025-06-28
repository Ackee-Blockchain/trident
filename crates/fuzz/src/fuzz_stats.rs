#![allow(dead_code)]

use prettytable::{row, Table};
use std::collections::BTreeMap;
use std::{fs::File, io::Write};

use crate::types::Seed;

/// Represents fuzzing statistics, specifically tracking the number of times
/// an instruction was invoked and successfully executed.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IterationStats {
    pub invoked: u64,
    pub successful: u64,
    pub failed: u64,
    pub failed_invariant: u64,
    pub transaction_panicked: u64,

    // error | number of occurances
    pub errors: BTreeMap<String, TransactionFailedMetric>,
    pub crashes: BTreeMap<String, Crash>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TransactionFailedMetric {
    pub occurrences: u64,
    pub logs: Option<Vec<String>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct Crash {
    pub seed: Seed,
}

/// Manages and aggregates statistics for fuzzing instructions.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FuzzingStatistics {
    pub instructions: BTreeMap<String, IterationStats>,
}

impl FuzzingStatistics {
    /// Constructs a new, empty `FuzzingStatistics`.
    pub fn new() -> Self {
        let empty_instructions = BTreeMap::<String, IterationStats>::default();
        Self {
            instructions: empty_instructions,
        }
    }

    /// Increments the invocation count for a given instruction.
    /// # Arguments
    /// * `instruction` - The instruction to increment the count for.
    pub fn increase_invoked(&mut self, instruction: &str) {
        self.instructions
            .entry(instruction.to_string())
            .and_modify(|iterations_stats| iterations_stats.invoked += 1)
            .or_insert(IterationStats {
                invoked: 1,
                successful: 0,
                failed: 0,
                failed_invariant: 0,
                transaction_panicked: 0,
                errors: BTreeMap::new(),
                crashes: BTreeMap::new(),
            });
    }

    /// Increments the successful invocation count for a given instruction.
    /// # Arguments
    /// * `instruction` - The instruction to increment the successful count for.
    pub fn increase_successful(&mut self, instruction: &str) {
        self.instructions
            .entry(instruction.to_string())
            .and_modify(|iterations_stats| iterations_stats.successful += 1);
    }
    pub fn increase_failed(&mut self, instruction: &str, error: String, logs: Option<Vec<String>>) {
        self.instructions
            .entry(instruction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.failed += 1;
                iterations_stats
                    .errors
                    .entry(error)
                    .and_modify(|fail| fail.occurrences += 1)
                    .or_insert(TransactionFailedMetric {
                        occurrences: 1,
                        logs,
                    });
            });
    }
    pub fn increase_failed_invariant(&mut self, instruction: &str, seed: Seed, error: String) {
        self.instructions
            .entry(instruction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.failed_invariant += 1;
                iterations_stats
                    .crashes
                    .entry(error)
                    .and_modify(|crash| crash.seed = seed)
                    .or_insert(Crash { seed });
            });
    }

    pub fn increase_transaction_panicked(&mut self, instruction: &str, seed: Seed, error: String) {
        self.instructions
            .entry(instruction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transaction_panicked += 1;
                iterations_stats
                    .crashes
                    .entry(error)
                    .and_modify(|crash| crash.seed = seed)
                    .or_insert(Crash { seed });
            });
    }

    /// Displays the collected statistics in a formatted table.
    pub fn show_table(&self) {
        let mut table = Table::new();
        table.add_row(row![
            "Instruction",
            "Invoked Total",
            "Ix Success",
            "Ix Failed",
            "Invariant Failed",
            "Instruction Panicked",
        ]);
        for (instruction, stats) in &self.instructions {
            table.add_row(row![
                instruction,
                stats.invoked,
                stats.successful,
                stats.failed,
                stats.failed_invariant,
                stats.transaction_panicked,
            ]);
        }
        table.printstd();
        println!("Note that unhandled panics are currently logged only as crashes and are not displayed in the table above.")
    }

    pub fn print_to_file(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    /// Merges statistics from another FuzzingStatistics instance into this one.
    /// # Arguments
    /// * `other` - The other FuzzingStatistics instance to merge from.
    pub fn merge_from(&mut self, other: FuzzingStatistics) {
        for (instruction, stats) in other.instructions {
            self.instructions
                .entry(instruction.to_string())
                .and_modify(|existing_stats| {
                    existing_stats.invoked += stats.invoked;
                    existing_stats.successful += stats.successful;
                    existing_stats.failed += stats.failed;
                    existing_stats.failed_invariant += stats.failed_invariant;
                    existing_stats.transaction_panicked += stats.transaction_panicked;
                    for (error, failed_metric) in &stats.errors {
                        existing_stats
                            .errors
                            .entry(error.to_string())
                            .and_modify(|fail| fail.occurrences += failed_metric.occurrences)
                            .or_insert(TransactionFailedMetric {
                                occurrences: failed_metric.occurrences,
                                logs: failed_metric.logs.clone(),
                            });
                    }
                    for (error, crash) in &stats.crashes {
                        existing_stats
                            .crashes
                            .entry(error.to_string())
                            .and_modify(|existing_crash| *existing_crash = *crash)
                            .or_insert(*crash);
                    }
                })
                .or_insert(stats);
        }
    }
}
