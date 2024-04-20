use anyhow::Result;
use rand::prelude::*;

const NUMBERS: &[u8] = b"123456789";
const SPECIALS: &[u8] = b"!@#$%^&*_-";
const UPPERS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWERS: &[u8] = b"abcdefghijkmnpqrstuvwxyz";

pub fn process_gen_pass(
    length: u8,
    no_number: bool,
    no_special: bool,
    no_upper: bool,
    no_lower: bool,
) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = vec![];
    if !no_number {
        chars.extend_from_slice(NUMBERS);

        password.push(*NUMBERS.choose(&mut rng).expect("字符集不可为空"));
    }
    if !no_special {
        chars.extend_from_slice(SPECIALS);
        password.push(*SPECIALS.choose(&mut rng).expect("字符集不可为空"));
    }
    if !no_upper {
        chars.extend_from_slice(UPPERS);
        password.push(*UPPERS.choose(&mut rng).expect("字符集不可为空"));
    }
    if !no_lower {
        chars.extend_from_slice(LOWERS);
        password.push(*LOWERS.choose(&mut rng).expect("字符集不可为空"));
    }

    // 从第四位往后随机
    for _ in 4..length {
        let idx = rng.gen_range(0..chars.len());
        password.push(chars[idx]);
    }

    // 对密码进行乱序
    password.shuffle(&mut rng);

    // 转换为字符串
    let password = String::from_utf8(password)?;

    Ok(password)
}
