use super::{validate_path, CmdExecutor};
use crate::process_http_serve;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Http服务子命令
#[derive(Debug, Clone, Parser)]
#[enum_dispatch::enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
    /// 启动静态文件服务
    Serve(HttpServeOpts),
}

/// HTTP服务参数
#[derive(Debug, Clone, Parser)]
pub struct HttpServeOpts {
    /// 监听目录 默认为当前目录
    #[arg(short, long, value_parser = validate_path, default_value = ".")]
    pub dir: PathBuf,
    /// 监听端口
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

/// 为HttpServeOpts实现CmdExecutor
impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> Result<()> {
        process_http_serve(self.port, self.dir).await
    }
}
