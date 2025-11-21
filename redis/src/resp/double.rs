use bytes::BytesMut;

use crate::resp::{RespDecode, RespEncode, RespError, frame};

impl RespEncode for f64 {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(b',');

        // 处理特殊值
        if self.is_nan() {
            buf.extend_from_slice(b"nan\r\n");
            return buf;
        }
        if self.is_infinite() {
            if self.is_sign_positive() {
                buf.extend_from_slice(b"inf\r\n");
            } else {
                buf.extend_from_slice(b"-inf\r\n");
            }
            return buf;
        }

        // 选择最佳表示形式
        let abs_val = self.abs();
        let s = if abs_val == 0.0 {
            "+0.0".to_string()
        } else if abs_val >= 1e12 || (abs_val > 0.0 && abs_val < 1e-12) {
            // 使用 ryu crate 进行精确的浮点数格式化
            // 或者使用更简单的方法：
            let sci_str = format!("{self:+e}");
            // 手动调整格式
            sci_str
                .replace("e+0", "e+") // 去除指数中的前导零
                .replace("e-0", "e-") // 去除指数中的前导零
                .replace('e', "e+") // 确保正指数有 + 号
                .replace("e+-", "e-") // 修复负指数
        } else if self.fract() == 0.0 {
            // 整数值，添加 .0
            format!("{self:+}.0")
        } else {
            // 常规小数表示，确保正数有 + 号
            format!("{self:+}")
        };

        // 直接写入字节
        buf.extend_from_slice(s.as_bytes());
        buf.extend_from_slice(b"\r\n");

        buf
    }
}

impl RespDecode for f64 {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = frame::extract_frame(buf, ",")?;
        let data = buf.split_to(end + 2);
        let s = String::from_utf8(data[1..end].to_vec())?;

        Ok(s.parse::<f64>()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_encode_decode() -> anyhow::Result<()> {
        let f = 123.456;
        let encoded = f.encode();
        let decoded = f64::decode(&mut BytesMut::from(&encoded[..]))?;

        assert_eq!(f, decoded);

        Ok(())
    }
}
