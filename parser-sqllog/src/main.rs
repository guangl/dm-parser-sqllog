use clap::Parser;
use parser_sqllog::command::cli::Cli;

use parser_sqllog::error::LogError;
use tracing::info;

fn main() -> Result<(), LogError> {
    let cli = Cli::parse();

    parser_sqllog::init_default_logging()?;
    info!("SQL 日志解析工具启动");
    info!("详细输出: {}", cli.verbose);
    info!("配置文件路径: {}", cli.config_path);

    Ok(())
}
