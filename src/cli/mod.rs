use clap::{Parser, Subcommand};

use self::{csv::CsvOpts, gen_pass::GenPassOpts};

mod csv;
mod gen_pass;

pub use self::csv::OutputFormat;

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
