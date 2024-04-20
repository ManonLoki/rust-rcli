use std::{fmt::Display, str::FromStr};

use clap::Parser;

use super::validate_input;

/// Base64子命令
#[derive(Debug, Clone, Parser)]
pub enum B64SubCommand {
    /// 编码
    Encode(B64EncodeOpts),
    /// 解码
    Decode(B64DecodeOpts),
}

/// Encode参数
#[derive(Debug, Clone, Parser)]
pub struct B64EncodeOpts {
    /// 输入文件
    #[arg(short, long,value_parser=validate_input,default_value="-")]
    pub input: String,
    /// 格式化方式
    #[arg(long, default_value = "standard")]
    pub format: B64Format,
}

#[derive(Debug, Clone, Parser)]
pub struct B64DecodeOpts {
    /// 输入文件
    #[arg(short, long,value_parser=validate_input,default_value="-")]
    pub input: String,
    /// 格式化方式
    #[arg(long, default_value = "standard")]
    pub format: B64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum B64Format {
    Standard,
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

impl From<B64Format> for &'static str {
    fn from(value: B64Format) -> Self {
        match value {
            B64Format::Standard => "standard",
            B64Format::UrlSafe => "urlsafe",
        }
    }
}

impl Display for B64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
