use anyhow::Result;
use clap::Parser;
use rcli::{
    cli::{self, SubCommand},
    csv_process,
};

fn main() -> Result<()> {
    // 转换命令行
    let cli = cli::Cli::parse();

    // 匹配子命令
    match cli.command {
        // 匹配CSV
        SubCommand::Csv(opts) => {
            csv_process::parse(&opts)?;
        }
    }

    Ok(())
}
