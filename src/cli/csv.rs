use std::{fmt::Display, str::FromStr};

use clap::Parser;

use crate::process_csv;

use super::{validate_file, CmdExecutor};
use anyhow::Result;

/// CSV选项
#[derive(Debug, Clone, Parser)]
pub struct CsvOpts {
    /// 输入文件 .csv
    #[arg(short, long,value_parser=validate_file)]
    pub input: String,
    /// 输出路径
    #[arg(short, long)]
    pub output: Option<String>,
    /// 输出文件格式 ，支持json和yaml
    #[arg(long,default_value="json",value_parser = parse_output_format )]
    pub format: OutputFormat,
    /// 分隔符
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    /// 是否有Header
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

/// 当前Opts的执行逻辑
impl CmdExecutor for CsvOpts {
    async fn execute(self) -> Result<()> {
        let output = self.output.unwrap_or_else(|| "output".to_string());
        let output = format!("{}.{}", output, self.format);
        process_csv(&self.input, &output, self.format)
    }
}

/// 输出格式
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    /// Json格式
    Json,
    /// Yaml格式
    Yaml,
    /// Toml格式
    Toml,
}

/// 实现将OutputFormat转换为&'static str
impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
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
            "toml" => Ok(OutputFormat::Toml),
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

// 转换输出格式
fn parse_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}
