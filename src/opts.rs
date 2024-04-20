use std::{fmt::Display, path::Path, str::FromStr};

use clap::{Parser, Subcommand};

/// 格式 rcli csv -i input.csv -o output.json -d ','

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
    /// CSV格式转换
    Csv(CsvOpts),
    /// 密码生成
    GenPass(GenPassOpts),
}

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
    #[arg(short, long,value_parser=validate_input_file)]
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

/// 验证输入文件
fn validate_input_file(input: &str) -> Result<String, &'static str> {
    if Path::new(input).exists() {
        Ok(input.into())
    } else {
        Err("File Not Exists")
    }
}

// 转换输出格式
fn parse_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}

/// 密码生成
#[derive(Debug, Clone, Parser)]
pub struct GenPassOpts {
    /// 密码长度
    #[arg(long, default_value_t = 16)]
    pub length: u8,
    /// 是否不包含数字
    #[arg(long, default_value_t = false)]
    pub no_number: bool,
    /// 是否不包含特殊字符
    #[arg(long, default_value_t = false)]
    pub no_special: bool,
    /// 是否不包含大写字母
    #[arg(long, default_value_t = false)]
    pub no_upper: bool,
    /// 是否不包含小写字母
    #[arg(long, default_value_t = false)]
    pub no_lower: bool,
}
