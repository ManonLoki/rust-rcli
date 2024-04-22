use anyhow::Result;

use clap::Parser;
use rcli::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> Result<()> {
    // 转换命令行
    let opts = Opts::parse();
    // 加入日志
    tracing_subscriber::fmt::init();

    // 执行命令
    opts.command.execute().await
}
