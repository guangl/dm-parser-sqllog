pub mod command;
pub mod config;
pub mod error;
pub mod logging;

// 重新导出主要的公共接口
pub use command::cli::Cli;
pub use config::logging::LogConfig;
pub use error::ConfigParseResult;
pub use logging::{init_default_logging, init_logging};

/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
