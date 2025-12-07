use std::path::Path;

use chat_core::error::AppError;
use sha1::{Digest as _, Sha1};

#[derive(Clone, Debug)]
pub struct ChatFile {
    pub workspace_id: i64,
    pub ext: String, // extract from filename
    pub hash: String,
}

impl ChatFile {
    pub fn try_new(workspace_id: i64, filename: &str, data: &[u8]) -> Result<Self, AppError> {
        let ext = match filename.split('.').next_back() {
            Some(ext) => ext,
            None => return Err(AppError::InvalidFile("invalid file extension".to_string())),
        };

        Ok(Self {
            workspace_id,
            ext: ext.to_string(),
            hash: hex::encode(Sha1::digest(data)),
        })
    }

    pub fn path(&self, file_dir: &Path) -> std::path::PathBuf {
        file_dir.join(self.hash_to_path())
    }

    pub fn hash_to_path(&self) -> String {
        let (part1, part2) = self.hash.split_at(3);
        let (part2, part3) = part2.split_at(3);
        format!(
            "{}/{}/{}/{}.{}",
            self.workspace_id, part1, part2, part3, self.ext
        )
    }

    pub fn url(&self) -> String {
        format!("chat/files/{}", self.hash_to_path())
    }
}
