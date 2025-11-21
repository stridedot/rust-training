use std::ops::Deref;

use bytes::{Buf as _, BytesMut};

use crate::resp::{RespDecode, RespEncode, RespError, frame};

#[derive(Clone, Debug, PartialEq)]
pub struct BulkString(pub Vec<u8>);

impl BulkString {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self(data.into())
    }
}

impl RespEncode for BulkString {
    fn encode(&self) -> Vec<u8> {
        let len = self.len();
        let len_str = len.to_string();
        let mut buf = Vec::with_capacity(1 + len_str.len() + 2 + len + 2);

        buf.push(b'$');
        buf.extend_from_slice(&len_str.into_bytes());
        buf.extend_from_slice(b"\r\n");
        buf.extend_from_slice(self);
        buf.extend_from_slice(b"\r\n");

        buf
    }
}

impl RespDecode for BulkString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        // $<length>\r\n<data>\r\n
        // len_val 为 <data> 的长度
        let (len_val, end) = frame::parse_len(buf, "$")?;

        // remained: <data>\r\n
        let remained = &buf[end + 2..];
        if remained.len() < len_val + 2 {
            return Err(RespError::Incomplete);
        }

        // 移动内部游标到 <data>\r\n 的结束位置
        buf.advance(end + 2);
        // 提取 <data>，buf = \r\n
        let data = buf.split_to(len_val + 2);

        Ok(BulkString::new(&data[..len_val]))
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_bulk_string_encode() {
        let bulk_string = BulkString(b"hello".to_vec());
        let encoded = bulk_string.encode();

        assert_eq!(encoded, b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_bulk_string_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::from("$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n");
        let bulk_string = BulkString::decode(&mut buf).unwrap();

        // 检查 buf 是否为空
        assert_eq!(bulk_string.as_slice(), b"set");

        Ok(())
    }
}
