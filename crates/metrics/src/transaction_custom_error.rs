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
        for (error, metadata) in other.errors.iter() {
            self.errors
                .entry(*error)
                .and_modify(|metadata| metadata.occurrences += metadata.occurrences)
                .or_insert(metadata.clone());
        }
    }
}
