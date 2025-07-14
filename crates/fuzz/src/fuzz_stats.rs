#![allow(dead_code)]

use prettytable::row;
use prettytable::Table;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

use crate::types::Seed;

/// Represents fuzzing statistics, specifically tracking the number of times
/// an instruction was invoked and successfully executed.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IterationStats {
    pub invoked: u64,
    pub transactions_successful: u64,
    pub transactions_failed: u64,
    pub transactions_failed_invariant: u64,
    pub transactions_panicked: u64,

    pub transactions_errors: BTreeMap<String, TransactionErrorMetrics>,
    pub custom_instruction_errors: BTreeMap<u32, TransactionErrorMetrics>,
    pub transactions_panics: BTreeMap<String, TransactionPanicMetrics>,
    pub transactions_invariant_fails: BTreeMap<String, TransactionInvariantMetrics>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TransactionErrorMetrics {
    pub occurrences: u64,
    pub logs: Option<Vec<String>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TransactionPanicMetrics {
    pub occurrences: u64,
    pub seed: String,
    pub logs: Option<Vec<String>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TransactionInvariantMetrics {
    pub occurrences: u64,
    pub seed: String,
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
                transactions_successful: 0,
                transactions_failed: 0,
                transactions_failed_invariant: 0,
                transactions_panicked: 0,
                transactions_errors: BTreeMap::new(),
                custom_instruction_errors: BTreeMap::new(),
                transactions_panics: BTreeMap::new(),
                transactions_invariant_fails: BTreeMap::new(),
            });
    }

    /// Increments the successful invocation count for a given instruction.
    /// # Arguments
    /// * `instruction` - The instruction to increment the successful count for.
    pub fn increase_successful(&mut self, instruction: &str) {
        self.instructions
            .entry(instruction.to_string())
            .and_modify(|iterations_stats| iterations_stats.transactions_successful += 1);
    }
    pub fn increase_failed(&mut self, instruction: &str, error: String, logs: Option<Vec<String>>) {
        self.instructions
            .entry(instruction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transactions_failed += 1;
                iterations_stats
                    .transactions_errors
                    .entry(error)
                    .and_modify(|fail| fail.occurrences += 1)
                    .or_insert(TransactionErrorMetrics {
                        occurrences: 1,
                        logs,
                    });
            });
    }

    pub fn increase_custom_instruction_error(
        &mut self,
        instruction: &str,
        error_code: &u32,
        logs: Option<Vec<String>>,
    ) {
        self.instructions
            .entry(instruction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transactions_failed += 1;
                iterations_stats
                    .custom_instruction_errors
                    .entry(*error_code)
                    .and_modify(|fail| fail.occurrences += 1)
                    .or_insert(TransactionErrorMetrics {
                        occurrences: 1,
                        logs,
                    });
            });
    }

    pub fn increase_failed_invariant(&mut self, instruction: &str, seed: Seed, error: String) {
        self.instructions
            .entry(instruction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transactions_failed_invariant += 1;
                iterations_stats
                    .transactions_invariant_fails
                    .entry(error)
                    .and_modify(|invariant| {
                        invariant.seed = hex::encode(seed);
                        invariant.occurrences += 1;
                    })
                    .or_insert(TransactionInvariantMetrics {
                        occurrences: 1,
                        seed: hex::encode(seed),
                    });
            });
    }

    pub fn increase_transaction_panicked(
        &mut self,
        instruction: &str,
        seed: Seed,
        panic: String,
        logs: Option<Vec<String>>,
    ) {
        self.instructions
            .entry(instruction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transactions_panicked += 1;
                iterations_stats
                    .transactions_panics
                    .entry(panic)
                    .and_modify(|panic| {
                        panic.seed = hex::encode(seed);
                        panic.occurrences += 1;
                    })
                    .or_insert(TransactionPanicMetrics {
                        occurrences: 1,
                        seed: hex::encode(seed),
                        logs,
                    });
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
                stats.transactions_successful,
                stats.transactions_failed,
                stats.transactions_failed_invariant,
                stats.transactions_panicked,
            ]);
        }
        table.printstd();
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
                    existing_stats.transactions_successful += stats.transactions_successful;
                    existing_stats.transactions_failed += stats.transactions_failed;
                    existing_stats.transactions_failed_invariant +=
                        stats.transactions_failed_invariant;
                    existing_stats.transactions_panicked += stats.transactions_panicked;
                    for (error, failed_metric) in &stats.transactions_errors {
                        existing_stats
                            .transactions_errors
                            .entry(error.to_string())
                            .and_modify(|fail| fail.occurrences += failed_metric.occurrences)
                            .or_insert(TransactionErrorMetrics {
                                occurrences: failed_metric.occurrences,
                                logs: failed_metric.logs.clone(),
                            });
                    }
                    for (error, panic) in &stats.transactions_panics {
                        existing_stats
                            .transactions_panics
                            .entry(error.to_string())
                            .and_modify(|existing_panic| {
                                existing_panic.occurrences += panic.occurrences;
                                existing_panic.seed = panic.seed.clone();
                                existing_panic.logs = panic.logs.clone();
                            })
                            .or_insert(panic.clone());
                    }
                    for (error, invariant) in &stats.transactions_invariant_fails {
                        existing_stats
                            .transactions_invariant_fails
                            .entry(error.to_string())
                            .and_modify(|existing_invariant| {
                                existing_invariant.occurrences += invariant.occurrences;
                                existing_invariant.seed = invariant.seed.clone();
                            })
                            .or_insert(invariant.clone());
                    }
                    for (error, custom_error) in &stats.custom_instruction_errors {
                        existing_stats
                            .custom_instruction_errors
                            .entry(*error)
                            .and_modify(|existing_custom_error| {
                                existing_custom_error.occurrences += custom_error.occurrences;
                                existing_custom_error.logs = custom_error.logs.clone();
                            })
                            .or_insert(custom_error.clone());
                    }
                })
                .or_insert(stats);
        }
    }
}
