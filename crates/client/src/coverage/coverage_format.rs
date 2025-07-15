use crate::coverage::constants::*;
use crate::coverage::CoverageError;

#[derive(Debug, Clone, PartialEq)]
pub enum CoverageFormat {
    Json,
    Html,
}

impl CoverageFormat {
    pub fn from_str(s: &str) -> Result<Self, CoverageError> {
        match s.to_lowercase().as_str() {
            "json" => Ok(CoverageFormat::Json),
            "html" => Ok(CoverageFormat::Html),
            _ => Err(CoverageError::InvalidReportFormat),
        }
    }

    pub fn get_report_filename(&self) -> &str {
        match self {
            CoverageFormat::Json => JSON_REPORT_FILENAME,
            CoverageFormat::Html => HTML_REPORT_DIRNAME,
        }
    }

    pub fn get_cargo_arg(&self) -> &str {
        match self {
            CoverageFormat::Json => "--json",
            CoverageFormat::Html => "--html",
        }
    }
}