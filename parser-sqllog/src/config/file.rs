use serde::Deserialize;
use std::{fs, path::Path};

use crate::{
    config::{error_exporter::ErrorExporterConfig, logging::LogConfig, sqllog::SqllogConfig},
    error::ConfigParseError,
};

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Root {
    pub logging: LogConfig,
    pub error_exporter: ErrorExporterConfig,
    pub sqllog: SqllogConfig,
}

impl Root {
    pub fn new() -> Self {
        Self {
            logging: LogConfig::default(),
            error_exporter: ErrorExporterConfig::default(),
            sqllog: SqllogConfig::default(),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let content = fs::read_to_string(path)
            .map_err(ConfigParseError::Io)
            .unwrap_or_default();
        Self::from_toml_str(&content)
    }

    pub fn from_toml_str(s: &str) -> Self {
        let root: Root = toml::from_str(s)
            .map_err(ConfigParseError::Parser)
            .unwrap_or_default();
        root
    }

    pub fn set_logging(mut self, logging: LogConfig) -> Self {
        self.logging = logging;
        self
    }

    pub fn set_error_exporter(mut self, error_exporter: ErrorExporterConfig) -> Self {
        self.error_exporter = error_exporter;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_from_toml_str() {
        let toml_str = r#"
            [logging]
            level = "info"
            path = "logs/app.log"

            [error_exporter]
            path = "error_logs"
            overwrite = true
            append = false
        "#;

        let root = Root::from_toml_str(toml_str);

        let logging = root.logging;
        assert_eq!(logging.level, "info");
        assert_eq!(logging.path, "logs/app.log");

        let error_exporter = root.error_exporter;
        assert_eq!(error_exporter.error_log_path, "error_logs");
        assert!(error_exporter.overwrite);
        assert!(!error_exporter.append);
    }

    #[test]
    fn test_root_from_toml_str_with_missing_sections() {
        let toml_str = r#"
            [logging]
            level = "debug"
            path = "logs/debug.log"
        "#;

        let root = Root::from_toml_str(toml_str);

        let logging = root.logging;
        assert_eq!(logging.level, "debug");
        assert_eq!(logging.path, "logs/debug.log");

        let error_exporter = root.error_exporter;
        assert_eq!(error_exporter.error_log_path, "error_logs".to_string());
        assert!(!error_exporter.overwrite);
        assert!(error_exporter.append);
    }

    #[test]
    fn test_root_setters() {
        let logging = LogConfig::new().set_level("warn").set_path("logs/warn.log");
        let error_exporter = ErrorExporterConfig::new()
            .set_error_log_path("error_logs")
            .set_overwrite(true)
            .set_append(false);

        let root = Root::new()
            .set_logging(logging.clone())
            .set_error_exporter(error_exporter.clone());

        assert_eq!(root.logging.clone().level, logging.level);
        assert_eq!(root.logging.clone().path, logging.path);

        assert_eq!(
            root.error_exporter.clone().error_log_path,
            error_exporter.error_log_path
        );
        assert_eq!(
            root.error_exporter.clone().overwrite,
            error_exporter.overwrite
        );
        assert_eq!(root.error_exporter.clone().append, error_exporter.append);
    }
}
