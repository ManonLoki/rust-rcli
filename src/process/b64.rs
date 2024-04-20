use crate::cli::B64Format;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

/// 编码
pub fn encode(input: &str, format: B64Format) -> Result<()> {
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

    println!("{}", encoded);

    Ok(())
}

/// 解码
pub fn decode(input: &str, format: B64Format) -> Result<()> {
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

    println!("{}", String::from_utf8(decoded)?);

    Ok(())
}

/// 获取Reader
fn get_reader(input: &str) -> Result<Box<dyn std::io::Read>> {
    match input {
        "-" => Ok(Box::new(std::io::stdin())),
        _ => Ok(Box::new(std::fs::File::open(input)?)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert!(encode("fixture/b64_origin.txt", B64Format::Standard).is_ok());
        assert!(encode("fixture/b64_origin.txt", B64Format::UrlSafe).is_ok());
    }

    #[test]
    fn test_decode() {
        assert!(decode("fixture/b64_standard.txt", B64Format::Standard).is_ok());
        assert!(decode("fixture/b64_urlsafe.txt", B64Format::UrlSafe).is_ok());
    }
}
