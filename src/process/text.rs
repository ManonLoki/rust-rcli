use std::path::Path;

use crate::{get_reader, TextFormat};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};

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

/// 定义加密Trait
pub trait TextEncrypt {
    /// 加密
    fn encrypt(&self, reader: &mut dyn std::io::Read) -> Result<Vec<u8>>;
}

/// 定义解密Trait
pub trait TextDecrypt {
    /// 解密
    fn decrypt(&self, reader: &mut dyn std::io::Read) -> Result<Vec<u8>>;
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

        Self::try_new(&key[..32])
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

#[derive(Debug)]
pub struct Chacha20 {
    key: chacha20poly1305::Key,
}

impl Chacha20 {
    fn new(key: chacha20poly1305::Key) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = chacha20poly1305::Key::from_slice(key);
        Ok(Self::new(key.to_owned()))
    }
}
impl KeyLoader for Chacha20 {
    fn load_key(key: impl AsRef<Path>) -> Result<Self> {
        let data = std::fs::read(key)?;
        let key = &data[0..32];
        Self::try_new(key)
    }
}

impl KeyGenerate for Chacha20 {
    fn generate_key() -> Result<Vec<Vec<u8>>> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng).to_vec();
        Ok(vec![key])
    }
}

impl TextEncrypt for Chacha20 {
    fn encrypt(&self, reader: &mut dyn std::io::Read) -> Result<Vec<u8>> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let buf = buf.trim().as_bytes();

        let cipher = ChaCha20Poly1305::new(&self.key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let cipher_text = cipher
            .encrypt(&nonce, buf)
            .map_err(|e| anyhow::anyhow!(e))?;

        // 将nonce和cipher_text合并
        let mut merged_nonce = nonce.to_vec();
        merged_nonce.extend_from_slice(&cipher_text);

        println!("{:?}", merged_nonce);
        Ok(merged_nonce)
    }
}

impl TextDecrypt for Chacha20 {
    fn decrypt(&self, reader: &mut dyn std::io::Read) -> Result<Vec<u8>> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let buf = buf.trim();
        // 得先解密
        let buf = URL_SAFE_NO_PAD.decode(buf)?;
        println!("{:?}", buf);

        // 头12位是nonce 之后的是cipher_text
        let nonce = &buf[0..12];
        let cipher_text = &buf[12..];
        let cipher = ChaCha20Poly1305::new(&self.key);
        let nonce = chacha20poly1305::Nonce::from_slice(nonce);

        let plain_text = cipher
            .decrypt(nonce, cipher_text)
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(plain_text)
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
        _ => {
            return Err(anyhow::anyhow!("Unsupported Sign"));
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
        _ => {
            return Err(anyhow::anyhow!("Unsupported Verify"));
        }
    };

    Ok(verified)
}

/// 生成Key
pub fn process_text_generate_key(format: TextFormat) -> Result<Vec<Vec<u8>>> {
    let keys = match format {
        TextFormat::Blake3 => Blake3::generate_key()?,
        TextFormat::Ed25519 => Ed25519Singer::generate_key()?,
        TextFormat::ChaCha20 => Chacha20::generate_key()?,
    };

    Ok(keys)
}

/// 加密
pub fn process_text_encrypt(key: &str, input: &str, format: TextFormat) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let encryptor = Chacha20::load_key(key)?;

    match format {
        TextFormat::ChaCha20 => encryptor.encrypt(&mut reader),
        _ => Err(anyhow::anyhow!("Unsupported Encrypt")),
    }
}

/// 解密
pub fn process_text_decrypt(key: &str, input: &str, format: TextFormat) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let decrypter = Chacha20::load_key(key)?;

    match format {
        TextFormat::ChaCha20 => decrypter.decrypt(&mut reader),
        _ => Err(anyhow::anyhow!("Unsupported Decrypt")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let signer = Blake3::load_key("fixtures/blake3.txt").unwrap();
        let data = b"hello world";
        let sig = signer.sign(&mut &data[..]).unwrap();

        assert!(signer.verify(&mut &data[..], &sig)?);

        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let signer = Ed25519Singer::load_key("fixtures/ed25519.sk")?;
        let data = b"hello world";
        let sig = signer.sign(&mut &data[..])?;

        let verifier = Ed25519Verifier::load_key("fixtures/ed25519.pk")?;
        assert!(verifier.verify(&mut &data[..], &sig)?);

        Ok(())
    }

    #[test]
    fn test_chacha20_encrypt_decrypt() -> Result<()> {
        let key = "fixtures/chacha20.txt";
        let data = b"Cargo.toml";
        let encryptor: Chacha20 = Chacha20::load_key(key)?;
        let cipher_text = encryptor.encrypt(&mut &data[..])?;

        let cipher_text = URL_SAFE_NO_PAD.encode(cipher_text);

        let encryptor: Chacha20 = Chacha20::load_key(key)?;
        let res = encryptor.decrypt(&mut &cipher_text.as_bytes()[..])?;
        assert_eq!(res, data);
        Ok(())
    }
}
