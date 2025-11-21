use crate::{
    backend::Backend,
    cmd::{Cmd, CmdError, CmdExecutor},
    resp::{array::RespArray, frame::RespFrame},
};
use anyhow::Result;

pub struct HSet {
    key: String,
    field: String,
    value: RespFrame,
}

impl CmdExecutor for HSet {
    fn execute(&self, backend: &Backend) -> Result<RespFrame> {
        backend.hset(self.key.clone(), self.field.clone(), self.value.clone())?;
        Ok(RespFrame::Integer(1))
    }
}

impl TryFrom<RespArray> for HSet {
    type Error = CmdError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match (&value[1], &value[2], &value[3]) {
            (
                RespFrame::BulkString(key),
                RespFrame::BulkString(field),
                RespFrame::BulkString(value),
            ) => {
                let key = String::from_utf8(key.to_vec())
                    .map_err(|e| CmdError::InvalidArguments(e.to_string()))?;

                let field = String::from_utf8(field.to_vec())
                    .map_err(|e| CmdError::InvalidArguments(e.to_string()))?;

                let value = RespFrame::BulkString(value.clone());

                Ok(HSet { key, field, value })
            }
            _ => Err(CmdError::InvalidArguments(
                "Invalid HSET command arguments".to_string(),
            )),
        }
    }
}

impl From<HSet> for Cmd {
    fn from(hset: HSet) -> Self {
        Cmd::HSet(hset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::{array::RespArray, bulk_string::BulkString, frame::RespFrame};

    #[test]
    fn test_hset_cmd() -> anyhow::Result<()> {
        let backend = Backend::new();
        let array = RespArray(vec![
            RespFrame::BulkString(BulkString(b"hset".to_vec())),
            RespFrame::BulkString(BulkString(b"key".to_vec())),
            RespFrame::BulkString(BulkString(b"field".to_vec())),
            RespFrame::BulkString(BulkString(b"value".to_vec())),
        ]);

        let hset_cmd = HSet::try_from(array).unwrap();
        let resp = hset_cmd.execute(&backend)?;

        assert_eq!(resp, RespFrame::Integer(1));

        Ok(())
    }
}
