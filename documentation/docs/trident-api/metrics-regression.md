# Metrics & Regression Methods

These methods provide functionality for collecting metrics during fuzzing and tracking account states for regression testing.

## Metrics Collection

### `record_histogram`

Records a value in a histogram metric. Only records if fuzzing metrics are enabled.

```rust
pub fn record_histogram(&mut self, metric_name: &str, value: f64)
```

**Parameters:**

- `metric_name` - Name of the histogram metric
- `value` - Value to record

**Description:** Records a value in a histogram metric for statistical analysis. Histograms track the distribution of values over time.

---

### `record_accumulator`

Records a value in an accumulator metric. Only records if fuzzing metrics are enabled.

```rust
pub fn record_accumulator(&mut self, metric_name: &str, value: f64)
```

**Parameters:**

- `metric_name` - Name of the accumulator metric
- `value` - Value to record

**Description:** Records a value in an accumulator metric. Accumulators sum values over time, useful for tracking totals.

---

## Regression Testing

### `track_account_regression`

Tracks an account for regression testing. Only records if fuzzing regression is enabled.

```rust
pub fn track_account_regression(&mut self, account: &Pubkey, account_name: &str)
```

**Parameters:**

- `account` - The account to track
- `account_name` - Name for the account in regression data

**Description:** Captures the current state of an account for regression testing, allowing you to detect unexpected changes in account data between test runs.

---

## Example Usage

```rust
use trident_fuzz::*;

#[flow]
fn test_comprehensive_metrics_and_regression(&mut self) {
    let mint_account = self.random_pubkey();
    let user_account = self.random_pubkey();
    let treasury_account = self.random_pubkey();
    let amount = self.random_from_range(1..1000u64);
    
    // Track performance metrics
    let start_time = std::time::Instant::now();
    
    // Perform your program operations
    // ... your program instructions ...
    
    let duration = start_time.elapsed().as_millis() as f64;
    self.record_histogram("operation_duration_ms", duration);
    
    // Track business logic metrics
    self.record_histogram("token_mint_amount", amount as f64);
    self.record_histogram("fee_collected", 50.0);
    self.record_accumulator("total_volume", amount as f64);
    
    // Track success/failure rates
    let result = self.process_transaction(&[/* instructions */], "Test Transaction");
    if result.is_success() {
        self.record_accumulator("successful_operations", 1.0);
    } else {
        self.record_accumulator("failed_operations", 1.0);
    }
    
    // Track critical accounts for regression testing
    self.track_account_regression(&mint_account, "token_mint");
    self.track_account_regression(&treasury_account, "protocol_treasury");
    self.track_account_regression(&user_account, "user_balance");
}
```


