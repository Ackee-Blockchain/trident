#![allow(dead_code)]

use prettytable::row;
use prettytable::Table;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

mod transaction_custom_error;
mod transaction_error;
mod transaction_invariants;
mod transaction_panics;
pub mod types;
use types::Seed;

use crate::transaction_custom_error::TransactionCustomErrorMetrics;
use crate::transaction_error::TransactionErrorMetrics;
use crate::transaction_invariants::TransactionInvariantMetrics;
use crate::transaction_panics::TransactionPanicMetrics;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct TransactionStats {
    transaction_invoked: u64,
    transaction_successful: u64,
    transaction_failed: u64,
    transaction_failed_invariant: u64,
    transaction_panicked: u64,

    transactions_errors: TransactionErrorMetrics,
    custom_instruction_errors: TransactionCustomErrorMetrics,
    transactions_panics: TransactionPanicMetrics,
    transactions_invariant_fails: TransactionInvariantMetrics,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FuzzingStatistics {
    transactions: BTreeMap<String, TransactionStats>,
}

impl FuzzingStatistics {
    pub fn new() -> Self {
        let empty_transactions = BTreeMap::<String, TransactionStats>::default();
        Self {
            transactions: empty_transactions,
        }
    }
    pub fn add_executed_transaction(&mut self, transaction: &str) {
        self.transactions
            .entry(transaction.to_string())
            .and_modify(|iterations_stats| iterations_stats.transaction_invoked += 1)
            .or_insert(TransactionStats {
                transaction_invoked: 1,
                transaction_successful: 0,
                transaction_failed: 0,
                transaction_failed_invariant: 0,
                transaction_panicked: 0,
                transactions_errors: TransactionErrorMetrics::default(),
                custom_instruction_errors: TransactionCustomErrorMetrics::default(),
                transactions_panics: TransactionPanicMetrics::default(),
                transactions_invariant_fails: TransactionInvariantMetrics::default(),
            });
    }

    pub fn add_successful_transaction(&mut self, transaction: &str) {
        self.transactions
            .entry(transaction.to_string())
            .and_modify(|iterations_stats| iterations_stats.transaction_successful += 1);
    }
    pub fn add_failed_transaction(
        &mut self,
        transaction: &str,
        error: String,
        logs: Option<Vec<String>>,
    ) {
        self.transactions
            .entry(transaction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transaction_failed += 1;
                iterations_stats.transactions_errors.add_error(&error, logs);
            });
    }

    pub fn add_custom_instruction_error(
        &mut self,
        transaction: &str,
        error_code: &u32,
        logs: Option<Vec<String>>,
    ) {
        self.transactions
            .entry(transaction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transaction_failed += 1;
                iterations_stats
                    .custom_instruction_errors
                    .add_error(error_code, logs);
            });
    }

    pub fn add_failed_invariant(&mut self, transaction: &str, seed: &Seed, error: String) {
        self.transactions
            .entry(transaction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transaction_failed_invariant += 1;
                iterations_stats
                    .transactions_invariant_fails
                    .add_failed_invariant(&error, seed);
            });
    }

    pub fn add_transaction_panicked(
        &mut self,
        transaction: &str,
        seed: Seed,
        panic: String,
        logs: Option<Vec<String>>,
    ) {
        self.transactions
            .entry(transaction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transaction_panicked += 1;
                iterations_stats
                    .transactions_panics
                    .add_transaction_panic(&panic, &seed, logs);
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
        for (instruction, stats) in &self.transactions {
            table.add_row(row![
                instruction,
                stats.transaction_invoked,
                stats.transaction_successful,
                stats.transaction_failed,
                stats.transaction_failed_invariant,
                stats.transaction_panicked,
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
        for (transaction, stats) in other.transactions {
            self.transactions
                .entry(transaction.to_string())
                .and_modify(|existing_stats| {
                    existing_stats.transaction_invoked += stats.transaction_invoked;
                    existing_stats.transaction_successful += stats.transaction_successful;
                    existing_stats.transaction_failed += stats.transaction_failed;
                    existing_stats.transaction_failed_invariant +=
                        stats.transaction_failed_invariant;
                    existing_stats.transaction_panicked += stats.transaction_panicked;
                    existing_stats
                        .transactions_errors
                        .concat(&stats.transactions_errors);
                    existing_stats
                        .custom_instruction_errors
                        .concat(&stats.custom_instruction_errors);
                    existing_stats
                        .transactions_panics
                        .concat(&stats.transactions_panics);
                    existing_stats
                        .transactions_invariant_fails
                        .concat(&stats.transactions_invariant_fails);
                })
                .or_insert(stats);
        }
    }
}
