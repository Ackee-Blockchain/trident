use std::collections::BTreeMap;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct TransactionCustomErrorMetrics {
    errors: BTreeMap<u32, TransactionCustomErrorMetricsMetadata>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct TransactionCustomErrorMetricsMetadata {
    occurrences: u64,
    logs: Option<Vec<String>>,
}

impl TransactionCustomErrorMetrics {
    pub(crate) fn add_error(&mut self, error: &u32, logs: Option<Vec<String>>) {
        self.errors
            .entry(*error)
            .and_modify(|metadata| metadata.occurrences += 1)
            .or_insert(TransactionCustomErrorMetricsMetadata {
                occurrences: 1,
                logs,
            });
    }

    pub(crate) fn concat(&mut self, other: &TransactionCustomErrorMetrics) {
        for (other_error, other_metadata) in other.errors.iter() {
            self.errors
                .entry(*other_error)
                .and_modify(|metadata| metadata.occurrences += other_metadata.occurrences)
                .or_insert(other_metadata.clone());
        }
    }

    pub(crate) fn to_dashboard_format(&self) -> serde_json::Value {
        serde_json::to_value(&self.errors).unwrap_or_default()
    }
}
