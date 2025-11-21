use crate::backend::Backend;
use crate::cmd::{Cmd, CmdError, CmdExecutor};
use crate::resp::array::RespArray;
use crate::resp::frame::RespFrame;
use crate::resp::map::RespMap;
use crate::resp::null::RespNull;
use crate::resp::simple_string::SimpleString;
use anyhow::Result;

pub struct HGetAll {
    key: String,
}

impl CmdExecutor for HGetAll {
    fn execute(&self, backend: &Backend) -> Result<RespFrame> {
        match backend.hgetall(&self.key) {
            Some(values) => {
                let mut m = RespMap::new();
                for entry in values.iter() {
                    m.insert(
                        SimpleString::new(entry.key().clone()),
                        entry.value().clone(),
                    );
                }
                Ok(RespFrame::Map(m))
            }
            None => Ok(RespFrame::Null(RespNull)),
        }
    }
}

impl TryFrom<RespArray> for HGetAll {
    type Error = CmdError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match &value[1] {
            RespFrame::BulkString(key) => {
                let key = String::from_utf8(key.to_vec())
                    .map_err(|e| CmdError::InvalidArguments(e.to_string()))?;

                Ok(HGetAll { key })
            }
            _ => Err(CmdError::InvalidArguments(
                "Invalid HGETALL command arguments".to_string(),
            )),
        }
    }
}

impl From<HGetAll> for Cmd {
    fn from(hgetall: HGetAll) -> Self {
        Cmd::HGetAll(hgetall)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::{array::RespArray, bulk_string::BulkString, frame::RespFrame};

    #[test]
    fn test_hgetall_cmd() -> anyhow::Result<()> {
        let backend = Backend::new();
        let array = RespArray(vec![
            RespFrame::BulkString(BulkString(b"hgetall".to_vec())),
            RespFrame::BulkString(BulkString(b"key".to_vec())),
        ]);

        let hgetall_cmd = HGetAll::try_from(array).unwrap();
        let resp = hgetall_cmd.execute(&backend)?;

        assert_eq!(resp, RespFrame::Null(RespNull));

        Ok(())
    }
}
