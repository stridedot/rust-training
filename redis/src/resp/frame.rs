use bytes::{Buf as _, BytesMut};

use crate::resp::{
    RespDecode, RespEncode, RespError, array::RespArray, bulk_string::BulkString, map::RespMap,
    null::RespNull, set::RespSet, simple_error::SimpleError, simple_string::SimpleString,
};

/*
  - serialize/deserialize Frame
    - simple string:        +OK\r\n                 // 短文本消息
    - error:                -Error message\r\n      // 短错误消息
    - integer:              :[<+|->]<value>\r\n
    - bulk string:          $<length>\r\n<data>\r\n // 任意二进制数据，可以传输图片、文件等
    - # null bulk string:     $-1\r\n                 // string 类型的 null
    - array:                *<number-of-elements>\r\n<element-1>... <element-n>
        -ele:                   *2\r\n$3\r\nget\r\n$5\r\nhello\r\n
    - # null array:           *-1\r\n                 // array 类型的 null
    - null:                 "_\r\n"
    - boolean:              #<t|f>\r\n
    - double:               ,[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n
    - map:                  %<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>
    - set:                  ~<number-of-elements>\r\n<element-1>...<element-n>
*/
#[derive(Clone, Debug, PartialEq)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    Array(RespArray),
    Null(RespNull),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}

impl RespEncode for RespFrame {
    fn encode(&self) -> Vec<u8> {
        match self {
            RespFrame::SimpleString(s) => s.encode(),
            RespFrame::Error(e) => e.encode(),
            RespFrame::Integer(i) => i.encode(),
            RespFrame::BulkString(b) => b.encode(),
            RespFrame::Array(a) => a.encode(),
            RespFrame::Null(n) => n.encode(),
            RespFrame::Boolean(b) => b.encode(),
            RespFrame::Double(d) => d.encode(),
            RespFrame::Map(m) => m.encode(),
            RespFrame::Set(s) => s.encode(),
        }
    }
}

impl RespDecode for RespFrame {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => Ok(RespFrame::SimpleString(SimpleString::decode(buf)?)),
            Some(b'-') => Ok(RespFrame::Error(SimpleError::decode(buf)?)),
            Some(b':') => Ok(RespFrame::Integer(i64::decode(buf)?)),
            Some(b'$') => Ok(RespFrame::BulkString(BulkString::decode(buf)?)),
            Some(b'*') => Ok(RespFrame::Array(RespArray::decode(buf)?)),
            Some(b'_') => Ok(RespFrame::Null(RespNull::decode(buf)?)),
            Some(b'#') => Ok(RespFrame::Boolean(bool::decode(buf)?)),
            Some(b',') => Ok(RespFrame::Double(f64::decode(buf)?)),
            Some(b'%') => Ok(RespFrame::Map(RespMap::decode(buf)?)),
            Some(b'~') => Ok(RespFrame::Set(RespSet::decode(buf)?)),
            _ => Err(RespError::Incomplete),
        }
    }
}

pub fn extract_frame(buf: &[u8], prefix: &str) -> Result<usize, RespError> {
    if buf.len() < 3 {
        return Err(RespError::Incomplete);
    }

    let len = prefix.len();
    if !buf.starts_with(prefix.as_bytes()) {
        return Err(RespError::InvalidFrame(format!(
            "Invalid frame prefix, expect {}, but got {:?}",
            prefix,
            &buf[..len]
        )));
    }

    // 获取第一帧的结束位置，即第一个 \r\n 的位置
    let end = find_crlf(buf, len).ok_or(RespError::Incomplete)?;

    Ok(end)
}

// 查找 buf 中第 nth 个 CRLF
fn find_crlf(buf: &[u8], nth: usize) -> Option<usize> {
    let mut count = 0;

    for i in 1..buf.len() - 1 {
        // buf 是 &[u8] 类型，buf[i] 是数字，b'\r' == 13, b'\n' == 10
        if buf[i] != b'\r' || buf[i + 1] != b'\n' {
            continue;
        }
        count += 1;
        if count == nth {
            return Some(i);
        }
    }

    None
}

// 解析长度字符串
pub fn parse_len(buf: &[u8], prefix: &str) -> Result<(usize, usize), RespError> {
    let end = extract_frame(buf, prefix)?;

    // 解析长度字符串
    let len_str = buf[prefix.len()..end].to_vec();
    let len_val = String::from_utf8(len_str)?.parse::<usize>()?;

    Ok((len_val, end))
}

pub fn extract_fixed_frame(buf: &mut BytesMut, prefix: &str, name: &str) -> Result<(), RespError> {
    if buf.len() < prefix.len() {
        return Err(RespError::Incomplete);
    }

    if !buf.starts_with(prefix.as_bytes()) {
        return Err(RespError::InvalidFrame(format!(
            "Invalid {} frame prefix, expect {}, but got {:?}",
            name,
            prefix,
            &buf[..prefix.len()]
        )));
    }

    // 移动内部游标到 \r\n 的结束位置
    buf.advance(prefix.len());

    Ok(())
}
