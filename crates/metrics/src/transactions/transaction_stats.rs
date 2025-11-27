#![allow(dead_code)]

use prettytable::row;
use prettytable::Table;
use std::collections::BTreeMap;

use crate::transactions::custom_metrics::CustomMetricValue;
use crate::transactions::transaction_custom_error::TransactionCustomErrorMetrics;
use crate::transactions::transaction_error::TransactionErrorMetrics;
use crate::transactions::transaction_panics::TransactionPanicMetrics;
use crate::types::Seed;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct TransactionStats {
    pub(crate) transaction_invoked: u64,
    pub(crate) transaction_successful: u64,
    pub(crate) transaction_failed: u64,
    pub(crate) transaction_panicked: u64,

    pub(crate) transactions_errors: TransactionErrorMetrics,
    pub(crate) custom_instruction_errors: TransactionCustomErrorMetrics,
    pub(crate) transactions_panics: TransactionPanicMetrics,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct FuzzingStatistics {
    pub(crate) transactions: BTreeMap<String, TransactionStats>,
    pub(crate) custom_metrics: BTreeMap<String, CustomMetricValue>,
}

impl FuzzingStatistics {
    pub(crate) fn new() -> Self {
        Self {
            transactions: BTreeMap::default(),
            custom_metrics: BTreeMap::default(),
        }
    }

    pub(crate) fn add_executed_transaction(&mut self, transaction: &str) {
        self.transactions
            .entry(transaction.to_string())
            .and_modify(|iterations_stats| iterations_stats.transaction_invoked += 1)
            .or_insert(TransactionStats {
                transaction_invoked: 1,
                transaction_successful: 0,
                transaction_failed: 0,
                transaction_panicked: 0,
                transactions_errors: TransactionErrorMetrics::default(),
                custom_instruction_errors: TransactionCustomErrorMetrics::default(),
                transactions_panics: TransactionPanicMetrics::default(),
            });
    }

    pub(crate) fn add_successful_transaction(&mut self, transaction: &str) {
        self.transactions
            .entry(transaction.to_string())
            .and_modify(|iterations_stats| iterations_stats.transaction_successful += 1);
    }
    pub(crate) fn add_failed_transaction(
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

    pub(crate) fn add_custom_instruction_error(
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

    pub(crate) fn add_transaction_panicked(
        &mut self,
        transaction: &str,
        seed: Seed,
        panic: String,
        logs: Option<Vec<String>>,
        instruction_inputs: String,
    ) {
        self.transactions
            .entry(transaction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transaction_panicked += 1;
                iterations_stats.transactions_panics.add_transaction_panic(
                    &panic,
                    &seed,
                    logs,
                    instruction_inputs,
                );
            });
    }

    /// Displays the collected statistics in a formatted table.
    pub(crate) fn show_table(&self) {
        let mut table = Table::new();
        table.add_row(row![
            "Instruction",
            "Invoked Total",
            "Ix Success",
            "Ix Failed",
            "Instruction Panicked",
        ]);
        for (instruction, stats) in &self.transactions {
            table.add_row(row![
                instruction,
                stats.transaction_invoked,
                stats.transaction_successful,
                stats.transaction_failed,
                stats.transaction_panicked,
            ]);
        }
        table.printstd();
    }

    /// Merges statistics from another FuzzingStatistics instance into this one.
    /// # Arguments
    /// * `other` - The other FuzzingStatistics instance to merge from.
    pub(crate) fn merge_from(&mut self, other: &FuzzingStatistics) {
        // Merge custom metrics
        for (metric_name, metric_value) in &other.custom_metrics {
            match self.custom_metrics.get_mut(metric_name) {
                Some(existing_metric) => {
                    match (existing_metric, metric_value) {
                        (
                            CustomMetricValue::Accumulator(existing),
                            CustomMetricValue::Accumulator(new),
                        ) => {
                            *existing += new;
                        }
                        (
                            CustomMetricValue::Histogram {
                                min: existing_min,
                                max: existing_max,
                                count: existing_count,
                                sum: existing_sum,
                                values: existing_values,
                                ..
                            },
                            CustomMetricValue::Histogram {
                                min: new_min,
                                max: new_max,
                                count: new_count,
                                sum: new_sum,
                                values: new_values,
                                ..
                            },
                        ) => {
                            // Update min/max
                            *existing_min = existing_min.min(*new_min);
                            *existing_max = existing_max.max(*new_max);
                            // Update sum and count efficiently
                            *existing_sum += new_sum;
                            *existing_count += new_count;
                            // Merge values (avg and median will be calculated when needed)
                            existing_values.extend(new_values);
                        }
                        _ => {
                            // Mismatched types, keep the existing one
                        }
                    }
                }
                None => {
                    self.custom_metrics
                        .insert(metric_name.to_string(), metric_value.clone());
                }
            }
        }

        for (transaction, stats) in &other.transactions {
            self.transactions
                .entry(transaction.to_string())
                .and_modify(|existing_stats| {
                    existing_stats.transaction_invoked += stats.transaction_invoked;
                    existing_stats.transaction_successful += stats.transaction_successful;
                    existing_stats.transaction_failed += stats.transaction_failed;
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
                })
                .or_insert_with(|| stats.clone());
        }

        // self.state_monitor.merge_from(&other.state_monitor);
    }

    // Dashboard-related methods

    /// Add to accumulator metric
    pub(crate) fn add_to_accumulator(&mut self, metric_name: &str, value: f64) {
        self.custom_metrics
            .entry(metric_name.to_string())
            .and_modify(|metric| {
                if let CustomMetricValue::Accumulator(ref mut total) = metric {
                    *total += value;
                }
            })
            .or_insert(CustomMetricValue::Accumulator(value));
    }

    /// Add value to histogram for distribution tracking
    pub(crate) fn add_to_histogram(&mut self, metric_name: &str, value: f64) {
        self.custom_metrics
            .entry(metric_name.to_string())
            .and_modify(|metric| {
                metric.add_to_histogram(value);
            })
            .or_insert_with(|| {
                let mut values: Vec<f64> = Vec::with_capacity(10_000);
                values.push(value);

                CustomMetricValue::Histogram {
                    min: value,
                    max: value,
                    count: 1,
                    sum: value,
                    avg: 0.0,     // Will be calculated when needed
                    median: 0.0,  // Will be calculated when needed
                    entropy: 0.0, // Will be calculated when needed
                    values,
                }
            });
    }

    pub(crate) fn get_exit_code(&self) -> i32 {
        for stats in self.transactions.values() {
            if stats.transaction_panicked > 0 {
                return 99;
            }
        }
        0
    }
}
