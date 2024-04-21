use crate::{cli::B64Format, get_reader};
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

/// 编码
pub fn process_encode(input: &str, format: B64Format) -> Result<String> {
    // 获取Reader
    let mut reader = get_reader(input)?;
    // 创建buf
    let mut buf = vec![];
    // 读取数据到Buf
    reader.read_to_end(&mut buf)?;

    // 处理Buf
    let encoded = match format {
        B64Format::Standard => STANDARD.encode(buf),
        B64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    Ok(encoded)
}

/// 解码
pub fn process_decode(input: &str, format: B64Format) -> Result<Vec<u8>> {
    // 获取Reader
    let mut reader = get_reader(input)?;
    // 创建buf
    let mut buf = String::new();
    // 读取数据到Buf
    reader.read_to_string(&mut buf)?;
    // 去掉回车
    let buf = buf.trim();

    // 处理Buf
    let decoded = match format {
        B64Format::Standard => STANDARD.decode(buf),
        B64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf),
    }?;
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert!(process_encode("fixtures/b64_origin.txt", B64Format::Standard).is_ok());
        assert!(process_encode("fixtures/b64_origin.txt", B64Format::UrlSafe).is_ok());
    }

    #[test]
    fn test_decode() {
        assert!(process_decode("fixtures/b64_standard.txt", B64Format::Standard).is_ok());
        assert!(process_decode("fixtures/b64_urlsafe.txt", B64Format::UrlSafe).is_ok());
    }
}
