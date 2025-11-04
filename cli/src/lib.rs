pub mod opts;
pub mod process;
pub mod utils;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(&self) -> anyhow::Result<()>;
}
