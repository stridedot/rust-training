use crate::{
    backend::Backend,
    cmd::{Cmd, CmdError, CmdExecutor},
    resp::{array::RespArray, frame::RespFrame, null::RespNull},
};
use anyhow::Result;

pub struct Get {
    key: String,
}

impl CmdExecutor for Get {
    fn execute(&self, backend: &Backend) -> Result<RespFrame> {
        match backend.get(&self.key) {
            Some(value) => Ok(value),
            None => Ok(RespFrame::Null(RespNull)),
        }
    }
}

impl TryFrom<RespArray> for Get {
    type Error = CmdError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match &value[1] {
            RespFrame::BulkString(key) => {
                let key = String::from_utf8(key.to_vec())
                    .map_err(|e| CmdError::InvalidArguments(e.to_string()))?;

                Ok(Self { key })
            }
            _ => Err(CmdError::InvalidArguments(
                "Invalid GET command arguments".to_string(),
            )),
        }
    }
}

impl From<Get> for Cmd {
    fn from(get: Get) -> Self {
        Cmd::Get(get)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::{array::RespArray, bulk_string::BulkString, frame::RespFrame};

    #[test]
    fn test_get_cmd() -> anyhow::Result<()> {
        let backend = Backend::new();
        let array = RespArray(vec![
            RespFrame::BulkString(BulkString(b"GET".to_vec())),
            RespFrame::BulkString(BulkString(b"key".to_vec())),
        ]);

        let get_cmd = Get::try_from(array).unwrap();
        let resp = get_cmd.execute(&backend)?;

        assert_eq!(resp, RespFrame::Null(RespNull));

        Ok(())
    }
}
