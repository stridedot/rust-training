use std::path::{Path, PathBuf};

use anyhow::Result;
use arrow::util::pretty;
use datafusion::prelude::{ParquetReadOptions, SessionContext};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use polars::{prelude::*, sql::SQLContext};

// #[tokio::main]
fn main() -> Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets/sample.parquet");

    // read_with_parquet(&path).await?;
    // read_with_datafusion(&path).await?;
    read_with_polars(&path)?;

    Ok(())
}

#[allow(dead_code)]
async fn read_with_parquet(path: &Path) -> Result<()> {
    let file = std::fs::File::open(path)?;
    let parquet_reader = ParquetRecordBatchReaderBuilder::try_new(file)?
        .with_batch_size(8192)
        .build()?;

    let mut batches = Vec::new();

    for batch in parquet_reader {
        batches.push(batch?);
    }

    pretty::print_batches(&batches).unwrap();

    Ok(())
}

#[allow(dead_code)]
async fn read_with_datafusion(path: &Path) -> Result<()> {
    let path_str = path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Path is not valid UTF-8"))?;

    let ctx = SessionContext::new();
    ctx.register_parquet("user_stat", path_str, ParquetReadOptions::default())
        .await?;

    let df = ctx.sql("SELECT * FROM user_stat").await?;
    let batches = df.collect().await?;
    pretty::print_batches(&batches).unwrap();

    Ok(())
}

fn read_with_polars(path: &Path) -> Result<()> {
    let path_str = path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Path is not valid UTF-8"))?;

    let lf: LazyFrame = LazyFrame::scan_parquet(PlPath::new(path_str), Default::default())?;

    let mut ctx = SQLContext::new();
    ctx.register("stats", lf);

    let df = ctx.execute("SELECT email, name FROM stats")?.collect()?;

    println!("{:?}", df);

    Ok(())
}
