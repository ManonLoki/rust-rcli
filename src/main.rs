use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_gen_pass, process_text_generate_key,
    process_text_sign, process_text_verify, B64SubCommand, Opts, SubCommand, TextFormat,
    TextSubCommand,
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
                let result = process_text_sign(&opts.key, &opts.input, opts.format)?;
                let result = URL_SAFE_NO_PAD.encode(result);
                println!("签名结果: {}", result);
            }
            TextSubCommand::Verify(opts) => {
                // 解出签名
                let sig = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let result = process_text_verify(&opts.key, &opts.input, opts.format, &sig)?;
                println!("验证结果: {}", result);
            }
            TextSubCommand::Generate(opts) => {
                let keys = process_text_generate_key(opts.format)?;

                match opts.format {
                    TextFormat::Blake3 => {
                        let path = opts.output.join("blake3.txt");
                        std::fs::write(path, &keys[0])?;
                    }
                    TextFormat::Ed25519 => {
                        let path = &opts.output;
                        let sk_path = path.join("ed25519.sk");
                        let pk_path = path.join("ed25519.pk");

                        std::fs::write(sk_path, &keys[0])?;
                        std::fs::write(pk_path, &keys[1])?;
                    }
                }
            }
        },
    }

    Ok(())
}
