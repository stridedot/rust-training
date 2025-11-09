use std::path::PathBuf;

use clap::Parser;

use crate::{CmdExecutor, opts::verify_path};

// cargo run --package cli -- http serve --dir assets
#[derive(Debug, Parser)]
pub enum HttpCmd {
    #[command(name = "serve", about = "serve a directory over HTTP")]
    Serve {
        #[arg(long, value_parser = verify_path, default_value = ".")]
        dir: PathBuf,

        #[arg(long, default_value = "8080")]
        port: u16,
    },
}

impl CmdExecutor for HttpCmd {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            HttpCmd::Serve { dir, port } => {
                crate::process::http::process_http_serve(dir, *port).await?;
            }
        }
        Ok(())
    }
}
