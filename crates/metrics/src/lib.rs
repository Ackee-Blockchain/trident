#![allow(dead_code)]

mod regression;
mod transactions;
pub mod types;
use std::fs::File;
use std::io::Write;

use solana_sdk::account::AccountSharedData;
use types::Seed;

use crate::regression::regression::FuzzingRegression;
use crate::transactions::transaction_stats::FuzzingStatistics;

pub use crate::regression::compare::compare_regression_files;
pub use crate::regression::compare::ComparisonResult;

#[derive(Clone, Default)]
pub struct TridentFuzzingData {
    master_seed: Option<String>,
    metrics: FuzzingStatistics,
    regression: FuzzingRegression,
}

// Metrics
impl TridentFuzzingData {
    pub fn add_to_histogram(&mut self, metric_name: &str, value: f64) {
        self.metrics.add_to_histogram(metric_name, value);
    }

    pub fn add_to_accumulator(&mut self, metric_name: &str, value: f64) {
        self.metrics.add_to_accumulator(metric_name, value);
    }

    pub fn add_executed_transaction(&mut self, transaction_name: &str) {
        self.metrics.add_executed_transaction(transaction_name);
    }

    pub fn add_successful_transaction(&mut self, transaction_name: &str) {
        self.metrics.add_successful_transaction(transaction_name);
    }

    pub fn add_failed_transaction(
        &mut self,
        transaction_name: &str,
        error: String,
        logs: Option<Vec<String>>,
    ) {
        self.metrics
            .add_failed_transaction(transaction_name, error, logs);
    }

    pub fn add_failed_invariant(
        &mut self,
        transaction_name: &str,
        seed: &Seed,
        error: String,
        transaction_inputs: String,
    ) {
        self.metrics
            .add_failed_invariant(transaction_name, seed, error, transaction_inputs);
    }

    pub fn add_transaction_panicked(
        &mut self,
        transaction_name: &str,
        seed: Seed,
        panic: String,
        logs: Option<Vec<String>>,
        instruction_inputs: String,
    ) {
        self.metrics.add_transaction_panicked(
            transaction_name,
            seed,
            panic,
            logs,
            instruction_inputs,
        );
    }

    pub fn add_custom_instruction_error(
        &mut self,
        transaction_name: &str,
        error_code: &u32,
        logs: Option<Vec<String>>,
    ) {
        self.metrics
            .add_custom_instruction_error(transaction_name, error_code, logs);
    }
}

// Regression
impl TridentFuzzingData {
    pub fn add_to_regression(
        &mut self,
        iteration_seed: &str,
        account_name: &str,
        account_shared_data: &AccountSharedData,
    ) {
        self.regression
            .add_to_regression(iteration_seed, account_name, account_shared_data);
    }
}

// Master seed
impl TridentFuzzingData {
    pub fn with_master_seed(seed: Seed) -> Self {
        Self {
            master_seed: Some(hex::encode(seed)),
            metrics: FuzzingStatistics::new(),
            regression: FuzzingRegression::default(),
        }
    }
    pub fn add_master_seed(&mut self, seed: &str) {
        self.master_seed = Some(seed.to_string());
    }
}

// Generation
impl TridentFuzzingData {
    pub fn generate(&self) -> std::io::Result<()> {
        // Generate metrics JSON if FUZZING_METRICS is set
        if std::env::var("FUZZING_METRICS").is_ok() {
            self.metrics.show_table();

            if let Ok(metrics_file_name) = std::env::var("FUZZING_JSON") {
                self.to_json(&metrics_file_name);
            }

            // Generate HTML dashboard if FUZZING_DASHBOARD is set
            if let Ok(dashboard_file_name) = std::env::var("FUZZING_DASHBOARD") {
                self.to_html(&dashboard_file_name)?;
            }
        }

        // Generate regression JSON if FUZZING_REGRESSION is set
        if let Ok(regression_file_name) = std::env::var("FUZZING_REGRESSION") {
            self.to_regression_json(&regression_file_name)?;
        }

        Ok(())
    }

    fn to_json(&self, path: &str) {
        let mut file = File::create(path).unwrap();

        // Create a copy and finalize custom metrics for proper serialization
        let mut metrics_copy = self.metrics.clone();
        for metric in metrics_copy.custom_metrics.values_mut() {
            metric.finalize_histogram();
        }

        // Create the data structure for JSON serialization with clean structure
        let mut json_data = serde_json::Map::new();

        // Include master seed at the start
        json_data.insert("master_seed".to_string(), self.master_seed.clone().into());

        // Include all metrics data
        json_data.insert(
            "transactions".to_string(),
            serde_json::to_value(&metrics_copy.transactions).unwrap_or_default(),
        );
        json_data.insert(
            "custom_metrics".to_string(),
            serde_json::to_value(&metrics_copy.custom_metrics).unwrap_or_default(),
        );

        // Include state hash at the end if regression is enabled and state exists
        if let Ok(state_hash) = self.regression.get_snapshots_hash() {
            json_data.insert("state_snapshots_hash".to_string(), state_hash.into());
        }

        let serialized = serde_json::to_string_pretty(&json_data).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    fn to_html(&self, path: &str) -> std::io::Result<()> {
        let html_content = self.create_dashboard_html();
        let mut file = File::create(path)?;
        file.write_all(html_content.as_bytes())?;
        Ok(())
    }

    /// Create the HTML content for the dashboard
    fn create_dashboard_html(&self) -> String {
        let template = include_str!("dashboard-template/dashboard_template.html");

        // Transform the internal data structure to match the expected dashboard format
        let dashboard_data = self.create_dashboard_data_structure();
        let json_data = serde_json::to_string_pretty(&dashboard_data).unwrap();

        template.replace("{{JSON_DATA}}", &json_data)
    }
    /// Transform internal transaction stats to dashboard-friendly format
    fn create_dashboard_data_structure(&self) -> serde_json::Value {
        // Create a mutable copy to finalize histograms for display
        let mut custom_metrics_for_display = self.metrics.custom_metrics.clone();
        for metric in custom_metrics_for_display.values_mut() {
            metric.finalize_histogram();
        }
        let mut instructions = serde_json::Map::new();

        for (transaction_name, stats) in &self.metrics.transactions {
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
        if let Ok(state_hash) = self.regression.get_snapshots_hash() {
            result.insert("state_snapshots_hash".to_string(), state_hash.into());
        }

        result.into()
    }

    fn to_regression_json(&self, path: &str) -> std::io::Result<()> {
        // Use the existing regression generate method
        self.regression.generate(path)
    }
}

impl TridentFuzzingData {
    pub fn _merge(&mut self, other: TridentFuzzingData) {
        self.metrics.merge_from(&other.metrics);
        self.regression.merge_from(&other.regression);
    }

    pub fn get_exit_code(&self) -> i32 {
        self.metrics.get_exit_code()
    }
}
