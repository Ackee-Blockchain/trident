use crate::error::Result;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Copy)]
pub enum DisplayFormat {
    Cli,
    JSONPretty,
    JSON,
}

impl DisplayFormat {
    pub fn formatted_string<T>(&self, item: &T) -> Result<String>
    where
        T: fmt::Display + Serialize,
    {
        match self {
            DisplayFormat::Cli => Ok(format!("{item}")),
            DisplayFormat::JSONPretty => Ok(serde_json::to_string_pretty(&item)?),
            DisplayFormat::JSON => Ok(serde_json::to_string(&item)?),
        }
    }
}
