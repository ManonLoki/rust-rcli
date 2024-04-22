use clap::Parser;

use crate::process_gen_pass;

use super::CmdExecutor;
use anyhow::Result;
use zxcvbn::zxcvbn;

/// 生成密码选项
#[derive(Debug, Clone, Parser)]
pub struct GenPassOpts {
    /// 密码长度
    #[arg(long, default_value_t = 16)]
    pub length: u8,
    /// 是否不包含数字
    #[arg(long, default_value_t = false)]
    pub no_number: bool,
    /// 是否不包含特殊字符
    #[arg(long, default_value_t = false)]
    pub no_special: bool,
    /// 是否不包含大写字母
    #[arg(long, default_value_t = false)]
    pub no_upper: bool,
    /// 是否不包含小写字母
    #[arg(long, default_value_t = false)]
    pub no_lower: bool,
}

/// 实现执行逻辑
impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> Result<()> {
        let password = process_gen_pass(
            self.length,
            self.no_number,
            self.no_special,
            self.no_upper,
            self.no_lower,
        )?;
        println!("Generated Password: {}", password);
        let entropy = zxcvbn(&password, &[])?;
        println!("Password Strength: {}", entropy.score());
        Ok(())
    }
}
