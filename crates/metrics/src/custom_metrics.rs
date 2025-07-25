/// Custom metric value types for tracking domain-specific statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CustomMetricValue {
    Accumulator(f64),
    Histogram {
        min: f64,
        max: f64,
        avg: f64,
        median: f64,
        count: u64,
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
                avg,
                median,
                count,
                values,
            } => {
                *min = min.min(value);
                *max = max.max(value);
                *avg = (*avg * (*count as f64) + value) / (*count as f64 + 1.0);
                values.push(value);
                values.sort_by(|a, b| a.partial_cmp(b).unwrap());

                // Calculate median
                let len = values.len();
                *median = if len % 2 == 0 {
                    (values[len / 2 - 1] + values[len / 2]) / 2.0
                } else {
                    values[len / 2]
                };

                *count += 1;
            }
            _ => panic!("Cannot add to non-histogram metric"),
        }
    }
}
