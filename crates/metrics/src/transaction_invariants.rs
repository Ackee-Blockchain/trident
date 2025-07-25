use std::collections::BTreeMap;

use crate::types::Seed;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct TransactionInvariantMetrics {
    invariants: BTreeMap<String, TransactionInvariantMetricsMetadata>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct TransactionInvariantMetricsMetadata {
    occurrences: u64,
    seed: String,
}

impl TransactionInvariantMetrics {
    pub(crate) fn add_failed_invariant(&mut self, invariant: &str, seed: &Seed) {
        self.invariants
            .entry(invariant.to_string())
            .and_modify(|metadata| metadata.occurrences += 1)
            .or_insert(TransactionInvariantMetricsMetadata {
                occurrences: 1,
                seed: hex::encode(seed),
            });
    }

    pub(crate) fn concat(&mut self, other: &TransactionInvariantMetrics) {
        for (other_invariant, other_metadata) in other.invariants.iter() {
            self.invariants
                .entry(other_invariant.to_string())
                .and_modify(|metadata| metadata.occurrences += other_metadata.occurrences)
                .or_insert(other_metadata.clone());
        }
    }

    pub(crate) fn to_dashboard_format(&self) -> serde_json::Value {
        serde_json::to_value(&self.invariants).unwrap_or_default()
    }
}
