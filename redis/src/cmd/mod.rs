use crate::{
    backend::Backend,
    cmd::{get::Get, hget::HGet, hgetall::HGetAll, hset::HSet, set::Set, unknown::Unknown},
    resp::{RespError, array::RespArray, frame::RespFrame},
};
use anyhow::Result;
use thiserror::Error;

pub mod get;
pub mod hget;
pub mod hgetall;
pub mod hset;
pub mod set;
pub mod unknown;

pub trait CmdExecutor {
    fn execute(&self, backend: &Backend) -> Result<RespFrame>;
}

#[derive(Debug, Error)]
pub enum CmdError {
    #[error("Unknown command: {0}")]
    InvalidCommand(String),
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
    #[error("Resp error: {0}")]
    RespError(#[from] RespError),
}

pub enum Cmd {
    Get(Get),
    HGet(HGet),
    HGetAll(HGetAll),
    HSet(HSet),
    Set(Set),

    Unknown(Unknown),
}

impl CmdExecutor for Cmd {
    fn execute(&self, backend: &Backend) -> Result<RespFrame> {
        match self {
            Cmd::Get(cmd) => cmd.execute(backend),
            Cmd::HGet(cmd) => cmd.execute(backend),
            Cmd::HGetAll(cmd) => cmd.execute(backend),
            Cmd::HSet(cmd) => cmd.execute(backend),
            Cmd::Set(cmd) => cmd.execute(backend),
            Cmd::Unknown(cmd) => cmd.execute(backend),
        }
    }
}

impl TryFrom<RespFrame> for Cmd {
    type Error = CmdError;

    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        match value {
            RespFrame::Array(array) => Self::try_from(array),
            _ => Err(CmdError::InvalidCommand("Must be array".to_string())),
        }
    }
}

impl TryFrom<RespArray> for Cmd {
    type Error = CmdError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match value.first() {
            Some(RespFrame::BulkString(cmd)) => match cmd.as_slice() {
                b"get" => Ok(Get::try_from(value)?.into()),
                b"hget" => Ok(HGet::try_from(value)?.into()),
                b"hgetall" => Ok(HGetAll::try_from(value)?.into()),
                b"hset" => Ok(HSet::try_from(value)?.into()),
                b"set" => Ok(Set::try_from(value)?.into()),
                _ => Ok(Unknown::try_from(value)?.into()),
            },
            _ => Ok(Cmd::Unknown(Unknown)),
        }
    }
}
