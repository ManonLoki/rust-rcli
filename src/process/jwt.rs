use anyhow::Result;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// 创建Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    exp: usize,
    sub: String,
    iat: usize,
}

/// 签名Jwt
pub fn process_jwt_sign(key: &str, sub: &str, aud: &str, exp: usize) -> Result<String> {
    // 创建header 这里暂时用HS256算法
    let header = Header::new(Algorithm::HS256);
    // 创建EncodingKey
    let encoding_key = EncodingKey::from_secret(key.as_bytes());

    // Token的创建时间
    let iat = chrono::Utc::now().timestamp() as usize;
    // Token的过期期时间 创建时间+过期时间
    let exp = iat + exp;
    tracing::info!("exp: {}", exp);
    // 创建claims
    let claims = Claims {
        aud: aud.to_owned(),
        exp,
        iat,
        sub: sub.to_owned(),
    };

    tracing::info!("Claims: {:?}", claims);

    // 生成token
    let token = jsonwebtoken::encode(&header, &claims, &encoding_key)?;

    Ok(token)
}

/// 验证Jwt
pub fn process_jwt_verify(key: &str, token: &str) -> Result<bool> {
    // 创建DecodingKey
    let decoding_key = DecodingKey::from_secret(key.as_bytes());

    // 创建验证器
    let mut validation = Validation::new(Algorithm::HS256);
    // 要设置验证过期时间 但是不验证目标
    validation.validate_aud = false;
    validation.validate_exp = true;

    let result = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation);

    // 这里只要结果是否正常
    Ok(result.is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_sign_verify() -> Result<()> {
        let key = "testKey";
        let sub = "testSub";
        let aud = "testAud";
        // 60 秒过期
        let exp_time = 60;

        let token = process_jwt_sign(key, sub, aud, exp_time)?;

        assert!(process_jwt_verify(key, &token)?);

        Ok(())
    }
}
