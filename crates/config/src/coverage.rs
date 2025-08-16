use serde::Deserialize;

use crate::constants::DEFAULT_COVERAGE_FORMAT;
use crate::constants::DEFAULT_COVERAGE_SERVER_PORT;
use crate::constants::DEFAULT_LOOPCOUNT;

#[derive(Debug, Deserialize, Clone)]
pub struct Coverage {
    pub enabled: Option<bool>,
    pub server_port: Option<u16>,
    pub loopcount: Option<u64>,
    pub format: Option<String>,
    pub attach_extension: Option<bool>,
}

impl Default for Coverage {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            server_port: Some(DEFAULT_COVERAGE_SERVER_PORT),
            loopcount: Some(DEFAULT_LOOPCOUNT),
            format: Some(DEFAULT_COVERAGE_FORMAT.to_string()),
            attach_extension: Some(false),
        }
    }
}

impl Coverage {
    pub fn get_enable(&self) -> bool {
        self.enabled.unwrap_or(false)
    }

    pub fn get_server_port(&self) -> u16 {
        self.server_port.unwrap_or(DEFAULT_COVERAGE_SERVER_PORT)
    }

    pub fn get_loopcount(&self) -> u64 {
        self.loopcount.unwrap_or(DEFAULT_LOOPCOUNT)
    }

    pub fn get_format(&self) -> String {
        self.format
            .clone()
            .unwrap_or_else(|| DEFAULT_COVERAGE_FORMAT.to_string())
    }

    pub fn get_attach_extension(&self) -> bool {
        self.attach_extension.unwrap_or(false)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.get_attach_extension() && !self.get_enable() {
            return Err("Cannot attach extension without enabling coverage!".to_string());
        }

        if self.get_attach_extension() && self.get_format() != "json" {
            return Err("Cannot attach extension with format other than json!".to_string());
        }

        Ok(())
    }
}
