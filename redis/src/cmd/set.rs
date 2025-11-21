use crate::{
    backend::Backend,
    cmd::{Cmd, CmdError, CmdExecutor},
    resp::{array::RespArray, frame::RespFrame, simple_string::SimpleString},
};
use anyhow::Result;

pub struct Set {
    key: String,
    value: RespFrame,
}

impl CmdExecutor for Set {
    fn execute(&self, backend: &Backend) -> Result<RespFrame> {
        backend.set(self.key.clone(), self.value.clone())?;
        Ok(RespFrame::SimpleString(SimpleString::new("OK")))
    }
}

impl TryFrom<RespArray> for Set {
    type Error = CmdError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match (&value[1], &value[2]) {
            (RespFrame::BulkString(key), RespFrame::BulkString(value)) => {
                let key = String::from_utf8(key.to_vec())
                    .map_err(|e| CmdError::InvalidArguments(e.to_string()))?;

                let value = RespFrame::BulkString(value.clone());

                Ok(Set { key, value })
            }
            _ => Err(CmdError::InvalidArguments(
                "Invalid SET command arguments".to_string(),
            )),
        }
    }
}

impl From<Set> for Cmd {
    fn from(set: Set) -> Self {
        Cmd::Set(set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::array::RespArray;
    use crate::resp::bulk_string::BulkString;

    #[test]
    fn test_set_cmd_from_array() -> anyhow::Result<()> {
        let backend = Backend::new();
        let array = RespArray(vec![
            RespFrame::BulkString(BulkString(b"SET".to_vec())),
            RespFrame::BulkString(BulkString(b"key".to_vec())),
            RespFrame::BulkString(BulkString(b"value".to_vec())),
        ]);

        let set_cmd = Set::try_from(array).unwrap();
        let resp = set_cmd.execute(&backend)?;

        assert_eq!(resp, RespFrame::SimpleString(SimpleString::new("OK")));

        Ok(())
    }
}
