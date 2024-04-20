use std::{fmt::Display, str::FromStr};

use clap::Parser;

use super::validate_file;

/// 输出格式
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

/// 实现将OutputFormat转换为&'static str
impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
/// 实现将&str 转换为OutputFormat
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid Format")),
        }
    }
}
/// 实现Display Trait 方便在输出时使用
impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 这里将自身解引用之后的值转换为&'static str
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

/// CSV选项
#[derive(Debug, Clone, Parser)]
pub struct CsvOpts {
    /// 输入文件
    #[arg(short, long,value_parser=validate_file)]
    pub input: String,
    /// 输出文件
    #[arg(short, long)]
    pub output: Option<String>,
    /// 输出格式
    #[arg(long,default_value="json",value_parser = parse_output_format )]
    pub format: OutputFormat,

    /// 分隔符
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    /// 是否有Header
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// 转换输出格式
fn parse_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}
