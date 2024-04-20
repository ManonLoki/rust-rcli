use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, process_gen_pass, Opts, SubCommand};

fn main() -> Result<()> {
    // 转换命令行
    let opts = Opts::parse();

    // 匹配子命令
    match opts.command {
        // 匹配CSV
        SubCommand::Csv(opts) => {
            let output = opts.output.unwrap_or_else(|| "output".to_string());
            let output = format!("{}.{}", output, opts.format);
            process_csv(&opts.input, &output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_gen_pass(
                opts.length,
                opts.no_number,
                opts.no_special,
                opts.no_upper,
                opts.no_lower,
            )?;
        }
    }

    Ok(())
}
