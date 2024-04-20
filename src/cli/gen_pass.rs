use clap::Parser;

/// 密码生成
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
