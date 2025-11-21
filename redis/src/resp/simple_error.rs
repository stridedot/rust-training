use std::ops::Deref;

use bytes::BytesMut;

use crate::resp::{RespDecode, RespEncode, RespError, frame};

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleError(String);

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl RespEncode for SimpleError {
    fn encode(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        let mut buf = Vec::with_capacity(bytes.len() + 3); // '-' + data + "\r\n"

        buf.push(b'-');
        buf.extend_from_slice(bytes);
        buf.extend_from_slice(b"\r\n");

        buf
    }
}

impl RespDecode for SimpleError {
    fn decode(but: &mut BytesMut) -> Result<Self, RespError> {
        let end = frame::extract_frame(but, "-")?;
        let data = but.split_to(end + 2);
        let s = String::from_utf8(data[1..end].to_vec())?;

        Ok(Self::new(s))
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn test_simple_error_encode() {
        let err = SimpleError::new("error");
        let encoded = err.encode();

        assert_eq!(encoded, b"-error\r\n");
    }

    #[test]
    fn test_simple_error_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::from("-error\r\n");
        let err = SimpleError::decode(&mut buf)?;

        assert_eq!(err.as_str(), "error");

        Ok(())
    }
}
