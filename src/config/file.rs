use serde::Deserialize;
use std::{fs, path::Path};

use crate::{
    config::{error_exporter::ErrorExporterConfig, logging::LogConfig},
    error::{ConfigParseError, ConfigParseResult},
};

#[derive(Debug, Deserialize, Default, Clone)]
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

    pub fn from_file<P: AsRef<Path>>(path: P) -> ConfigParseResult<Self> {
        let content = fs::read_to_string(path).map_err(ConfigParseError::Io)?;
        Self::from_toml_str(&content)
    }

    pub fn from_toml_str(s: &str) -> ConfigParseResult<Self> {
        let root: Root = toml::from_str(s).map_err(ConfigParseError::Parser)?;

        if root.logging.is_none() && root.error_exporter.is_none() {
            return Err(ConfigParseError::MissingField(
                "missing both `logging` and `error_exporter` sections".into(),
            ));
        }

        // 如果 logging 存在，但某些必需字段缺失（例如 level），可以返回更具体的错误
        if let Some(ref logging) = root.logging {
            if logging.level.is_empty() {
                return Err(ConfigParseError::MissingField(
                    "logging.level is empty".into(),
                ));
            }
            if logging.path.is_empty() {
                return Err(ConfigParseError::MissingField(
                    "logging.path is empty".into(),
                ));
            }
        }

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

        let root = Root::from_toml_str(toml_str).unwrap();

        assert!(root.logging.is_some());
        let logging = root.logging.unwrap();
        assert_eq!(logging.level, "info");
        assert_eq!(logging.path, "logs/app.log");

        assert!(root.error_exporter.is_some());
        let error_exporter = root.error_exporter.unwrap();
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

        let root = Root::from_toml_str(toml_str).unwrap();

        assert!(root.logging.is_some());
        let logging = root.logging.unwrap();
        assert_eq!(logging.level, "debug");
        assert_eq!(logging.path, "logs/debug.log");

        assert!(root.error_exporter.is_none());
    }

    #[test]
    fn test_root_from_toml_str_with_invalid_toml() {
        let toml_str = r#"
            [logging
            level = "info"
            file_path = "logs/app.log"
        "#;

        let result = Root::from_toml_str(toml_str);
        assert!(result.is_err());

        // 更具体地检查错误类型，确认是 ConfigParse（底层 toml 解析错误）
        match result.unwrap_err() {
            ConfigParseError::Parser(cfg_err) => {
                // TOML 错误信息里应包含语法错误的提示
                assert!(cfg_err.to_string().contains("TOML") || !cfg_err.to_string().is_empty());
            }
            other => panic!("expected ConfigParse error, got: {:?}", other),
        }
    }

    #[test]
    fn test_root_default() {
        let root = Root::new();
        assert!(root.logging.is_none());
        assert!(root.error_exporter.is_none());
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

        assert!(root.logging.is_some());
        assert_eq!(root.logging.clone().unwrap().level, logging.level);
        assert_eq!(root.logging.clone().unwrap().path, logging.path);

        assert!(root.error_exporter.is_some());
        assert_eq!(
            root.error_exporter.clone().unwrap().error_log_path,
            error_exporter.error_log_path
        );
        assert_eq!(
            root.error_exporter.clone().unwrap().overwrite,
            error_exporter.overwrite
        );
        assert_eq!(
            root.error_exporter.clone().unwrap().append,
            error_exporter.append
        );
    }

    #[test]
    fn test_from_file_io_error_not_found() {
        // 使用一个几乎不可能存在的临时文件路径来触发 I/O 错误（NotFound）
        let tmp = std::env::temp_dir().join("this_file_should_not_exist_1234567890.toml");
        // 确保不存在
        let _ = std::fs::remove_file(&tmp);

        let res = Root::from_file(&tmp);
        assert!(res.is_err(), "读取不存在的文件应该返回错误");

        match res {
            Err(ConfigParseError::Io(e)) => {
                assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
            }
            other => panic!("Expected Io error, got: {:?}", other),
        }
    }

    #[test]
    fn test_from_toml_str_returns_config_error_with_message() {
        let toml_str = "not a toml";

        let res = Root::from_toml_str(toml_str);
        assert!(res.is_err());

        let err = res.unwrap_err();
        match err {
            ConfigParseError::Parser(cfg_err) => {
                let s = format!("{}", cfg_err);
                assert!(!s.is_empty(), "underlying toml error should not be empty");
            }
            other => panic!("Expected ConfigParse error, got: {:?}", other),
        }
    }
}
