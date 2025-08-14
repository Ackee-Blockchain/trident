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
    instruction_inputs: String,
}

impl TransactionPanicMetrics {
    pub(crate) fn add_transaction_panic(
        &mut self,
        panic: &str,
        seed: &Seed,
        logs: Option<Vec<String>>,
        instruction_inputs: String,
    ) {
        self.panics
            .entry(panic.to_string())
            .and_modify(|metadata| metadata.occurrences += 1)
            .or_insert(TransactionPanicMetricsMetadata {
                occurrences: 1,
                seed: hex::encode(seed),
                logs,
                instruction_inputs,
            });
    }

    pub(crate) fn concat(&mut self, other: &TransactionPanicMetrics) {
        for (other_panic, other_metadata) in other.panics.iter() {
            self.panics
                .entry(other_panic.to_string())
                .and_modify(|metadata| metadata.occurrences += other_metadata.occurrences)
                .or_insert(other_metadata.clone());
        }
    }

    pub(crate) fn to_dashboard_format(&self) -> serde_json::Value {
        serde_json::to_value(&self.panics).unwrap_or_default()
    }
}
