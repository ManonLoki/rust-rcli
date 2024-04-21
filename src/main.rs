use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_gen_pass, process_http_serve,
    process_text_decrypt, process_text_encrypt, process_text_generate_key, process_text_sign,
    process_text_verify, B64SubCommand, HttpServeSubcommand, Opts, SubCommand, TextFormat,
    TextSubCommand,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> Result<()> {
    // 转换命令行
    let opts = Opts::parse();
    // 加入日志
    tracing_subscriber::fmt::init();

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
            tracing::info!("Generated Password: {}", password);
            let entropy = zxcvbn(&password, &[])?;
            tracing::info!("Password Strength: {}", entropy.score());
        }
        SubCommand::Base64(sub_command) => match sub_command {
            B64SubCommand::Encode(opts) => {
                let encoede = process_encode(&opts.input, opts.format)?;
                tracing::info!("Encoded Base64: {}", encoede);
            }
            B64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                tracing::info!("Decoded Base64: {:?}", String::from_utf8_lossy(&decoded));
            }
        },
        SubCommand::Text(sub_command) => match sub_command {
            TextSubCommand::Sign(opts) => {
                let result = process_text_sign(&opts.key, &opts.input, opts.format)?;
                let result = URL_SAFE_NO_PAD.encode(result);
                tracing::info!("签名结果: {}", result);
            }
            TextSubCommand::Verify(opts) => {
                // 解出签名
                let sig = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let result = process_text_verify(&opts.key, &opts.input, opts.format, &sig)?;
                tracing::info!("验证结果: {}", result);
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
                    TextFormat::ChaCha20 => {
                        let path = &opts.output.join("chacha20.txt");
                        std::fs::write(path, &keys[0])?;
                    }
                }
            }

            TextSubCommand::Encrypt(opts) => {
                let result = process_text_encrypt(&opts.key, &opts.input, opts.format)?;
                let result = URL_SAFE_NO_PAD.encode(result);
                tracing::info!("加密结果: {}", result);
            }
            TextSubCommand::Decrypt(opts) => {
                let result = process_text_decrypt(&opts.key, &opts.input, opts.format)?;
                let result = String::from_utf8_lossy(&result);
                tracing::info!("解密结果: {}", result);
            }
        },
        SubCommand::Http(opts) => match opts {
            HttpServeSubcommand::Serve(opts) => {
                process_http_serve(opts.port, opts.dir).await?;
            }
        },
    }

    Ok(())
}
