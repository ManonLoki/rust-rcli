use anyhow::Result;
use clap::Parser;
use rcli::{decode, encode, process_csv, process_gen_pass, B64SubCommand, Opts, SubCommand};

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
        SubCommand::Base64(sub_command) => match sub_command {
            B64SubCommand::Encode(opts) => {
                encode(&opts.input, opts.format)?;
            }
            B64SubCommand::Decode(opts) => {
                decode(&opts.input, opts.format)?;
            }
        },
    }

    Ok(())
}
