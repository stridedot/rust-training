pub mod csv;

use clap::Parser;

use crate::CmdExecutor;

#[derive(Debug, Parser)]
#[command(author, about, version, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "convert CSV file to JSON or...")]
    Csv(csv::CsvCmd),
}

impl CmdExecutor for SubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(cmd) => cmd.execute().await,
        }
    }
}

pub fn verify_file(filename: &str) -> Result<String, String> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("File not found: {}", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Err(format!("File not found: {}", "-")));
        assert_eq!(
            verify_file("test.csv"),
            Err(format!("File not found: {}", "test.csv"))
        );
    }
}
