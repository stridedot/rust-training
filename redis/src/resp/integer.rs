use bytes::BytesMut;

use crate::resp::{RespDecode, RespEncode, RespError, frame};

impl RespEncode for i64 {
    fn encode(&self) -> Vec<u8> {
        // *self < 0 时，例如 -123，负号已存在，不需要加 "-"
        let sign = if *self < 0 { "" } else { "+" };

        let s = self.to_string();
        let mut buf = Vec::with_capacity(s.len() + sign.len() + 3); // ':' + [+|-] + data + "\r\n"

        buf.push(b':');
        buf.extend_from_slice(sign.as_bytes());
        buf.extend_from_slice(s.as_bytes());
        buf.extend_from_slice(b"\r\n");

        buf
    }
}

impl RespDecode for i64 {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = frame::extract_frame(buf, ":")?;
        let data = buf.split_to(end + 2);
        let s = String::from_utf8(data[1..end].to_vec())?;

        // i64::from_str 支持 + 前缀，s.parse::<i64>() 能正确解析 +42
        Ok(s.parse()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_encode() {
        let i = 123;
        let buf = i.encode();
        assert_eq!(buf, b":+123\r\n");

        let i = -123;
        let buf = i.encode();
        assert_eq!(buf, b":-123\r\n");
    }

    #[test]
    fn test_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::from(":42\r\n");
        let frame = i64::decode(&mut buf)?;

        assert_eq!(frame, 42);

        let mut buf = BytesMut::from(":-42\r\n");
        let frame = i64::decode(&mut buf)?;

        assert_eq!(frame, -42);

        let mut buf = BytesMut::from(":+42\r\n");
        let frame = i64::decode(&mut buf)?;

        assert_eq!(frame, 42);

        Ok(())
    }
}
