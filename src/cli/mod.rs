use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
mod b64;
mod csv;
mod gen_pass;
mod http;
mod text;
use anyhow::Result;

use crate::CmdExecutor;

use self::{csv::CsvOpts, gen_pass::GenPassOpts};
pub use {
    b64::{B64Format, B64SubCommand},
    csv::OutputFormat,
    http::HttpSubCommand,
    text::{TextFormat, TextSubCommand},
};

/// 应用程序命令行
#[derive(Debug, Clone, Parser)]
pub struct Opts {
    /// 子命令
    #[command(subcommand)]
    pub command: SubCommand,
}

/// 子命令枚举 对应SubCommand
#[derive(Debug, Clone, Subcommand)]
pub enum SubCommand {
    /// 将CSV转换为其他格式，如Json,Yaml
    Csv(CsvOpts),
    /// 生成随机密码
    GenPass(GenPassOpts),
    /// Base64编码解码
    #[clap(subcommand)]
    Base64(B64SubCommand),
    /// 文本签名及加解密
    #[clap(subcommand)]
    Text(TextSubCommand),
    /// Http服务
    #[clap(subcommand)]
    Http(HttpSubCommand),
}

/// 为SubCommand实现CmdExecutor
impl CmdExecutor for SubCommand {
    async fn execute(self) -> Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Base64(sub) => sub.execute().await,
            SubCommand::Text(sub) => sub.execute().await,
            SubCommand::Http(sub) => sub.execute().await,
        }
    }
}

/// 验证输入文件
pub fn validate_file(input: &str) -> Result<String> {
    if Path::new(input).exists() || input == "-" {
        Ok(input.into())
    } else {
        Err(anyhow::anyhow!("File Not Exists"))
    }
}

/// 验证路径
pub fn validate_path(input: &str) -> Result<PathBuf> {
    let p = Path::new(input);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        Err(anyhow::anyhow!("Path Not Exists or Not Directory"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validate_input() -> Result<()> {
        assert!(validate_file("Cargo.toml")? == *"Cargo.toml");
        assert!(validate_file("-")? == *"-");
        assert!(validate_file("not_exists").is_err());
        Ok(())
    }
}
