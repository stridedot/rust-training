use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use bytes::{Buf as _, BytesMut};

use crate::resp::{
    RespDecode, RespEncode, RespError,
    frame::{self, RespFrame},
    simple_string::SimpleString,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RespMap(HashMap<SimpleString, RespFrame>);

impl RespMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Default for RespMap {
    fn default() -> Self {
        Self::new()
    }
}

impl RespEncode for RespMap {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.push(b'%');
        buf.extend_from_slice(&self.len().to_string().into_bytes());
        buf.extend_from_slice(b"\r\n");

        for (k, v) in &self.0 {
            buf.extend_from_slice(k.encode().as_slice());
            buf.extend_from_slice(v.encode().as_slice());
        }

        buf
    }
}

impl RespDecode for RespMap {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (len, end) = frame::parse_len(buf, "%")?;
        let mut map = HashMap::new();
        buf.advance(end + 2);

        for _ in 0..len {
            let k = SimpleString::decode(buf)?;
            let v = RespFrame::decode(buf)?;
            map.insert(k, v);
        }

        Ok(RespMap(map))
    }
}

impl Deref for RespMap {
    type Target = HashMap<SimpleString, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_encode() -> anyhow::Result<()> {
        let m = RespMap::new();
        let frame = RespFrame::Map(m);
        let buf = frame.encode();
        assert_eq!(buf, b"%0\r\n");

        let mut m = RespMap::new();
        m.insert(SimpleString::new("a"), RespFrame::Integer(1));
        m.insert(
            SimpleString::new("b"),
            RespFrame::SimpleString(SimpleString::new("c")),
        );
        let encoded = m.encode();
        println!("{}", String::from_utf8(encoded)?);

        Ok(())
    }

    #[test]
    fn test_map_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::from("%2\r\n+a\r\n:+1\r\n+b\r\n+c\r\n");
        let frame = RespFrame::decode(&mut buf)?;

        // map 无法确定顺序
        println!("{}", String::from_utf8(frame.encode())?);

        Ok(())
    }
}
