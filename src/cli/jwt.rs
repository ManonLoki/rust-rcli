use crate::{process_jwt_sign, process_jwt_verify, CmdExecutor};
use anyhow::Result;
use clap::Parser;

/// Jwt子命令 用户签名和验证Jwt
#[derive(Debug, Clone, Parser)]
#[enum_dispatch::enum_dispatch(CmdExecutor)]
pub enum JwtSubCommand {
    /// 签名
    Sign(JwtSignOpts),
    /// 验证
    Verify(JwtVerifyOpts),
}

/// Jwt签名选项
#[derive(Debug, Clone, Parser)]
pub struct JwtSignOpts {
    /// 签名密钥 非必填
    #[arg(short, long, default_value = "")]
    key: String,
    #[arg(short, long)]
    /// 主题
    sub: String,
    #[arg(short, long)]
    /// 目标收件人
    aud: String,
    /// 过期时间 支持 s m h d 等单位
    #[arg(short, long, value_parser= parse_exp,default_value="7d")]
    exp: usize,
}
impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> Result<()> {
        let token = process_jwt_sign(&self.key, &self.sub, &self.aud, self.exp)?;
        println!("Token: {}", token);
        Ok(())
    }
}

/// Jwt验证选项
#[derive(Debug, Clone, Parser)]
pub struct JwtVerifyOpts {
    /// 签名密钥
    #[arg(short, long, default_value = "")]
    key: String,
    /// Token 必填
    #[arg(short, long)]
    token: String,
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> Result<()> {
        let verified: bool = process_jwt_verify(&self.key, &self.token)?;
        println!("Token valid: {}", verified);
        Ok(())
    }
}

/// 转换过期时间
fn parse_exp(exp: &str) -> Result<usize> {
    let mut num = 0;
    let mut unit = 's';
    // 遍历字符，解析数字和单位
    for c in exp.chars() {
        if c.is_ascii_digit() {
            num = num * 10 + c.to_digit(10).unwrap() as usize;
        } else {
            unit = c;
        }
    }
    let exp = match unit {
        's' => num,
        'm' => num * 60,
        'h' => num * 60 * 60,
        'd' => num * 60 * 60 * 24,
        _ => return Err(anyhow::anyhow!("invalid unit: {}", unit)),
    };
    Ok(exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试日期转换
    #[test]
    fn test_parse_exp() {
        assert_eq!(parse_exp("1s").unwrap(), 1);
        assert_eq!(parse_exp("1m").unwrap(), 60);
        assert_eq!(parse_exp("1h").unwrap(), 60 * 60);
        assert_eq!(parse_exp("1d").unwrap(), 60 * 60 * 24);
    }
}
