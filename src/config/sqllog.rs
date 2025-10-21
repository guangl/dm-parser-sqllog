use std::path::Path;

use serde::Deserialize;

use crate::{ConfigParseResult, config::file::Root};

#[derive(Default, Debug, Deserialize, Clone)]
pub struct SqllogConfig {
    /// 批处理大小 (配置文件中键为 `batch-size`)
    #[serde(default)]
    pub batch_size: usize,

    /// 多线程处理
    #[serde(default)]
    pub thread_num: usize,

    /// 日志输出文件路径，默认输出到 sqllog 目录
    #[serde(default, rename = "path")]
    pub sqllog_path: String,
}

impl SqllogConfig {
    pub fn new() -> Self {
        Self {
            thread_num: 0,
            batch_size: 0,
            sqllog_path: "sqllog".to_string(),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> ConfigParseResult<Self> {
        let root = Root::from_file(path)?;
        Ok(root.sqllog.unwrap_or_default())
    }

    pub fn set_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn set_thread_num(mut self, thread_num: usize) -> Self {
        self.thread_num = thread_num;
        self
    }

    pub fn set_sqllog_path(mut self, path: &str) -> Self {
        self.sqllog_path = path.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_sqllog_config_default() {
        let config = SqllogConfig::new();
        assert_eq!(config.batch_size, 0);
        assert_eq!(config.thread_num, 0);
        assert_eq!(config.sqllog_path, "sqllog".to_string());
    }

    #[test]
    fn test_sqllog_config_setters() {
        let config = SqllogConfig::new()
            .set_batch_size(100)
            .set_thread_num(4)
            .set_sqllog_path("output/sqllog");
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.thread_num, 4);
        assert_eq!(config.sqllog_path, "output/sqllog".to_string());
    }

    #[test]
    fn test_sqllog_config_from_file() {
        let toml_str = r#"
            [sqllog]
            path = "/var/logs/errors"
            batch_size = 10
            thread_num = 10
        "#;
        let mut config_file = NamedTempFile::new().unwrap();
        config_file.write_all(toml_str.as_bytes()).unwrap();
        let config_content = SqllogConfig::from_file(config_file.path()).unwrap();

        assert_eq!(config_content.sqllog_path, "/var/logs/errors".to_string());
        assert_eq!(config_content.batch_size, 10);
        assert_eq!(config_content.thread_num, 10);
    }
}
