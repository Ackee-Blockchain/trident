#![allow(dead_code)]

use prettytable::row;
use prettytable::Table;
use solana_sdk::account::AccountSharedData;
use solana_sdk::pubkey::Pubkey;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

mod custom_metrics;
mod dashboard;
mod state_monitoring;
mod transaction_custom_error;
mod transaction_error;
mod transaction_invariants;
mod transaction_panics;
pub mod types;
use types::Seed;

use crate::custom_metrics::CustomMetricValue;
use crate::dashboard::DashboardConfig;
use crate::state_monitoring::StateMonitor;
use crate::transaction_custom_error::TransactionCustomErrorMetrics;
use crate::transaction_error::TransactionErrorMetrics;
use crate::transaction_invariants::TransactionInvariantMetrics;
use crate::transaction_panics::TransactionPanicMetrics;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
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

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct FuzzingStatistics {
    // Master seed - always included in serialization
    #[serde(default)]
    master_seed: Option<String>,
    transactions: BTreeMap<String, TransactionStats>,
    // New field for custom metrics without breaking existing API
    #[serde(default)]
    custom_metrics: BTreeMap<String, CustomMetricValue>,
    // State snapshots hash - only included if snapshots exist
    #[serde(skip_serializing_if = "Option::is_none")]
    state_snapshots_hash: Option<String>,

    #[serde(skip)]
    state_monitor: StateMonitor,
}

impl FuzzingStatistics {
    pub fn new() -> Self {
        let empty_transactions = BTreeMap::<String, TransactionStats>::default();
        Self {
            master_seed: None,
            transactions: empty_transactions,
            custom_metrics: BTreeMap::default(),
            state_snapshots_hash: None,
            state_monitor: StateMonitor::default(),
        }
    }

    pub fn add_master_seed(&mut self, seed: &str) {
        self.master_seed = Some(seed.to_string());
    }

    pub fn with_master_seed(seed: &str) -> Self {
        Self {
            master_seed: Some(seed.to_string()),
            transactions: BTreeMap::default(),
            custom_metrics: BTreeMap::default(),
            state_snapshots_hash: None,
            state_monitor: StateMonitor::default(),
        }
    }

