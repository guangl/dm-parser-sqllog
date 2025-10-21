use lazy_static::lazy_static;
use std::sync::Mutex;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    EnvFilter, Registry,
    fmt::{self, time::SystemTime},
    prelude::*,
};

use crate::{LogConfig, error::LogError, error::LogResult};

lazy_static! {
    // 保存 WorkerGuard 防止其被 drop。使用 Mutex 以便在多线程中安全写入一次。
    static ref LOG_GUARD: Mutex<Option<WorkerGuard>> = Mutex::new(None);
}

/// 日志初始化
/// 只需初始化一次，返回 LogResult<()>。
/// 函数内部会持有 WorkerGuard 防止文件 appender 被提前关闭。
pub fn init_logging(config: &LogConfig) -> LogResult<()> {
    // 如果已经初始化则直接返回 Ok(())
    if LOG_GUARD
        .lock()
        .map_err(|e| LogError::Init(format!("mutex poisoned: {}", e)))?
        .is_some()
    {
        return Ok(());
    }
    // 创建环境过滤器，默认使用配置的级别
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(config.level.clone()));

    // 控制台输出层
    let console_layer = fmt::layer()
        .with_timer(SystemTime)
        .with_target(true)
        // 显示文件和行号，可以帮助定位到函数（如需精确函数名，请使用 #[tracing::instrument]）
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(true);

    // 文件输出层 - 每日轮换，输出到指定路径，文件名前缀为 sqllog
    let file_appender = tracing_appender::rolling::daily(&config.path, "sqllog");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_timer(SystemTime)
        .with_target(true)
        // 在文件日志中也包含文件和行号
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(false); // 文件中不使用颜色

    // 将层添加到订阅者并设置为全局默认
    let subscriber = Registry::default()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| LogError::Init(format!("failed to set global subscriber: {}", e)))?;

    // 保持 guard
    *LOG_GUARD
        .lock()
        .map_err(|e| LogError::Init(format!("mutex poisoned: {}", e)))? = Some(guard);

    Ok(())
}

/// 使用默认参数初始化日志
pub fn init_default_logging() -> LogResult<()> {
    let default_config = LogConfig::new();
    init_logging(&default_config)
}
