use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
mod b64;
mod csv;
mod gen_pass;
mod text;
use self::{csv::CsvOpts, gen_pass::GenPassOpts};
pub use {
    self::csv::OutputFormat,
    b64::{B64Format, B64SubCommand},
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
    /// CSV格式转换
    Csv(CsvOpts),
    /// 密码生成
    GenPass(GenPassOpts),
    /// Base64编码解码
    #[clap(subcommand)]
    Base64(B64SubCommand),
    /// 文本签名
    #[clap(subcommand)]
    Text(TextSubCommand),
}

/// 验证输入文件
pub fn validate_file(input: &str) -> Result<String, &'static str> {
    if Path::new(input).exists() || input == "-" {
        Ok(input.into())
    } else {
        Err("File Not Exists")
    }
}

/// 验证路径
pub fn validate_path(input: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(input);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        Err("Path Not Exists or Not Directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validate_input() {
        assert_eq!(validate_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(validate_file("-"), Ok("-".to_string()));
        assert_eq!(validate_file("not_exists"), Err("File Not Exists"));
    }
}
