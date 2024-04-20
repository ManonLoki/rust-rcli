use std::path::Path;

use crate::{get_reader, TextFormat};
use anyhow::Result;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use super::gen_pass;
/// 使用多种方式对文本进行签名

/// 定义签名Trait
pub trait TextSign {
    /// 签名， 尽量用抽象行为替代具体类型
    fn sign(&self, reader: &mut dyn std::io::Read) -> Result<Vec<u8>>;
}
/// 定义Verify Trait
pub trait TextVerify {
    /// 验证签名
    fn verify(&self, reader: impl std::io::Read, sig: &[u8]) -> Result<bool>;
}
/// 定义KeyLoader Trait
pub trait KeyLoader {
    /// 根据加载的Key 生成类型，这个类型是编译期已知长度的
    fn load_key(key: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}
/// 定义生成Key Trait
pub trait KeyGenerate {
    /// 生成Key
    fn generate_key() -> Result<Vec<Vec<u8>>>;
}

/// Blake3签名
#[derive(Debug)]
struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = key.try_into()?;
        Ok(Self::new(key))
    }
}
impl KeyLoader for Blake3 {
    fn load_key(key: impl AsRef<Path>) -> Result<Self> {
        let key = std::fs::read(key)?;
        Self::try_new(&key)
    }
}
impl KeyGenerate for Blake3 {
    fn generate_key() -> Result<Vec<Vec<u8>>> {
        let key = gen_pass::process_gen_pass(32, false, false, false, false)?;
        Ok(vec![key.as_bytes().to_vec()])
    }
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

impl Ed25519Singer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }
}

impl KeyLoader for Ed25519Singer {
    fn load_key(key: impl AsRef<Path>) -> Result<Self> {
        let key = std::fs::read(key)?;
        Self::try_new(&key)
    }
}
impl KeyGenerate for Ed25519Singer {
    fn generate_key() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
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

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load_key(key: impl AsRef<Path>) -> Result<Self> {
        let key = std::fs::read(key)?;
        Self::try_new(&key)
    }
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

/// 签名逻辑
pub fn process_text_sign(key: &str, input: &str, format: TextFormat) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextFormat::Blake3 => {
            let signer = Blake3::load_key(key)?;
            signer.sign(&mut reader)?
        }
        TextFormat::Ed25519 => {
            let signer = Ed25519Singer::load_key(key)?;
            signer.sign(&mut reader)?
        }
    };

    Ok(signed)
}

/// 验证逻辑
pub fn process_text_verify(key: &str, input: &str, format: TextFormat, sig: &[u8]) -> Result<bool> {
    let mut reader = get_reader(input)?;

    let verified = match format {
        TextFormat::Blake3 => {
            let verifier = Blake3::load_key(key)?;
            verifier.verify(&mut reader, sig)?
        }
        TextFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load_key(key)?;
            verifier.verify(&mut reader, sig)?
        }
    };

    Ok(verified)
}

/// 生成Key
pub fn process_text_generate_key(format: TextFormat) -> Result<Vec<Vec<u8>>> {
    let keys = match format {
        TextFormat::Blake3 => Blake3::generate_key()?,
        TextFormat::Ed25519 => Ed25519Singer::generate_key()?,
    };

    Ok(keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let signer = Blake3::load_key("fixture/blake3.txt").unwrap();
        let data = b"hello world";
        let sig = signer.sign(&mut &data[..]).unwrap();

        assert!(signer.verify(&mut &data[..], &sig)?);

        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let signer = Ed25519Singer::load_key("fixture/ed25519.sk")?;
        let data = b"hello world";
        let sig = signer.sign(&mut &data[..])?;

        let verifier = Ed25519Verifier::load_key("fixture/ed25519.pk")?;
        assert!(verifier.verify(&mut &data[..], &sig)?);

        Ok(())
    }
}
