use anyhow::Result;
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_gen_pass, process_sign, process_verify,
    B64SubCommand, Opts, SubCommand, TextSubCommand,
};
use zxcvbn::zxcvbn;

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
            let password = process_gen_pass(
                opts.length,
                opts.no_number,
                opts.no_special,
                opts.no_upper,
                opts.no_lower,
            )?;
            println!("Generated Password: {}", password);
            let entropy = zxcvbn(&password, &[])?;
            println!("Password Strength: {}", entropy.score());
        }
        SubCommand::Base64(sub_command) => match sub_command {
            B64SubCommand::Encode(opts) => {
                let encoede = process_encode(&opts.input, opts.format)?;
                println!("Encoded Base64: {}", encoede);
            }
            B64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                println!("Decoded Base64: {:?}", String::from_utf8_lossy(&decoded));
            }
        },
        SubCommand::Text(sub_command) => match sub_command {
            TextSubCommand::Sign(opts) => {
                process_sign(&opts.key, &opts.input, opts.format)?;
            }
            TextSubCommand::Verify(opts) => {
                process_verify(&opts.key, &opts.input, opts.format, &opts.sig)?;
            }
        },
    }

    Ok(())
}
