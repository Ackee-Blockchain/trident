use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub(crate) struct Regression {
    pub(crate) enabled: Option<bool>,
}
