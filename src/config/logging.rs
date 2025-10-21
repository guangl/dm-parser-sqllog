use std::path::Path;

use serde::Deserialize;

use crate::{ConfigParseResult, config::file::Root};

#[derive(Default, Debug, Deserialize, Clone)]
pub struct LogConfig {
    /// 日志级别文本: "error", "warn", "info", "debug", "trace"
    pub level: String,
    /// 日志输出文件路径，默认输出到 logs 目录
    pub path: String,
}

impl LogConfig {
    pub fn new() -> Self {
        Self {
            level: "info".to_string(),
            path: "logs".to_string(),
        }
    }

    /// 从 TOML 字符串解析配置，便于单元测试和内存中解析。
    pub fn from_file<P: AsRef<Path>>(path: P) -> ConfigParseResult<Self> {
        let root = Root::from_file(path)?;
        Ok(root.logging.unwrap_or_default())
    }

    pub fn set_level(mut self, level: &str) -> Self {
        self.level = level.to_string();
        self
    }

    pub fn set_path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn default_log_config_has_expected_values() {
        let cfg = LogConfig::new();

        assert_eq!(cfg.level, "info".to_string());
        assert_eq!(cfg.path, "logs".to_string());
    }

    #[test]
    fn setters_update_values() {
        let cfg = LogConfig::new().set_level("debug").set_path("/tmp/mylogs");

        assert_eq!(cfg.level, "debug".to_string());
        assert_eq!(cfg.path, "/tmp/mylogs".to_string());
    }

    #[test]
    fn from_file_parses_config_correctly() {
        let toml_str = r#"
            [logging]
            level = "error"
            path = "/var/logs/errors"
        "#;
        let mut config_file = NamedTempFile::new().unwrap();
        config_file.write_all(toml_str.as_bytes()).unwrap();
        let config_content = LogConfig::from_file(config_file.path()).unwrap();

        assert_eq!(config_content.level, "error".to_string());
        assert_eq!(config_content.path, "/var/logs/errors".to_string());
    }
}
