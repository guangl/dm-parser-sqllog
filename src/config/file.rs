use serde::Deserialize;
use std::path::Path;

use crate::{
    config::{error_exporter::ErrorExporterConfig, logging::LogConfig},
    error::{LogError, LogResult},
};

#[derive(Debug, Deserialize)]
pub struct Root {
    pub logging: Option<LogConfig>,
    pub error_exporter: Option<ErrorExporterConfig>,
}

impl Root {
    pub fn new() -> Self {
        Self {
            logging: None,
            error_exporter: None,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> LogResult<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_toml_str(&content)
    }

    pub fn from_toml_str(s: &str) -> LogResult<Self> {
        let root: Root = toml::from_str(s)
            .map_err(|e| LogError::Config(format!("failed to parse config.toml: {}", e)))?;
        Ok(root)
    }

    pub fn set_logging(mut self, logging: LogConfig) -> Self {
        self.logging = Some(logging);
        self
    }

    pub fn set_error_exporter(mut self, error_exporter: ErrorExporterConfig) -> Self {
        self.error_exporter = Some(error_exporter);
        self
    }
}
