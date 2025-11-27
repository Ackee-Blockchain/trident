use crate::trident::Trident;
use solana_sdk::pubkey::Pubkey;

impl Trident {
    /// Records a value in a histogram metric
    ///
    /// Histogram metrics track the distribution of values over time,
    /// useful for measuring performance characteristics like execution times,
    /// gas usage, or other numerical distributions.
    ///
    /// # Arguments
    /// * `metric_name` - Name of the histogram metric
    /// * `value` - The value to record in the histogram
    ///
    /// # Note
    /// Metrics are only recorded when the FUZZING_METRICS environment variable is set
    pub fn record_histogram(&mut self, metric_name: &str, value: f64) {
        let metrics = std::env::var("FUZZING_METRICS");
        if metrics.is_ok() {
            self.fuzzing_data.add_to_histogram(metric_name, value);
        }
    }

    /// Records a value in an accumulator metric
    ///
    /// Accumulator metrics sum up values over time, useful for tracking
    /// totals like total gas consumed, total tokens transferred, etc.
    ///
    /// # Arguments
    /// * `metric_name` - Name of the accumulator metric
    /// * `value` - The value to add to the accumulator
    ///
    /// # Note
    /// Metrics are only recorded when the FUZZING_METRICS environment variable is set
    pub fn record_accumulator(&mut self, metric_name: &str, value: f64) {
        let metrics = std::env::var("FUZZING_METRICS");
        if metrics.is_ok() {
            self.fuzzing_data.add_to_accumulator(metric_name, value);
        }
    }

    /// Tracks account state for regression testing
    ///
    /// This method captures the current state of an account for regression
    /// analysis, allowing you to detect when account states change unexpectedly
    /// between fuzzing runs.
    ///
    /// # Arguments
    /// * `account` - The public key of the account to track
    /// * `account_name` - A descriptive name for the account
    ///
    /// # Note
    /// Account tracking is only enabled when the FUZZING_REGRESSION environment variable is set
    pub fn track_account_regression(&mut self, account: &Pubkey, account_name: &str) {
        let regression = std::env::var("FUZZING_REGRESSION");
        if regression.is_ok() {
            let account_shared_data = self.client.get_account(account).unwrap_or_default();
            self.fuzzing_data.add_to_regression(
                &hex::encode(self.rng.get_seed()),
                account_name,
                &account_shared_data,
            );
        }
    }
}
