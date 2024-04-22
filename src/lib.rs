mod cli;
pub use cli::{
    B64Format, B64SubCommand, HttpSubCommand, Opts, OutputFormat, SubCommand, TextFormat,
    TextSubCommand,
};
mod process;
pub use process::*;
mod utils;
pub use utils::*;

/// 定义Trait Executor
/// async trait在 1.75之后版本才提供
#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    /// 执行动作
    async fn execute(self) -> anyhow::Result<()>;
}
