/// Custom metric value types for tracking domain-specific statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CustomMetricValue {
    Accumulator(f64),
    Histogram {
        min: f64,
        max: f64,
        count: u64,
        sum: f64, // Track sum for efficient average calculation
        avg: f64,
        median: f64,
        entropy: f64,
        #[serde(skip)]
        values: Vec<f64>,
    },
}

impl CustomMetricValue {
    pub fn add_to_accumulator(&mut self, value: f64) {
        match self {
            Self::Accumulator(accumulator) => *accumulator += value,
            _ => panic!("Cannot add to non-accumulator metric"),
        }
    }

    pub fn add_to_histogram(&mut self, value: f64) {
        match self {
            Self::Histogram {
                min,
                max,
                count,
                sum,
                values,
                ..
            } => {
                *min = min.min(value);
                *max = max.max(value);
                *sum += value;
                *count += 1;
                values.push(value);
                if values.len() == values.capacity() {
                    values.reserve(10_000);
                }
                // Note: We don't sort here! Only when needed for median calculation
            }
            _ => panic!("Cannot add to non-histogram metric"),
        }
    }

    /// Calculate average efficiently from sum and count
    pub fn get_avg(&self) -> f64 {
        match self {
            Self::Histogram { count, sum, .. } => {
                if *count > 0 {
                    *sum / (*count as f64)
                } else {
                    0.0
                }
            }
            _ => panic!("Cannot get average from non-histogram metric"),
        }
    }

    /// Calculate median (sorts values only when needed)
    pub fn get_median(&self) -> f64 {
        match self {
            Self::Histogram { values, .. } => {
                if values.is_empty() {
                    return 0.0;
                }

                let mut sorted_values = values.clone();
                sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let len = sorted_values.len();
                if len % 2 == 0 {
                    (sorted_values[len / 2 - 1] + sorted_values[len / 2]) / 2.0
                } else {
                    sorted_values[len / 2]
                }
            }
            _ => panic!("Cannot get median from non-histogram metric"),
        }
    }

    /// Calculate Shannon's entropy for a list of values
    /// Shannon entropy: H(X) = -Σ p(x) * log2(p(x))
    /// where p(x) is the probability of value x occurring
    pub fn calculate_shannon_entropy(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        // Create frequency map
        let mut frequency_map = std::collections::HashMap::new();
        for &value in values {
            *frequency_map.entry(value.to_bits()).or_insert(0) += 1;
        }

        let total_count = values.len() as f64;
        let mut entropy = 0.0;

        for &count in frequency_map.values() {
            let probability = count as f64 / total_count;
            if probability > 0.0 {
                entropy -= probability * probability.log2();
            }
        }

        entropy
    }

    /// Update computed values for serialization
    pub fn finalize_histogram(&mut self) {
        if let Self::Histogram {
            count,
            sum,
            values,
            avg,
            median,
            entropy,
            ..
        } = self
        {
            // Calculate average efficiently from sum and count
            *avg = if *count > 0 {
                *sum / (*count as f64)
            } else {
                0.0
            };

            // Calculate median (sorts values only when needed)
            *median = if values.is_empty() {
                0.0
            } else {
                let mut sorted_values = values.clone();
                sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let len = sorted_values.len();
                if len % 2 == 0 {
                    (sorted_values[len / 2 - 1] + sorted_values[len / 2]) / 2.0
                } else {
                    sorted_values[len / 2]
                }
            };

            // Calculate entropy (only when needed for dashboard)
            *entropy = Self::calculate_shannon_entropy(values);
        }
    }
}
