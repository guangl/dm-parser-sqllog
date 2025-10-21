use clap::Parser;

#[derive(Parser)]
#[command(name = crate::NAME)]
#[command(about = crate::DESCRIPTION, long_about = None)]
#[command(version = crate::VERSION)]
pub struct Cli {
    /// 命令行详细输出
    #[arg(short, long)]
    pub verbose: bool,

    /// 配置文件路径
    #[arg(short, long, default_value = "config.toml")]
    pub config_path: String,
}
