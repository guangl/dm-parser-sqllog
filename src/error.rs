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
    #[error("日志初始化错误: {0}")]
    Init(String),
}
