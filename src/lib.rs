#[cfg(feature = "cli")]
pub mod command;
pub mod config;
pub mod error;
#[cfg(feature = "logging")]
pub mod logging;

// 重新导出主要的公共接口
#[cfg(feature = "cli")]
pub use command::cli::Cli;
#[cfg(feature = "logging")]
pub use config::logging::LogConfig;
#[cfg(feature = "logging")]
pub use error::LogError;
#[cfg(feature = "logging")]
pub use logging::{init_default_logging, init_logging};

/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
