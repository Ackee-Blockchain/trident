use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Argument {
    pub short_opt: Option<String>,
    pub long_opt: Option<String>,
    // for value type options, string is the value
    // for flag type options, it will be None
    pub value: Option<String>,
}

impl Argument {
    pub(crate) fn new(short_opt: &str, long_opt: &str, val: Option<&str>) -> Self {
        let short_opt = if short_opt.is_empty() {
            None
        } else {
            Some(short_opt.to_owned())
        };
        let long_opt = if long_opt.is_empty() {
            None
        } else {
            Some(long_opt.to_owned())
        };
        let value = val.map(|value| value.to_string());
        Self {
            short_opt,
            long_opt,
            value,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
}

impl EnvironmentVariable {
    pub(crate) fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
