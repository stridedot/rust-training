use std::ops::Deref;

use bytes::{Buf as _, BytesMut};

use crate::resp::{
    RespDecode, RespEncode, RespError,
    frame::{self, RespFrame},
};

#[derive(Clone, Debug, PartialEq)]
pub struct RespArray(pub Vec<RespFrame>);

impl RespArray {
    pub fn new(v: impl Into<Vec<RespFrame>>) -> Self {
        Self(v.into())
    }
}

impl RespEncode for RespArray {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.push(b'*');
        buf.extend_from_slice(&self.len().to_string().into_bytes());
        buf.extend_from_slice(b"\r\n");

        for frame in self.iter() {
            buf.extend_from_slice(&frame.encode());
        }

        buf
    }
}

impl RespDecode for RespArray {
    // "*3\r\n+hello\r\n:+42\r\n$5\r\nworld\r\n";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (len, end) = frame::parse_len(buf, "*")?;
        buf.advance(end + 2);

        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            let frame = RespFrame::decode(buf)?;
            frames.push(frame);
        }

        Ok(RespArray(frames))
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::{bulk_string::BulkString, simple_string::SimpleString};

    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_encode() {
        let array = RespArray::new(vec![
            RespFrame::SimpleString(SimpleString::new("hello")),
            RespFrame::Integer(42),
            RespFrame::BulkString(BulkString::new(b"world")),
        ]);
        let encoded = array.encode();
        println!("{:?}", encoded);

        assert_eq!(encoded, b"*3\r\n+hello\r\n:+42\r\n$5\r\nworld\r\n");
    }

    #[test]
    fn test_decode() -> anyhow::Result<()> {
        let encoded = "*3\r\n+hello\r\n:+42\r\n$5\r\nworld\r\n";
        let mut buf = BytesMut::from(encoded);
        let frame = RespArray::decode(&mut buf)?;

        assert_eq!(frame.len(), 3);

        if let Some(RespFrame::SimpleString(s)) = frame.first() {
            assert_eq!(s.as_str(), "hello");
        } else {
            panic!("Expected SimpleString");
        }

        if let Some(RespFrame::Integer(i)) = frame.get(1) {
            assert_eq!(*i, 42);
        } else {
            panic!("Expected Integer");
        }

        if let Some(RespFrame::BulkString(s)) = frame.get(2) {
            assert_eq!(s.as_slice(), b"world");
        } else {
            panic!("Expected BulkString");
        }

        Ok(())
    }
}
