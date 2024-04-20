use std::path::Path;

use clap::{Parser, Subcommand};
mod b64;
mod csv;
mod gen_pass;
use self::{csv::CsvOpts, gen_pass::GenPassOpts};
pub use {
    self::csv::OutputFormat,
    b64::{B64Format, B64SubCommand},
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
    /// CSV格式转换
    Csv(CsvOpts),
    /// 密码生成
    GenPass(GenPassOpts),
    /// Base64编码解码
    #[clap(subcommand)]
    Base64(B64SubCommand),
}

/// 验证输入文件
pub fn validate_input(input: &str) -> Result<String, &'static str> {
    if Path::new(input).exists() || input == "-" {
        Ok(input.into())
    } else {
        Err("File Not Exists")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validate_input() {
        assert_eq!(validate_input("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(validate_input("-"), Ok("-".to_string()));
        assert_eq!(validate_input("not_exists"), Err("File Not Exists"));
    }
}
