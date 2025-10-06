use crate::trident::Trident;
use solana_sdk::pubkey::Pubkey;

impl Trident {
    pub fn add_histogram_metric(&mut self, metric_name: &str, value: f64) {
        let metrics = std::env::var("FUZZING_METRICS");
        if metrics.is_ok() {
            self.fuzzing_data.add_to_histogram(metric_name, value);
        }
    }

    pub fn add_accumulator_metric(&mut self, metric_name: &str, value: f64) {
        let metrics = std::env::var("FUZZING_METRICS");
        if metrics.is_ok() {
            self.fuzzing_data.add_to_accumulator(metric_name, value);
        }
    }

    pub fn add_to_regression(&mut self, account: &Pubkey, account_name: &str) {
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
