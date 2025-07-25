use std::collections::BTreeMap;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct TransactionErrorMetrics {
    errors: BTreeMap<String, TransactionErrorMetricsMetadata>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct TransactionErrorMetricsMetadata {
    occurrences: u64,
    logs: Option<Vec<String>>,
}

impl TransactionErrorMetrics {
    pub(crate) fn add_error(&mut self, error: &str, logs: Option<Vec<String>>) {
        self.errors
            .entry(error.to_string())
            .and_modify(|metadata| metadata.occurrences += 1)
            .or_insert(TransactionErrorMetricsMetadata {
                occurrences: 1,
                logs,
            });
    }

    pub(crate) fn concat(&mut self, other: &TransactionErrorMetrics) {
        for (error, metadata) in other.errors.iter() {
            self.errors
                .entry(error.to_string())
                .and_modify(|metadata| metadata.occurrences += metadata.occurrences)
                .or_insert(metadata.clone());
        }
    }

    pub(crate) fn to_dashboard_format(&self) -> serde_json::Value {
        serde_json::to_value(&self.errors).unwrap_or_default()
    }
}
