use std::path::PathBuf;

use clap::Parser;

use crate::{
    CmdExecutor,
    opts::{self, verify_path},
    process::text,
};

#[derive(Debug, Parser)]
pub enum TextCmd {
    // argo run --package cli -- text generate --format blake3
    // cargo run --package cli -- text generate --format ed25519
    #[command(name = "generate", about = "generate a key")]
    Generate {
        #[arg(long, value_parser = parse_format, default_value = "blake3")]
        format: TextSignFormat,

        #[arg(long, value_parser = verify_path)]
        output: PathBuf,
    },

    // argo run --package cli -- text sign --key ./cli/fixtures/blake3.key --format blake3
    // cargo run --package cli -- text sign --format ed25519 --key ./cli/fixtures/ed25519_signing.key
    #[command(name = "sign", about = "sign a message with a private key")]
    Sign {
        #[arg(long, value_parser = parse_format, default_value = "blake3")]
        format: TextSignFormat,

        #[arg(long, value_parser = opts::verify_file)]
        key: String,

        #[arg(long, value_parser = opts::verify_file, default_value = "-")]
        input: String,
    },

    // argo run --package cli -- text verify --key ./cli/fixtures/blake3.key --format blake3 --sig xxx
    // cargo run --package cli -- text verify --format ed25519 --key ./cli/fixtures/ed25519_verifying.key --sig xxx
    #[command(name = "verify", about = "verify a message with a public key")]
    Verify {
        #[arg(long, value_parser = parse_format, default_value = "blake3")]
        format: TextSignFormat,

        #[arg(long, value_parser = opts::verify_file)]
        key: String,

        #[arg(long, value_parser = opts::verify_file, default_value = "-")]
        input: String,

        #[arg(long)]
        sig: String,
    },
}

#[derive(Debug, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl CmdExecutor for TextCmd {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            TextCmd::Generate { format, output } => {
                let key = text::process_generate(format).await?;
                match format {
                    TextSignFormat::Blake3 => {
                        std::fs::write(output.join("blake3.key"), &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        std::fs::write(output.join("ed25519_signing.key"), &key[0])?;
                        std::fs::write(output.join("ed25519_verifying.key"), &key[1])?;
                    }
                }
            }
            TextCmd::Sign { input, key, format } => {
                let signed = text::process_sign(input, key, format).await?;
                println!("signature: {:?}", signed);
            }
            TextCmd::Verify {
                input,
                key,
                sig,
                format,
            } => {
                let is_valid = text::process_verify(input, key, sig, format).await?;
                println!("signature is_valid: {:?}", is_valid);
            }
        }

        Ok(())
    }
}

fn parse_format(format: &str) -> anyhow::Result<TextSignFormat> {
    match format.to_lowercase().as_str() {
        "blake3" => Ok(TextSignFormat::Blake3),
        "ed25519" => Ok(TextSignFormat::Ed25519),
        _ => anyhow::bail!("Unsupported format: {}", format),
    }
}
