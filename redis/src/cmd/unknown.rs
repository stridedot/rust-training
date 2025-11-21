use anyhow::Result;

use crate::{
    backend::Backend,
    cmd::{Cmd, CmdError, CmdExecutor},
    resp::{array::RespArray, frame::RespFrame},
};

pub struct Unknown;

impl CmdExecutor for Unknown {
    fn execute(&self, _backend: &Backend) -> Result<RespFrame> {
        Ok(RespFrame::Array(RespArray::new(vec![])))
    }
}

impl TryFrom<RespArray> for Unknown {
    type Error = CmdError;

    fn try_from(_value: RespArray) -> Result<Self, Self::Error> {
        Ok(Unknown)
    }
}

impl From<Unknown> for Cmd {
    fn from(_: Unknown) -> Self {
        Cmd::Unknown(Unknown)
    }
}
