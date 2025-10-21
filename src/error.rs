/// 定义日志相关的错误类型和结果类型
pub type LogResult<T> = std::result::Result<T, LogError>;

/// 日志相关错误枚举
/// 包括文件 I/O 错误、配置错误和初始化错误
#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("日志文件 I/O 错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("日志配置错误: {0}")]
    Config(String),
    /// 配置文件解析错误（更细粒度的分类，封装自 ConfigParseError）
    #[error("配置文件解析错误: {0}")]
    ConfigParse(#[from] ConfigParseError),
    /// 日志解析相关错误（封装自 ParseError）
    #[error("日志解析错误: {0}")]
    Parse(#[from] ParseError),
    #[error("日志初始化错误: {0}")]
    Init(String),
}

/// 日志解析错误的具体类型
/// 放在这里便于统一管理所有与日志文本/字段解析相关的错误
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("无效的日志格式: {0}")]
    InvalidFormat(String),

    #[error("缺少字段: {0}")]
    MissingField(String),

    #[error("字段解析失败: {field}: {reason}")]
    FieldParse { field: String, reason: String },
}

// 下面是配置解析错误的更细粒度分类，用于把 toml/serde 的解析错误映射为可辨识的变体
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, thiserror::Error)]
pub enum ConfigParseError {
    #[error("TOML 语法错误: {0}")]
    Toml(String),

    #[error("缺少字段: {0}")]
    MissingField(String),

    #[error("字段类型错误: field={field}, expected={expected}, found={found}")]
    FieldType { field: String, expected: String, found: String },

    #[error("未知字段: {0}")]
    UnknownField(String),

    #[error("其他解析错误: {0}")]
    Other(String),
}

impl From<toml::de::Error> for ConfigParseError {
    fn from(e: toml::de::Error) -> Self {
        let s = e.to_string();

        lazy_static! {
            static ref RE_MISSING: Regex = Regex::new(r"missing field `([^`]+)`").unwrap();
            static ref RE_UNKNOWN: Regex = Regex::new(r"unknown field `([^`]+)`").unwrap();
            static ref RE_INVALID_TYPE_WITH_KEY: Regex = Regex::new(r"invalid type: ([^,]+), expected ([^,]+) for key `([^`]+)`").unwrap();
            static ref RE_INVALID_TYPE_SIMPLE: Regex = Regex::new(r"invalid type: ([^,]+), expected ([^\.\n]+)").unwrap();
        }

        if let Some(c) = RE_MISSING.captures(&s) {
            return ConfigParseError::MissingField(c[1].to_string());
        }

        if let Some(c) = RE_UNKNOWN.captures(&s) {
            return ConfigParseError::UnknownField(c[1].to_string());
        }

        if let Some(c) = RE_INVALID_TYPE_WITH_KEY.captures(&s) {
            return ConfigParseError::FieldType {
                field: c[3].to_string(),
                expected: c[2].to_string(),
                found: c[1].to_string(),
            };
        }

        if let Some(c) = RE_INVALID_TYPE_SIMPLE.captures(&s) {
            return ConfigParseError::FieldType {
                field: "<unknown>".to_string(),
                expected: c[2].trim().to_string(),
                found: c[1].to_string(),
            };
        }

        // 回退为原始 toml 错误信息
        ConfigParseError::Toml(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_error_into_log_error_and_message() {
        let p = ParseError::MissingField("timestamp".into());
        // 可以直接从 ParseError 转换为 LogError（#[from] 自动生成转换）
        let le: LogError = p.into();
        assert!(matches!(le, LogError::Parse(_)));
        assert_eq!(le.to_string(), "日志解析错误: 缺少字段: timestamp");
    }
}
