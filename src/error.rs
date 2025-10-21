/// 定义日志相关的错误类型和结果类型
pub type ConfigParseResult<T> = std::result::Result<T, ConfigParseError>;
pub type LogResult<T> = std::result::Result<T, LogError>;

#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("初始化日志失败: {0}")]
    Init(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigParseError {
    #[error("IO 错误: {0}")]
    Io(std::io::Error),

    #[error("TOML 语法错误: {0}")]
    Toml(String),

    #[error("TOML 解析错误: {0}")]
    Parser(toml::de::Error),

    #[error("缺少字段: {0}")]
    MissingField(String),

    #[error("字段类型错误: field={field}, expected={expected}, found={found}")]
    FieldType {
        field: String,
        expected: String,
        found: String,
    },

    #[error("未知字段: {0}")]
    UnknownField(String),
}
