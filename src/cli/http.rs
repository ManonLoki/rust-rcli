use std::path::PathBuf;

use clap::Parser;

use super::validate_path;

#[derive(Debug, Clone, Parser)]
pub enum HttpServeSubcommand {
    /// 启动HTTP服务
    Serve(HttpServeOpts),
}

/// HTTP服务参数
#[derive(Debug, Clone, Parser)]
pub struct HttpServeOpts {
    /// 路径
    #[arg(short, long, value_parser = validate_path, default_value = ".")]
    pub dir: PathBuf,
    /// 监听地址
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
