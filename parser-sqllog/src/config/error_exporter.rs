use serde::Deserialize;
use std::path::Path;

/// 错误导出配置
use crate::config::file::Root;

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorExporterConfig {
    /// 错误日志导出路径 (配置文件中键为 `path`)
    #[serde(rename = "path", default = "default_error_log_path")]
    pub error_log_path: String,

    /// 是否覆盖已存在的文件
    #[serde(default = "default_overwrite")]
    pub overwrite: bool,

    /// 是否以追加的方式写入文件
    #[serde(default = "default_append")]
    pub append: bool,
}

fn default_error_log_path() -> String {
    "error_logs".to_string()
}

fn default_overwrite() -> bool {
    false
}

fn default_append() -> bool {
    true
}

impl Default for ErrorExporterConfig {
    fn default() -> Self {
        Self {
            error_log_path: "error_logs".to_string(),
            overwrite: false,
            append: true,
        }
    }
}

impl ErrorExporterConfig {
    /// 创建一个默认的错误导出配置
    pub fn new() -> Self {
        Self {
            error_log_path: "error_logs".to_string(),
            overwrite: false,
            append: true,
        }
    }

    /// 从 TOML 字符串解析配置，便于单元测试和内存中解析。
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let root = Root::from_file(path);
        root.error_exporter
    }

    /// 设置错误日志导出路径
    pub fn set_error_log_path(mut self, path: &str) -> Self {
        self.error_log_path = path.to_string();
        self
    }

    /// 设置是否覆盖已存在的文件
    pub fn set_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    /// 设置是否以追加的方式写入文件
    pub fn set_append(mut self, append: bool) -> Self {
        self.append = append;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn default_error_exporter_config_has_expected_values() {
        let cfg = ErrorExporterConfig::new();
        assert_eq!(cfg.error_log_path, "error_logs".to_string());
        assert!(!cfg.overwrite);
        assert!(cfg.append);
    }

    #[test]
    fn setters_update_values() {
        let cfg = ErrorExporterConfig::new()
            .set_error_log_path("/tmp/error_logs")
            .set_overwrite(true)
            .set_append(false);

        assert_eq!(cfg.error_log_path, "/tmp/error_logs".to_string());
        assert!(cfg.overwrite);
        assert!(!cfg.append);
    }

    #[test]
    fn from_file_parses_config_correctly() {
        let toml_str = r#"
            [error_exporter]
            path = "/var/logs/errors"
            overwrite = true
            append = false
        "#;
        let mut config_file = NamedTempFile::new().unwrap();
        config_file.write_all(toml_str.as_bytes()).unwrap();
        let config_content = ErrorExporterConfig::from_file(config_file.path());

        assert_eq!(
            config_content.error_log_path,
            "/var/logs/errors".to_string()
        );
        assert!(config_content.overwrite);
    }
}
