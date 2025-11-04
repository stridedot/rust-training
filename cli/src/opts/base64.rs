use clap::{Parser, ValueEnum};

use crate::{CmdExecutor, opts};

#[derive(Clone, Debug, ValueEnum)]
pub enum Base64Format {
    Standard,
    UrlSafeNoPad,
}

#[derive(Debug, Parser)]
pub enum Base64Cmd {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode {
        #[arg(long, value_parser = opts::verify_file, default_value = "-")]
        input: String,

        #[arg(long, value_parser=parse_base64_format, default_value = "standard")]
        format: Base64Format,
    },

    #[command(name = "decode", about = "Decode a base64 string")]
    Decode {
        #[arg(long, value_parser = opts::verify_file, default_value = "-")]
        input: String,

        #[arg(long, value_parser=parse_base64_format, default_value = "standard")]
        format: Base64Format,
    },
}

impl CmdExecutor for Base64Cmd {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            // cargo run --package cli -- base64 encode --input cli/fixtures/base64_encode.txt --format urlsaf
            Base64Cmd::Encode { input, format } => {
                let encoded = crate::process::base64::process_encode(input, format).await?;
                eprintln!("{:?}", encoded);
            }
            // argo run --package cli -- base64 decode --input cli/fixtures/base64_decode.txt --format urlsafe
            Base64Cmd::Decode { input, format } => {
                let decoded = crate::process::base64::process_decode(input, format).await?;
                eprintln!("{:?}", decoded);
            }
        }
        Ok(())
    }
}

fn parse_base64_format(format: &str) -> Result<Base64Format, String> {
    match format {
        "standard" => Ok(Base64Format::Standard),
        "urlsafe" => Ok(Base64Format::UrlSafeNoPad),
        _ => Err(format!("Unknown base64 format: {}", format)),
    }
}
