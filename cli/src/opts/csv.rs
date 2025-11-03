use clap::{Parser, ValueEnum};

use crate::{CmdExecutor, opts};

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Json,
    Toml,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvCmd {
    #[arg(long, value_parser = opts::verify_file)]
    pub input: String,

    #[arg(long, default_value = "output.json")]
    pub output: String,

    #[arg(long, default_value = "json")]
    pub format: OutputFormat,

    #[arg(long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

impl CmdExecutor for CsvCmd {
    async fn execute(&self) -> anyhow::Result<()> {
        crate::process::csv::process_csv(&self.input, &self.output, &self.format).await
    }
}
