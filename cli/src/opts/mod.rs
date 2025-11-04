pub mod base64;
pub mod csv;
pub mod genpass;

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

    #[command(name = "genpass", about = "generate random password")]
    GenPass(genpass::GenPassCmd),

    #[command(subcommand, name = "base64", about = "base64 encode or decode")]
    Base64(base64::Base64Cmd),
}

impl CmdExecutor for SubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(cmd) => cmd.execute().await,
            SubCommand::GenPass(cmd) => cmd.execute().await,
            SubCommand::Base64(cmd) => cmd.execute().await,
        }
    }
}

pub fn verify_file(input: &str) -> Result<String, String> {
    if input == "-" || std::path::Path::new(input).exists() {
        Ok(input.to_string())
    } else {
        Err(format!("Input File not found: {}", input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(
            verify_file("test.csv"),
            Err(format!("Input File not found: {}", "test.csv"))
        );
    }
}
