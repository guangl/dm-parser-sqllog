use clap::Parser;

use parser_sqllog::LogConfig;
use parser_sqllog::command::cli::Cli;
use parser_sqllog::config::error_exporter::ErrorExporterConfig;
use parser_sqllog::config::sqllog::SqllogConfig;
use parser_sqllog::error::LogError;

use tracing::{debug, info};

fn init_logging(log_cfg: &LogConfig) {
    if let Err(_) = parser_sqllog::init_logging(&log_cfg) {
        let _ = parser_sqllog::init_default_logging();
    }
}

fn main() -> Result<(), LogError> {
    let cli = Cli::parse();

    // 加载日志配置
    let log_cfg = LogConfig::from_file(&cli.config_path);
    init_logging(&log_cfg);

    // 启动日志解析工具
    info!("SQL 日志解析工具启动");

    let sqllog_cfg = SqllogConfig::from_file(&cli.config_path);
    let error_exporter_cfg = ErrorExporterConfig::from_file(&cli.config_path);

    info!("配置文件路径: {}", cli.config_path);

    debug!("日志配置: {:?}", log_cfg);
    debug!("解析配置: {:?}", sqllog_cfg);
    debug!("错误导出配置: {:?}", error_exporter_cfg);

    Ok(())
}
