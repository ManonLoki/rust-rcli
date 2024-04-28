use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    process_text_decrypt, process_text_encrypt, process_text_generate_key, process_text_sign,
    process_text_verify,
};

use super::{validate_file, validate_path, CmdExecutor};

/// 文本签名子命令
#[derive(Debug, Clone, Parser)]
#[enum_dispatch::enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    /// 签名
    Sign(TextSignOpts),
    /// 验证
    Verify(TextVerifyOpts),
    /// 生成Key
    Generate(TextKeyGenerateOpts),
    /// 加密
    Encrypt(TextEncryptOpts),
    /// 解密
    Decrypt(TextDecryptOpts),
}

/// 签名参数
#[derive(Debug, Clone, Parser)]
pub struct TextSignOpts {
    /// 内容
    #[arg(short, long,value_parser=validate_file,default_value="-")]
    pub input: String,
    /// Key
    #[arg(short, long,value_parser=validate_file)]
    pub key: String,
    /// 格式化方式
    #[arg(long, default_value = "blake3")]
    pub format: TextFormat,
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> Result<()> {
        let result = process_text_sign(&self.key, &self.input, self.format)?;
        let result = URL_SAFE_NO_PAD.encode(result);
        tracing::info!("签名结果: {}", result);
        Ok(())
    }
}

/// 验证参数
#[derive(Debug, Clone, Parser)]
pub struct TextVerifyOpts {
    /// 内容
    #[arg(short, long,value_parser=validate_file,default_value="-")]
    pub input: String,
    /// Key
    #[arg(short, long,value_parser=validate_file)]
    pub key: String,
    /// 格式化方式
    #[arg(long, default_value = "blake3")]
    pub format: TextFormat,
    /// 签名
    #[arg(long)]
    pub sig: String,
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> Result<()> {
        // 解出签名
        let sig = URL_SAFE_NO_PAD.decode(&self.sig)?;
        let result = process_text_verify(&self.key, &self.input, self.format, &sig)?;
        tracing::info!("验证结果: {}", result);
        Ok(())
    }
}

/// 格式化方式
#[derive(Debug, Clone, Copy)]
pub enum TextFormat {
    /// Blake3 方式，支持签名和验证
    Blake3,
    /// Ed25519 方式，支持签名和验证
    Ed25519,
    /// Chacha20 支持加密和解密
    ChaCha20,
}

impl FromStr for TextFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(Self::Blake3),
            "ed25519" => Ok(Self::Ed25519),
            "chacha20" => Ok(Self::ChaCha20),
            _ => Err(anyhow::anyhow!("Invalid TextFormat")),
        }
    }
}

impl From<TextFormat> for &'static str {
    fn from(format: TextFormat) -> &'static str {
        match format {
            TextFormat::Blake3 => "blake3",
            TextFormat::Ed25519 => "ed25519",
            TextFormat::ChaCha20 => "chacha20",
        }
    }
}

impl Display for TextFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

/// 生成Key的选项
#[derive(Debug, Clone, Parser)]
pub struct TextKeyGenerateOpts {
    /// 输出位置
    #[arg(short,long,value_parser=validate_path)]
    pub output: PathBuf,
    /// 格式化方式
    #[arg(long, default_value = "blake3")]
    pub format: TextFormat,
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> Result<()> {
        let keys = process_text_generate_key(self.format)?;

        for (path, key) in keys {
            let path = self.output.join(path);
            tokio::fs::write(path, key).await?;
        }
        Ok(())
    }
}

/// 文本加密选项
#[derive(Debug, Clone, Parser)]
pub struct TextEncryptOpts {
    /// 输入
    #[arg(short,long,value_parser=validate_file,default_value="-")]
    pub input: String,
    /// Key 这里不从文件读取了
    #[arg(short, long)]
    pub key: String,
    /// 格式化方式
    #[arg(long, default_value = "chacha20")]
    pub format: TextFormat,
}

impl CmdExecutor for TextEncryptOpts {
    async fn execute(self) -> Result<()> {
        let result = process_text_encrypt(&self.key, &self.input, self.format)?;
        tracing::info!("加密结果: {}", result);
        Ok(())
    }
}

/// 文本解密选项
#[derive(Debug, Clone, Parser)]
pub struct TextDecryptOpts {
    /// 输入
    #[arg(short,long,value_parser=validate_file,default_value="-")]
    pub input: String,
    /// Key 这里不从文件读取了
    #[arg(short, long)]
    pub key: String,
    /// 格式化方式
    #[arg(long, default_value = "chacha20")]
    pub format: TextFormat,
}

impl CmdExecutor for TextDecryptOpts {
    async fn execute(self) -> Result<()> {
        let result = process_text_decrypt(&self.key, &self.input, self.format)?;
        tracing::info!("解密结果: {}", result);
        Ok(())
    }
}
