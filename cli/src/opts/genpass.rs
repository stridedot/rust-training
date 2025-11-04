use clap::Parser;

use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub struct GenPassCmd {
    #[arg(long, default_value = "16")]
    pub length: usize,

    #[arg(long, default_value_t = true)]
    pub include_upper: bool,

    #[arg(long, default_value_t = true)]
    pub include_lower: bool,

    #[arg(long, default_value_t = true)]
    pub include_digit: bool,

    #[arg(long, default_value_t = true)]
    pub include_symbol: bool,
}

impl CmdExecutor for GenPassCmd {
    async fn execute(&self) -> anyhow::Result<()> {
        let password = crate::process::genpass::process_genpass(
            self.length,
            self.include_upper,
            self.include_lower,
            self.include_digit,
            self.include_symbol,
        )
        .await?;

        println!("password: {}", password);
        Ok(())
    }
}
