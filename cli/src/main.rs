use clap::Parser;
use cli::{CmdExecutor, opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = opts::Args::parse();
    opts.cmd.execute().await
}
