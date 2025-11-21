pub mod array;
pub mod bool;
pub mod bulk_string;
pub mod double;
pub mod frame;
pub mod integer;
pub mod map;
pub mod null;
pub mod set;
pub mod simple_error;
pub mod simple_string;

use bytes::BytesMut;

pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length: {0}")]
    InvalidLength(usize),

    // less than a full frame means incomplete
    #[error("Frame is incomplete")]
    Incomplete,

    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Parse utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}
