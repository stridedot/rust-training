use crate::{opts::base64::Base64Format, utils};
use base64::prelude::*;

pub async fn process_encode(input: &str, format: &Base64Format) -> anyhow::Result<String> {
    let mut reader = utils::get_reader(input).await?;
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(input),
        Base64Format::UrlSafeNoPad => BASE64_URL_SAFE_NO_PAD.encode(input),
    };

    Ok(encoded)
}

pub async fn process_decode(input: &str, format: &Base64Format) -> anyhow::Result<String> {
    let mut reader = utils::get_reader(input).await?;
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(input)?,
        Base64Format::UrlSafeNoPad => BASE64_URL_SAFE_NO_PAD.decode(input)?,
    };
    let decoded_str = String::from_utf8(decoded)?;

    Ok(decoded_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_encode() -> anyhow::Result<()> {
        let input = "-";
        let format = Base64Format::Standard;
        let encoded = process_encode(input, &format).await;
        assert!(encoded.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_decode() -> anyhow::Result<()> {
        let input = "-";
        let format = Base64Format::Standard;
        let decoded = process_decode(input, &format).await;
        assert!(decoded.is_ok());

        Ok(())
    }
}
