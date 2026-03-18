use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct Regression {
    pub(crate) enabled: Option<bool>,
}