    pub fn monitor_account_state(
        &mut self,
        iteration_seed: &str,
        account_name: &str,
        address: &Pubkey,
        account_shared_data: &AccountSharedData,
    ) {
        self.state_monitor.monitor_account_state(
            iteration_seed,
            account_name,
            address,
            account_shared_data,
        );
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

    pub fn add_failed_invariant(
        &mut self,
        transaction: &str,
        seed: &Seed,
        error: String,
        transaction_inputs: String,
    ) {
        self.transactions
            .entry(transaction.to_string())
            .and_modify(|iterations_stats| {
                iterations_stats.transaction_failed_invariant += 1;
                iterations_stats
                    .transactions_invariant_fails
                    .add_failed_invariant(&error, seed, transaction_inputs);
            });
    }

    pub fn add_transaction_panicked(
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

        // Create a copy with the state hash included and finalize custom metrics
        let mut stats_with_hash = self.clone();
        stats_with_hash.state_snapshots_hash = self.state_monitor.get_state_hash().unwrap_or(None);

        // Finalize all histogram metrics for proper serialization
        for metric in stats_with_hash.custom_metrics.values_mut() {
            metric.finalize_histogram();
        }

        let serialized = serde_json::to_string_pretty(&stats_with_hash).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    /// Merges statistics from another FuzzingStatistics instance into this one.
    /// # Arguments
    /// * `other` - The other FuzzingStatistics instance to merge from.
    pub fn merge_from(&mut self, other: &FuzzingStatistics) {
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
                .or_insert_with(|| stats.clone());
        }

        self.state_monitor.merge_from(&other.state_monitor);
    }

    // Dashboard-related methods

    /// Add to accumulator metric
    pub fn add_to_accumulator(&mut self, metric_name: &str, value: f64) {
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
    pub fn add_to_histogram(&mut self, metric_name: &str, value: f64) {
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

    /// Generate an HTML dashboard with the collected statistics
    pub fn generate_dashboard_html(&self, path: &str) -> std::io::Result<()> {
        self.generate_dashboard_html_with_config(path, &DashboardConfig::with_default_title())
    }

    /// Generate an HTML dashboard with custom configuration
    pub fn generate_dashboard_html_with_config(
        &self,
        path: &str,
        config: &DashboardConfig,
    ) -> std::io::Result<()> {
        let html_content = self.create_dashboard_html(config);
        let mut file = File::create(path)?;
        file.write_all(html_content.as_bytes())?;
        Ok(())
    }

    /// Create the HTML content for the dashboard
    fn create_dashboard_html(&self, _config: &DashboardConfig) -> String {
        let template = include_str!("dashboard-template/dashboard_template.html");

        // Transform the internal data structure to match the expected dashboard format
        let dashboard_data = self.create_dashboard_data_structure();
        let json_data = serde_json::to_string_pretty(&dashboard_data).unwrap();

        template.replace("{{JSON_DATA}}", &json_data)
    }

    /// Transform internal transaction stats to dashboard-friendly format
    fn create_dashboard_data_structure(&self) -> serde_json::Value {
        // Create a mutable copy to finalize histograms for display
        let mut custom_metrics_for_display = self.custom_metrics.clone();
        for metric in custom_metrics_for_display.values_mut() {
            metric.finalize_histogram();
        }
        let mut instructions = serde_json::Map::new();

        for (transaction_name, stats) in &self.transactions {
            let mut instruction_data = serde_json::Map::new();
            instruction_data.insert("invoked".to_string(), stats.transaction_invoked.into());
            instruction_data.insert(
                "transactions_successful".to_string(),
                stats.transaction_successful.into(),
            );
            instruction_data.insert(
                "transactions_failed".to_string(),
                stats.transaction_failed.into(),
            );
            instruction_data.insert(
                "transactions_failed_invariant".to_string(),
                stats.transaction_failed_invariant.into(),
            );
            instruction_data.insert(
                "transactions_panicked".to_string(),
                stats.transaction_panicked.into(),
            );

            // Convert internal error structures to dashboard format
            let transactions_errors = stats.transactions_errors.to_dashboard_format();
            let custom_instruction_errors = stats.custom_instruction_errors.to_dashboard_format();
            let transactions_panics = stats.transactions_panics.to_dashboard_format();
            let transactions_invariant_fails =
                stats.transactions_invariant_fails.to_dashboard_format();

            instruction_data.insert("transactions_errors".to_string(), transactions_errors);
            instruction_data.insert(
                "custom_instruction_errors".to_string(),
                custom_instruction_errors,
            );
            instruction_data.insert("transactions_panics".to_string(), transactions_panics);
            instruction_data.insert(
                "transactions_invariant_fails".to_string(),
                transactions_invariant_fails,
            );

            instructions.insert(transaction_name.clone(), instruction_data.into());
        }

        let mut result = serde_json::Map::new();
        result.insert("instructions".to_string(), instructions.into());
        result.insert(
            "custom_metrics".to_string(),
            serde_json::to_value(&custom_metrics_for_display).unwrap_or_default(),
        );

        // Always include master seed (null if not set)
        result.insert("master_seed".to_string(), self.master_seed.clone().into());

        // Include state snapshots hash if available
        if let Ok(Some(state_hash)) = self.state_monitor.get_state_hash() {
            result.insert("state_snapshots_hash".to_string(), state_hash.into());
        }

        result.into()
    }

    pub fn generate(&self) -> std::io::Result<()> {
        if let Ok(file_name) = std::env::var("FUZZING_METRICS") {
            self.show_table();
            self.print_to_file(&file_name);

            if let Ok(dashboard_file_name) = std::env::var("FUZZING_DASHBOARD") {
                self.generate_dashboard_html(&dashboard_file_name)?;
            }

            if let Ok(state_monitor_file_name) = std::env::var("FUZZING_STATE_MONITOR") {
                self.state_monitor.generate(&state_monitor_file_name)?;
            }
        }
        Ok(())
    }

    pub fn get_exit_code(&self) -> i32 {
        for stats in self.transactions.values() {
            if stats.transaction_failed_invariant > 0 || stats.transaction_panicked > 0 {
                return 99;
            }
        }
        0
    }
}
