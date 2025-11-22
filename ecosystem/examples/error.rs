use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("I/O error {0}")]
    IoError(#[from] std::io::Error),

    #[error("parse int error {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("unknown error")]
    Unknown,
}

fn main() -> anyhow::Result<()> {
    let result = parse_int("ac");
    match result {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }

    Ok(())
}

fn parse_int(s: &str) -> Result<i32, MyError> {
    let result = s.parse::<i32>()?;
    Ok(result)
}
