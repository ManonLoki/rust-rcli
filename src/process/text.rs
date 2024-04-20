use crate::{get_reader, TextFormat};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
/// 使用多种方式对文本进行签名

/// 定义签名Trait
trait TextSign {
    /// 签名， 尽量用抽象行为替代具体类型
    fn sign(&self, reader: &mut dyn std::io::Read) -> Result<Vec<u8>>;
}
/// 定义Verify Trait
trait TextVerify {
    /// 验证签名
    fn verify(&self, reader: impl std::io::Read, sig: &[u8]) -> Result<bool>;
}

/// Blake3签名
#[derive(Debug)]
struct Blake3 {
    key: [u8; 32],
}
impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn std::io::Read) -> Result<Vec<u8>> {
        // 读取Buf
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // 进行签名
        // TODO: 这里也可以以流 Update的方式实现
        let key = self.key.as_slice();
        let key = key.try_into()?;
        let hash = blake3::keyed_hash(key, &buf);
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl std::io::Read, sig: &[u8]) -> Result<bool> {
        // 读取Buf
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();

        let hash = blake3::keyed_hash(self.key.as_slice().try_into()?, &buf);
        let hash = hash.as_bytes();

        // 比较签名
        Ok(hash == sig)
    }
}

/// Ed25519签名
#[derive(Debug)]
struct Ed25519Singer {
    key: SigningKey,
}

impl TextSign for Ed25519Singer {
    fn sign(&self, reader: &mut dyn std::io::Read) -> Result<Vec<u8>> {
        // 读取Buf
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // 进行签名
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

/// Ed25519验证
#[derive(Debug)]
struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl std::io::Read, sig: &[u8]) -> Result<bool> {
        // 读取Buf
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();

        // 验证签名
        let sig = ed25519_dalek::Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

/// 处理签名
pub fn process_sign(key: &str, input: &str, format: TextFormat) -> Result<()> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextFormat::Blake3 => {
            let key = std::fs::read(key)?.as_slice().try_into()?;
            Blake3 { key }.sign(&mut reader)?
        }
        TextFormat::Ed25519 => {
            let key = std::fs::read(key)?.as_slice().try_into()?;
            let key = SigningKey::from_bytes(&key);

            Ed25519Singer { key }.sign(&mut reader)?
        }
    };

    println!("{}", URL_SAFE_NO_PAD.encode(signed));
    Ok(())
}

pub fn process_verify(key: &str, input: &str, format: TextFormat, sig: &str) -> Result<()> {
    let mut reader = get_reader(input)?;

    let verified = match format {
        TextFormat::Blake3 => {
            let key = std::fs::read(key)?.as_slice().try_into()?;
            Blake3 { key }.verify(&mut reader, sig.as_bytes())?
        }
        TextFormat::Ed25519 => {
            let key = std::fs::read(key)?.as_slice().try_into()?;
            let key = VerifyingKey::from_bytes(&key)?;

            Ed25519Verifier { key }.verify(&mut reader, sig.as_bytes())?
        }
    };

    println!("Verified:{}", verified);

    Ok(())
}
