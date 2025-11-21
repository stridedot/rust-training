use std::ops::{Deref, DerefMut};

use bytes::{Buf as _, BytesMut};

use crate::resp::{
    RespDecode, RespEncode, RespError,
    frame::{self, RespFrame},
};

#[derive(Clone, Debug, PartialEq)]
pub struct RespSet(Vec<RespFrame>);

impl RespSet {
    pub fn new(v: impl Into<Vec<RespFrame>>) -> Self {
        Self(v.into())
    }
}

impl RespEncode for RespSet {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.push(b'~');
        buf.extend_from_slice(&self.len().to_string().into_bytes());
        buf.extend_from_slice(b"\r\n");

        for v in &self.0 {
            buf.extend_from_slice(v.encode().as_slice());
        }

        buf
    }
}

impl RespDecode for RespSet {
    fn decode(but: &mut BytesMut) -> Result<Self, RespError> {
        let mut set = Vec::new();
        let (len, end) = frame::parse_len(but, "~")?;
        but.advance(end + 2);

        for _ in 0..len {
            let v = RespFrame::decode(but)?;
            set.push(v);
        }

        Ok(RespSet(set))
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::{bulk_string::BulkString, null::RespNull, simple_string::SimpleString};

    use super::*;

    use bytes::BytesMut;

    #[test]
    fn test_encode_set() {
        let set = RespSet::new(vec![
            RespFrame::SimpleString(SimpleString::new("value1")),
            RespFrame::BulkString(BulkString::new("value2")),
            RespFrame::Integer(10),
            RespFrame::Null(RespNull),
            RespFrame::Boolean(true),
            // RespFrame::Double(1.23),
            // RespFrame::Map(RespMap::new()),
            // RespFrame::Set(RespSet::new(vec![
            //     RespFrame::SimpleString(SimpleString::new("value1")),
            //     RespFrame::BulkString(BulkString::new("value2")),
            //     RespFrame::Integer(10),
            //     RespFrame::NullBulkString(RespNullBulkString),
            // ])),
        ]);

        println!("{:?}", String::from_utf8(set.encode()));

        assert_eq!(
            set.encode(),
            b"~5\r\n+value1\r\n$6\r\nvalue2\r\n:+10\r\n_\r\n#t\r\n"
        );
    }

    #[test]
    fn test_decode_set() {
        let mut buf = BytesMut::from("~5\r\n+value1\r\n$6\r\nvalue2\r\n:+10\r\n_\r\n#t\r\n");
        let set = RespSet::decode(&mut buf).unwrap();

        println!("{:?}", set.encode());
    }
}
