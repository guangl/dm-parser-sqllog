use serde::Deserialize;

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
}
