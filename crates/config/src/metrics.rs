use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct Metrics {
    pub(crate) enabled: Option<bool>,
    pub(crate) json: Option<bool>,
    pub(crate) dashboard: Option<bool>,
}
