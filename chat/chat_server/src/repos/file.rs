use std::{path::Path, str::FromStr};

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
        format!("/chat/files/{}", self.hash_to_path())
    }
}

// convert /chat/files/1/779/20c/5872d746712af6e43d2397aa8795df04a6.jpg to ChatFile
impl FromStr for ChatFile {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix("/chat/files/") else {
            return Err(AppError::InvalidFile(format!("invalid file path: {}", s)));
        };

        let parts = s.split('/').collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(AppError::InvalidFile(format!("invalid file path: {}", s)));
        }

        let Ok(workspace_id) = parts[0].parse::<i64>() else {
            return Err(AppError::InvalidFile(format!(
                "invalid workspace_id: {}",
                parts[0]
            )));
        };

        let Some((part3, ext)) = parts[3].split_once('.') else {
            return Err(AppError::InvalidFile(format!(
                "invalid filename or ext: {}",
                parts[3]
            )));
        };

        let hash = format!("{}{}{}", parts[1], parts[2], part3);

        Ok(Self {
            workspace_id,
            ext: ext.to_string(),
            hash: hash.to_string(),
        })
    }
}
