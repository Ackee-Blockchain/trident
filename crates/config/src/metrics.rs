use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub(crate) struct Metrics {
    pub(crate) fuzzing_with_stats: Option<bool>,
    pub(crate) state_monitor: Option<bool>,
    pub(crate) dashboard: Option<bool>,
}
