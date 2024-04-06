use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// 格式 rcli csv -i input.csv -o output.json --header -d ','

/// 应用程序命令行
#[derive(Debug, Clone, Parser)]
pub struct Cli {
    /// 子命令
    #[command(subcommand)]
    pub command: SubCommand,
}

/// 子命令枚举 对应SubCommand
#[derive(Debug, Clone, Subcommand)]
pub enum SubCommand {
    /// CSV格式转换
    Csv(CsvOpts),
}

/// CSV选项
#[derive(Debug, Clone, Parser)]
pub struct CsvOpts {
    /// 输入文件
    #[arg(short, long,value_parser=validate_input_file)]
    pub input: PathBuf,
    /// 输出文件
    #[arg(short, long, default_value = "output.json")]
    pub output: PathBuf,
    /// 分隔符
    #[arg(short, long, default_value = ",")]
    pub delimiter: u8,
}

/// 验证输入文件
fn validate_input_file(input: &str) -> Result<PathBuf, &'static str> {
    let path = PathBuf::from(input);

    if path.exists() {
        Ok(path)
    } else {
        Err("输入文件不存在")
    }
}
