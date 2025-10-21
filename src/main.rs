#[cfg(feature = "cli")]
use clap::Parser;
#[cfg(feature = "cli")]
use dm_parser_sqllog::command::cli::Cli;

use dm_parser_sqllog::error::LogError;

fn main() -> Result<(), LogError> {
    #[cfg(feature = "cli")]
    let cli = Cli::parse();

    #[cfg(feature = "logging")]
    {
        dm_parser_sqllog::init_default_logging()?;
        tracing::info!("SQL 日志解析工具启动");
        tracing::info!("详细输出: {}", cli.verbose);
        tracing::info!("批处理大小: {}", cli.batch_size);
        tracing::info!("线程数量: {}", cli.thread_num);
        tracing::info!("配置文件路径: {}", cli.config_path);
    }

    Ok(())
}
