use std::ops::Deref;

use bytes::BytesMut;

use crate::resp::{RespDecode, RespEncode, RespError, frame};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SimpleString(pub String);

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl RespEncode for SimpleString {
    fn encode(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        let mut buf = Vec::with_capacity(bytes.len() + 3); // '+' + data + "\r\n"

        buf.push(b'+');
        buf.extend_from_slice(bytes);
        buf.extend_from_slice(b"\r\n");

        buf
    }
}

impl RespDecode for SimpleString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = frame::extract_frame(buf, "+")?;
        let data = buf.split_to(end + 2);
        let s = String::from_utf8(data[1..end].to_vec())?;

        Ok(Self::new(s))
    }
}

impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::RespEncode;
    use bytes::BytesMut;

    #[test]
    fn test_simple_string_encode() {
        let s = SimpleString::new("OK");
        let buf = s.encode();

        assert_eq!(buf, b"+OK\r\n");
    }

    #[test]
    fn test_simple_string_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::from("+OK\r\n");
        let s = SimpleString::decode(&mut buf)?;

        assert_eq!(s.as_str(), "OK");

        Ok(())
    }
}
