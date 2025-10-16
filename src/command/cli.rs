use clap::Parser;

#[derive(Parser)]
#[command(name = crate::NAME)]
#[command(about = crate::DESCRIPTION, long_about = None)]
#[command(version = crate::VERSION)]
pub struct Cli {
    /// 命令行详细输出
    #[arg(short, long)]
    pub verbose: bool,

    /// 解析批处理大小 (0 表示不分批处理)
    #[arg(short, long, default_value = "0")]
    pub batch_size: usize,

    /// 解析线程数量 (0 表示和文件数量一致)
    #[arg(short, long, default_value = "0")]
    pub thread_num: usize,

    /// 配置文件路径
    #[arg(short, long, default_value = "config.toml")]
    pub config_path: String,
}
