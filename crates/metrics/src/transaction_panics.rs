use std::collections::BTreeMap;

use crate::types::Seed;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct TransactionPanicMetrics {
    panics: BTreeMap<String, TransactionPanicMetricsMetadata>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct TransactionPanicMetricsMetadata {
    occurrences: u64,
    seed: String,
    logs: Option<Vec<String>>,
}

impl TransactionPanicMetrics {
    pub(crate) fn add_transaction_panic(
        &mut self,
        panic: &str,
        seed: &Seed,
        logs: Option<Vec<String>>,
    ) {
        self.panics
            .entry(panic.to_string())
            .and_modify(|metadata| metadata.occurrences += 1)
            .or_insert(TransactionPanicMetricsMetadata {
                occurrences: 1,
                seed: hex::encode(seed),
                logs,
            });
    }

    pub(crate) fn concat(&mut self, other: &TransactionPanicMetrics) {
        for (panic, metadata) in other.panics.iter() {
            self.panics
                .entry(panic.to_string())
                .and_modify(|metadata| metadata.occurrences += metadata.occurrences)
                .or_insert(metadata.clone());
        }
    }
}
