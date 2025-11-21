use crate::backend::Backend;
use crate::cmd::{Cmd, CmdError, CmdExecutor};
use crate::resp::array::RespArray;
use crate::resp::frame::RespFrame;
use crate::resp::null::RespNull;
use anyhow::Result;

pub struct HGet {
    key: String,
    field: String,
}

impl CmdExecutor for HGet {
    fn execute(&self, backend: &Backend) -> Result<RespFrame> {
        match backend.hget(&self.key, &self.field) {
            Some(value) => Ok(value),
            None => Ok(RespFrame::Null(RespNull)),
        }
    }
}

impl TryFrom<RespArray> for HGet {
    type Error = CmdError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match (&value[1], &value[2]) {
            (RespFrame::BulkString(key), RespFrame::BulkString(field)) => {
                let key = String::from_utf8(key.to_vec())
                    .map_err(|e| CmdError::InvalidArguments(e.to_string()))?;

                let field = String::from_utf8(field.to_vec())
                    .map_err(|e| CmdError::InvalidArguments(e.to_string()))?;

                Ok(HGet { key, field })
            }
            _ => Err(CmdError::InvalidArguments(
                "Invalid HGET command arguments".to_string(),
            )),
        }
    }
}

impl From<HGet> for Cmd {
    fn from(hget: HGet) -> Self {
        Cmd::HGet(hget)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::{array::RespArray, bulk_string::BulkString, frame::RespFrame};

    #[test]
    fn test_hget_cmd() -> anyhow::Result<()> {
        let backend = Backend::new();
        let array = RespArray(vec![
            RespFrame::BulkString(BulkString(b"hget".to_vec())),
            RespFrame::BulkString(BulkString(b"key".to_vec())),
            RespFrame::BulkString(BulkString(b"field".to_vec())),
        ]);

        let hget_cmd = HGet::try_from(array).unwrap();
        let resp = hget_cmd.execute(&backend)?;

        assert_eq!(resp, RespFrame::Null(RespNull));

        Ok(())
    }
}
