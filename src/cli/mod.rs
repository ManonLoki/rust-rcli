use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
mod b64;
mod csv;
mod gen_pass;
mod http;
mod jwt;
mod text;

use crate::CmdExecutor;

pub use {b64::*, csv::*, gen_pass::*, http::*, jwt::*, text::*};

/// 应用程序命令行
#[derive(Debug, Clone, Parser)]
#[clap(
    version,
    author = "manonloki",
    about,
    help_template = "{before-help} {name} {version} {author-with-newline} {about-with-newline}
{usage-heading} {usage}{all-args} {after-help}"
)]
pub struct Opts {
    /// 子命令
    #[command(subcommand)]
    pub command: SubCommand,
}

/// 子命令枚举 对应SubCommand
#[derive(Debug, Clone, Subcommand)]
#[enum_dispatch::enum_dispatch(CmdExecutor)]
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
    /// Jwt签名及验证
    #[clap(subcommand)]
    Jwt(JwtSubCommand),
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
