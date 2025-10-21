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
