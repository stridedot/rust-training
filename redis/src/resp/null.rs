use bytes::BytesMut;

use crate::resp::{RespDecode, RespEncode, RespError, frame};

#[derive(Clone, Debug, PartialEq)]
pub struct RespNull;

impl RespEncode for RespNull {
    fn encode(&self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

impl RespDecode for RespNull {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        frame::extract_fixed_frame(buf, "_\r\n", "Null")?;

        Ok(RespNull)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_null_encode() {
        let null = RespNull;
        let encoded = null.encode();
        assert_eq!(encoded, b"_\r\n");
    }

    #[test]
    fn test_null_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::from("_\r\n");
        let null = RespNull::decode(&mut buf)?;
        println!("{:?}", null);

        Ok(())
    }
}
