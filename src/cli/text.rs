use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use clap::Parser;

use super::validate_file;

/// 文本签名子命令
#[derive(Debug, Clone, Parser)]

pub enum TextSubCommand {
    /// 签名
    Sign(SignOpts),
    /// 验证
    Verify(VerifyOpts),
}

/// 签名参数
#[derive(Debug, Clone, Parser)]
pub struct SignOpts {
    /// 内容
    #[arg(short, long,value_parser=validate_file,default_value="-")]
    pub input: String,
    /// Key
    #[arg(short, long,value_parser=validate_file)]
    pub key: String,
    /// 格式化方式
    #[arg(long,value_parser=parse_format,default_value="blake3")]
    pub format: TextFormat,
}
/// 验证参数
#[derive(Debug, Clone, Parser)]
pub struct VerifyOpts {
    /// 内容
    #[arg(short, long,value_parser=validate_file,default_value="-")]
    pub input: String,
    /// Key
    #[arg(short, long,value_parser=validate_file)]
    pub key: String,
    /// 格式化方式
    #[arg(long,value_parser = parse_format,default_value="blake3")]
    pub format: TextFormat,
    /// 签名
    #[arg(long)]
    pub sig: String,
}

/// 格式化方式
#[derive(Debug, Clone, Copy)]
pub enum TextFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(Self::Blake3),
            "ed25519" => Ok(Self::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid TextFormat")),
        }
    }
}

impl From<TextFormat> for &'static str {
    fn from(format: TextFormat) -> &'static str {
        match format {
            TextFormat::Blake3 => "blake3",
            TextFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

fn parse_format(s: &str) -> Result<TextFormat, anyhow::Error> {
    s.parse()
}
