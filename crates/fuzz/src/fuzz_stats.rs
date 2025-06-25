#![allow(dead_code)]

use prettytable::{row, Table};
use std::collections::BTreeMap;
use std::{fs::File, io::Write};

/// Represents fuzzing statistics, specifically tracking the number of times
/// an instruction was invoked and successfully executed.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IterationStats {
    pub invoked: u64,
    pub successful: u64,
    pub failed: u64,
    pub failed_check: u64,
    // error | number of occurances
    pub errors: BTreeMap<String, u64>,
}

/// Manages and aggregates statistics for fuzzing instructions.
#[derive(Debug, Default)]
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
    pub fn increase_invoked(&mut self, instruction: String) {
        self.instructions
            .entry(instruction)
            .and_modify(|iterations_stats| iterations_stats.invoked += 1)
            .or_insert(IterationStats {
                invoked: 1,
                successful: 0,
                failed: 0,
                failed_check: 0,
                errors: BTreeMap::new(),
            });
    }

    /// Increments the successful invocation count for a given instruction.
    /// # Arguments
    /// * `instruction` - The instruction to increment the successful count for.
    pub fn increase_successful(&mut self, instruction: String) {
        self.instructions
            .entry(instruction)
            .and_modify(|iterations_stats| iterations_stats.successful += 1)
            .or_insert(
                // this should not occure as instruction has to be invoked
                // and then successfully_invoked
                IterationStats {
                    invoked: 1,
                    successful: 1,
                    failed: 0,
                    failed_check: 0,
                    errors: BTreeMap::new(),
                },
            );
    }
    pub fn increase_failed(&mut self, instruction: String, error: String) {
        self.instructions
            .entry(instruction)
            .and_modify(|iterations_stats| {
                iterations_stats.failed += 1;
                iterations_stats
                    .errors
                    .entry(error)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            })
            .or_insert(
                // this should not occure as instruction has to be invoked
                // and then unsuccessfully_invoked
                IterationStats {
                    invoked: 1,
                    successful: 0,
                    failed: 1,
                    failed_check: 0,
                    errors: BTreeMap::new(),
                },
            );
    }
    pub fn increase_failed_check(&mut self, instruction: String) {
        self.instructions
            .entry(instruction)
            .and_modify(|iterations_stats| iterations_stats.failed_check += 1)
            .or_insert(
                // this should not occure as instruction has to be invoked
                // and then unsuccessfully_invoked
                IterationStats {
                    invoked: 1,
                    successful: 1,
                    failed: 0,
                    failed_check: 1,
                    errors: BTreeMap::new(),
                },
            );
    }

    /// Displays the collected statistics in a formatted table.
    pub fn show_table(&self) {
        let mut table = Table::new();
        table.add_row(row![
            "Instruction",
            "Invoked Total",
            "Ix Success",
            "Check Failed",
            "Ix Failed",
        ]);
        for (instruction, stats) in &self.instructions {
            table.add_row(row![
                instruction,
                stats.invoked,
                stats.successful,
                stats.failed_check,
                stats.failed,
            ]);
        }
        table.printstd();
        println!("Note that unhandled panics are currently logged only as crashes and are not displayed in the table above.")
    }

    pub fn print_to_file(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let serialized = serde_json::to_string(&self.instructions).unwrap();
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
                    existing_stats.failed_check += stats.failed_check;
                    for (error, count) in &stats.errors {
                        existing_stats
                            .errors
                            .entry(error.to_string())
                            .and_modify(|existing_count| *existing_count += count)
                            .or_insert(*count);
                    }
                })
                .or_insert(stats);
        }
    }
}
