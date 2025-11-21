use bytes::BytesMut;

use crate::resp::{RespDecode, RespEncode, RespError, frame};

impl RespEncode for bool {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.push(b'#');
        buf.push(if *self { b't' } else { b'f' });
        buf.push(b'\r');
        buf.push(b'\n');

        buf
    }
}

impl RespDecode for bool {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        match frame::extract_fixed_frame(buf, "#t\r\n", "Bool") {
            Ok(_) => Ok(true),
            Err(RespError::Incomplete) => Err(RespError::Incomplete),
            Err(_) => match frame::extract_fixed_frame(buf, "#f\r\n", "Bool") {
                Ok(_) => Ok(false),
                Err(e) => Err(e),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_bool_encode() {
        let b = true;
        let encoded = b.encode();

        assert_eq!(encoded, b"#t\r\n");

        let b = false;
        let encoded = b.encode();

        assert_eq!(encoded, b"#f\r\n");
    }

    #[test]
    fn test_bool_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::from("#t\r\n");
        let b = bool::decode(&mut buf)?;

        assert!(b);

        let mut buf = BytesMut::from("#f\r\n");
        let b = bool::decode(&mut buf)?;

        assert!(!b);

        Ok(())
    }
}
