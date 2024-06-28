#![allow(dead_code)]

use prettytable::{row, Table};
use std::collections::HashMap;

/// Represents fuzzing statistics, specifically tracking the number of times
/// an instruction was invoked and successfully executed.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IterationStats {
    pub invoked: u64,
    pub successful: u64,
    pub failed: u64,
    pub failed_check: u64,
}

/// Manages and aggregates statistics for fuzzing instructions.
#[derive(Debug, Default)]
pub struct FuzzingStatistics {
    pub instructions: HashMap<String, IterationStats>,
}

impl FuzzingStatistics {
    /// Constructs a new, empty `FuzzingStatistics`.
    pub fn new() -> Self {
        let empty_instructions = HashMap::<String, IterationStats>::default();
        Self {
            instructions: empty_instructions,
        }
    }
    /// Outputs the statistics as a serialized JSON string.
    pub fn output_serialized(&self) {
        let serialized = serde_json::to_string(&self.instructions).unwrap();
        println!("{}", serialized);
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
                },
            );
    }
    pub fn increase_failed(&mut self, instruction: String) {
        self.instructions
            .entry(instruction)
            .and_modify(|iterations_stats| iterations_stats.failed += 1)
            .or_insert(
                // this should not occure as instruction has to be invoked
                // and then unsuccessfully_invoked
                IterationStats {
                    invoked: 1,
                    successful: 0,
                    failed: 1,
                    failed_check: 0,
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
                },
            );
    }

    /// Inserts or updates instructions with statistics provided in a serialized string.
    /// # Arguments
    /// * `serialized_iteration` - The serialized statistics to insert or update.
    pub fn insert_serialized(&mut self, serialized_iteration: &str) {
        let result = serde_json::from_str::<HashMap<String, IterationStats>>(serialized_iteration);

        if let Ok(deserialized_instruction) = result {
            for (key, value) in deserialized_instruction {
                self.instructions
                    .entry(key)
                    .and_modify(|instruction_stats| {
                        instruction_stats.invoked += value.invoked;
                        instruction_stats.successful += value.successful;
                        instruction_stats.failed += value.failed;
                        instruction_stats.failed_check += value.failed_check;
                    })
                    .or_insert_with(|| IterationStats {
                        invoked: value.invoked,
                        successful: value.successful,
                        failed: value.failed,
                        failed_check: value.failed_check,
                    });
            }
        }
    }
    /// Displays the collected statistics in a formatted table.
    pub fn show_table(&self) {
        let mut table = Table::new();
        table.add_row(row![
            "Instruction",
            "Invoked Total",
            "Ix Success",
            "Check Failed",
            "Ix Failed"
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
}
