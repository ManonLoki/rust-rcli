use anyhow::Result;
use clap::Parser;
use std::{fmt::Display, str::FromStr};

use crate::{process_decode, process_encode};

use super::{validate_file, CmdExecutor};

/// Base64子命令
#[derive(Debug, Clone, Parser)]
pub enum B64SubCommand {
    /// Base64编码
    Encode(B64EncodeOpts),
    /// Base64解码
    Decode(B64DecodeOpts),
}

/// 实现B64SubCommand的CmdExecutor
impl CmdExecutor for B64SubCommand {
    async fn execute(self) -> Result<()> {
        match self {
            B64SubCommand::Encode(opts) => opts.execute().await,
            B64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}

/// Encode参数
#[derive(Debug, Clone, Parser)]
pub struct B64EncodeOpts {
    /// 输入文件路径，默认为标准输入
    #[arg(short, long,value_parser=validate_file,default_value="-")]
    pub input: String,
    /// 输出文件格式化方式 支持standard和urlsafe
    #[arg(long, default_value = "standard")]
    pub format: B64Format,
}
/// 实现B64Encode的CmdExecutor
impl CmdExecutor for B64EncodeOpts {
    async fn execute(self) -> Result<()> {
        let encoede = process_encode(&self.input, self.format)?;
        tracing::info!("Encoded Base64: {}", encoede);
        Ok(())
    }
}
#[derive(Debug, Clone, Parser)]
pub struct B64DecodeOpts {
    /// 输入文件
    #[arg(short, long,value_parser=validate_file,default_value="-")]
    pub input: String,
    /// 格式化方式
    #[arg(long, default_value = "standard")]
    pub format: B64Format,
}

/// 实现B64Decode的CmdExecutor
impl CmdExecutor for B64DecodeOpts {
    async fn execute(self) -> Result<()> {
        let decoded = process_decode(&self.input, self.format)?;
        tracing::info!("Decoded Base64: {:?}", String::from_utf8_lossy(&decoded));
        Ok(())
    }
}

/// 格式化方式
#[derive(Debug, Clone, Copy)]
pub enum B64Format {
    /// 标准形式
    Standard,
    /// Urf安全形式
    UrlSafe,
}

/// 实现FromStr
impl FromStr for B64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "standard" => Ok(B64Format::Standard),
            "urlsafe" => Ok(B64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid Format")),
        }
    }
}
/// 从b64Format转换为&'static str
impl From<B64Format> for &'static str {
    fn from(value: B64Format) -> Self {
        match value {
            B64Format::Standard => "standard",
            B64Format::UrlSafe => "urlsafe",
        }
    }
}

/// 实现Display Trait 方便在输出时使用
impl Display for B64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
